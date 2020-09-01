use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use rand::{thread_rng, Rng};

struct AnimationFrame {
    index: i32,
    time: f32,
}

struct Animation {
    frames: Vec<AnimationFrame>,
    current_frame: i32,
}

struct Animations {
    animations: Vec<Animation>,
    current_animation: i32,
}

struct CloudTimer(Timer);

fn cloud_spawn_system(mut commands: Commands, time: Res<Time>, mut cloud_timer: ResMut<CloudTimer>, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>)
{
    let mut rng = thread_rng();
    let cloud_texture = match rng.gen_bool(0.5) {
        true => 
            asset_server.load("assets/cloud_1.png").unwrap()
        ,
        false => 
            asset_server.load("assets/cloud_2.png").unwrap()
    };

    cloud_timer.0.tick(time.delta_seconds);
    if cloud_timer.0.finished {
        commands.spawn(SpriteComponents {
            material: materials.add(cloud_texture.into()),
            scale: Scale(rng.gen_range(6.0, 30.0)),
            translation: Translation(Vec3::new(1920.0*0.5+30.0*43.0, rng.gen_range(-1280.0*0.5, 1280.0*0.5), 2.0)),
            .. Default::default()
        })
        .with(Velocity(Vec2::new(rng.gen_range(-700.0, -400.0), rng.gen_range(-10.0, 10.0))));
    }
}

struct MountainTimer(Timer);

fn mountain_spawn_system(mut commands: Commands, time: Res<Time>, mut mountain_timer: ResMut<MountainTimer>, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>)
{
    let mut rng = thread_rng();
    let mountain_texture = match rng.gen_bool(0.5) {
        true => 
            asset_server.load("assets/mountain.png").unwrap()
        ,
        false => 
            asset_server.load("assets/mountain.png").unwrap()
    };

    mountain_timer.0.tick(time.delta_seconds);
    if mountain_timer.0.finished {
        commands.spawn(SpriteComponents {
            scale: Scale(3.0),
            material: materials.add(ColorMaterial::modulated_texture(mountain_texture, Color::rgb(0.36,0.36,0.36))),
            translation: Translation(Vec3::new(1920.0*0.5+30.0*43.0, -1280.0*0.5, 0.2)),
            .. Default::default()
        })
        .with(OffsceenDeletion)
        .with(Velocity(Vec2::new(-200.0, 0.0)));
        commands.spawn(SpriteComponents {
            scale: Scale(3.0),
            translation: Translation(Vec3::new(1920.0*0.5+30.0*43.0, -1280.0*0.5-100.0, 0.3)),
            material: materials.add(ColorMaterial::modulated_texture(mountain_texture, Color::rgb(0.26,0.26,0.26))),
            .. Default::default()
        })
        .with(OffsceenDeletion)
        .with(Velocity(Vec2::new(-400.0, 0.0)));
    }
}

struct SpawnTimer {
    timer: Timer,
    // center pos of pipes, in precentage
    last_pos: f32,
}

struct OffsceenDeletion;

struct PipeSpawnSettings {
    min_time: f32,
    max_time: f32,
    speed: f32,
    // distance from upper and lower pipe, in precentage
    min_pipe_distance: f32,
    max_pipe_distance: f32,
    max_center_delta: f32,
}

#[derive(PartialEq)]
enum Collider {
    Solid,
    ScoreGiver,
}

struct GameData {
    game_state: GameState,
    score: i32,
}

#[derive(std::cmp::PartialEq)]
enum GameState {
    Menu,
    Playing,
    Dead,
}
struct StartScreen;
struct EndScreen;
struct Player;
struct Pipe;
struct Velocity(Vec2);
struct Gravity(f32);
struct JumpHeight(f32);
struct AffectedByGravity;
struct VelocityRotator {
    angle_up: f32,
    angle_down: f32,
    // The amount of velocity to reach the min or max angle
    velocity_max: f32,
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(animate_sprite_system.system())
        .add_system(animate_system.system())
        .add_system(player_input.system())
        .add_system(player_bounds_system.system())
        .add_system(handle_gamestate_system.system())
        .add_system(player_collision_system.system())
        .add_system(velocity_system.system())
        .add_system(gravity_system.system())
        .add_system(velocity_rotator_system.system())
        .add_system(velocity_animator_system.system())
        .add_system(mountain_spawn_system.system())
        .add_system(spawn_pipe_system.system())
        //.add_system(pipe_move_system.system())
        .add_system(offscreen_remove_system.system())
        .add_system(cloud_spawn_system.system())
        .add_resource(ClearColor(Color::rgb(0.34, 0.75, 0.79)))
        .add_resource(JumpHeight(23.0*40.0))
        .add_resource(Gravity(45.0*40.0))
        .add_resource(CloudTimer(Timer::from_seconds(1.0, true)))
        .add_resource(MountainTimer(Timer::from_seconds(3.0, true)))
        .add_resource(GameData {
            game_state: GameState::Menu,
            score: 0,
        })
        .add_resource(SpawnTimer {
            timer: Timer::from_seconds(2.0, true),
            last_pos: 0.5,
        })
        .add_resource(PipeSpawnSettings {
            min_time: 0.9,
            max_time: 1.2,
            speed: -700.0,
            min_pipe_distance: 300.0,
            max_pipe_distance: 600.0,
            max_center_delta: 0.4,
        })
        .run();
}

fn offscreen_remove_system(
    mut commands: Commands,
    mut worlds: Query<&mut World>,
    mut pipe_query: Query<(Entity, &mut Translation, &OffsceenDeletion)>,
) {
    let padding = 300.0;
    for (entity, translation, _pipe) in &mut pipe_query.iter() {
        // Left side of screen
        if translation.0.x() < -1920.0 * 0.5 - padding {
            for world in &mut worlds.iter() {
                if !world.contains(entity) {
                    commands.despawn(entity);
                }
            }
        }
    }
}

fn velocity_rotator_system(
    velocity: Mut<Velocity>,
    mut rotation: Mut<Rotation>,
    velocity_rotator: Mut<VelocityRotator>,
) {
    //let quat = Quat::from_rotation_z(velocity_rotator.).lerp();
    let mut procentage = velocity.0.y() / velocity_rotator.velocity_max;
    procentage = procentage.max(-1.0);
    procentage = procentage.min(1.0);
    // convert from -1 -> 1 to: 0 -> 1
    procentage = (procentage + 1.0) * 0.5;

    // Lerp from lower angle to upper angle
    let rad_angle =
        (1.0 - procentage) * velocity_rotator.angle_down + procentage * velocity_rotator.angle_up;

    rotation.0 = Quat::from_rotation_z(rad_angle);
}

fn handle_gamestate_system(
    mut game_data: ResMut<GameData>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut Translation, &mut Velocity)>,
    mut end_screen_query: Query<(&EndScreen, &mut Draw)>,
    mut start_screen_query: Query<(&StartScreen, &mut Draw)>,
) {
    match game_data.game_state {
        GameState::Menu => {
            if keyboard_input.just_pressed(KeyCode::Space) {
                game_data.game_state = GameState::Playing;
                for (_ss, mut draw) in &mut start_screen_query.iter() {
                    draw.is_visible = false;
                }
            }
        }
        GameState::Playing => {}
        GameState::Dead => {
            if keyboard_input.just_pressed(KeyCode::Space) {
                game_data.game_state = GameState::Playing;
                for (_p, mut translation, mut velocity) in &mut player_query.iter() {
                    translation.0 = Vec3::new(0.0, 0.0, 100.0);
                    velocity.0.set_y(0.0);
                }
                for (_es, mut draw) in &mut end_screen_query.iter() {
                    draw.is_visible = false;
                }
            }
        }
    }
}

fn player_input(
    game_data: Res<GameData>,
    jump_height: Res<JumpHeight>,
    keyboard_input: Res<Input<KeyCode>>,
    _player: Mut<Player>,
    mut translation: Mut<Translation>,
    mut velocity: Mut<Velocity>,
) {
    match game_data.game_state {
        GameState::Menu => {
            handle_stay_in_screen(jump_height, velocity, translation);
        }
        GameState::Playing => {
            handle_jump(keyboard_input, jump_height, velocity);
        }
        GameState::Dead => {}
    }
}

// Auto jump until input is given
fn handle_stay_in_screen(
    jump_height: Res<JumpHeight>,
    mut velocity: Mut<Velocity>,
    translation: Mut<Translation>,
) {
    if translation.0.y() < 0.0 {
        velocity.0.set_y(jump_height.0);
    }
}

fn velocity_animator_system(mut query: Query<(&mut Animations, &Velocity)>) {
    for (mut animations, velocity) in &mut query.iter() {
        if velocity.0.y() > 0.0 {
            animations.current_animation = 0;
        } else {
            animations.current_animation = 1;
        }
    }
}

fn handle_jump(
    keyboard_input: Res<Input<KeyCode>>,
    jump_height: Res<JumpHeight>,
    mut velocity: Mut<Velocity>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        velocity.0.set_y(jump_height.0);
    }
}

fn gravity_system(
    gravity: Res<Gravity>,
    time: Res<Time>,
    _affected_by_gravity: &AffectedByGravity,
    mut velocity: Mut<Velocity>,
) {
    *velocity.0.y_mut() -= gravity.0 * time.delta_seconds;
}

fn velocity_system(
    time: Res<Time>,
    mut position: Mut<Translation>,
    velocity: Mut<Velocity>,
) {
    let y = position.0.y();
    let x = position.0.x();
    let delta = time.delta_seconds;
    position.0.set_y(y + velocity.0.y() * delta);
    position.0.set_x(x + velocity.0.x() * delta);
}

fn animate_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &mut Animations,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle, mut animations) in &mut query.iter() {
        if timer.finished {
            let current_animation_index = animations.current_animation;
            match animations
                .animations
                .get_mut(current_animation_index as usize)
            {
                Some(animation) => {
                    animation.current_frame += 1;
                    if animation.current_frame as usize >= animation.frames.len() {
                        animation.current_frame = 0;
                    }
                    let frame_data = animation
                        .frames
                        .get(animation.current_frame as usize)
                        .unwrap();
                    timer.duration = frame_data.time;
                    if let Some(frame) = animation.frames.get(animation.current_frame as usize) {
                        sprite.index = frame.index as u32;
                    }
                }
                None => {}
            }
        }
    }
}

fn animate_sprite_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (timer, mut sprite, texture_atlas_handle) in &mut query.iter() {
        if timer.finished {
            let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

// fn pipe_move_system(
//     time: Res<Time>,
//     pipe_settings: Res<PipeSpawnSettings>,
//     _pipe: Mut<Pipe>,
//     mut translation: Mut<Translation>,
// ) {
//     let x_pos = translation.0.x_mut();
//     *x_pos += time.delta_seconds * pipe_settings.speed;
// }

fn spawn_pipe_system(
    mut commands: Commands,
    pipe_settings: Res<PipeSpawnSettings>,
    game_data: Res<GameData>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    if game_data.game_state != GameState::Playing {
        return;
    }

    spawn_timer.timer.tick(time.delta_seconds);
    if !spawn_timer.timer.finished {
        return;
    }

    let mut rng = thread_rng();
    spawn_timer.timer.duration = rng.gen_range(pipe_settings.min_time, pipe_settings.max_time);

    let mut new_center_pos = spawn_timer.last_pos
        - rng.gen_range(
            -pipe_settings.max_center_delta,
            pipe_settings.max_center_delta,
        );

    // sorry for the hardcoded values
    // This is the extent from the center in Y, a pipe can go maximum, until it flies in the air
    let clamp_range = (1280.0 - (6.0 * 128.0)) / 1280.0;

    // Clamp func seem to be nightly only for now
    new_center_pos = new_center_pos.min(clamp_range);
    new_center_pos = new_center_pos.max(-clamp_range);
    spawn_timer.last_pos = new_center_pos;
    // to world units
    new_center_pos *= 1280.0 * 0.5;

    let pipe_texture_handle = asset_server
        .load_sync(&mut textures, "assets/pipe.png")
        .unwrap();

    let pipe_offset_y = (6.0 * 128.0) * 0.5;
    let pipe_offset_x = (6.0 * 32.0) * 0.5;
    let mut pipe_delta = rng.gen_range(
        pipe_settings.min_pipe_distance,
        pipe_settings.max_pipe_distance,
    );
    // half the size because both pipes will be offseted in opposide direction
    pipe_delta *= 0.5;
    let x_pos = 1920.0 * 0.5 + pipe_offset_x;

    // lower pipe
    commands
        .spawn(SpriteComponents {
            material: materials.add(pipe_texture_handle.into()),
            scale: Scale(6.0),
            draw: Draw {
                is_transparent: true,
                is_visible: true,
                render_commands: Vec::new(),
            },
            translation: Translation::new(
                x_pos,
                -pipe_offset_y + new_center_pos - pipe_delta,
                3.0,
            ),
            ..Default::default()
        })
        .with(Velocity(Vec2::new(pipe_settings.speed,0.0)))
        .with(Pipe)
        .with(OffsceenDeletion)
        .with(Collider::Solid);
    // higher pipe
    commands
        .spawn(SpriteComponents {
            material: materials.add(pipe_texture_handle.into()),
            scale: Scale(6.0),
            draw: Draw {
                is_transparent: true,
                is_visible: true,
                render_commands: Vec::new(),
            },
            translation: Translation::new(
                x_pos,
                pipe_offset_y + new_center_pos + pipe_delta,
                3.0,
            ),
            rotation: Rotation::from_rotation_z(std::f32::consts::PI),
            ..Default::default()
        })
        .with(Pipe)
        .with(OffsceenDeletion)
        .with(Velocity(Vec2::new(pipe_settings.speed,0.0)))
        .with(Collider::Solid);

    // score collider offseted by half player size
    let score_offset = Vec3::new(32.0*6.0*0.5, 0.0, 0.0); 
    commands.spawn((
        Translation(score_offset + Vec3::new(x_pos, 0.0, 0.0)),
        Collider::ScoreGiver,
        Velocity(Vec2::new(pipe_settings.speed, 0.0)),
    ))
    .with(OffsceenDeletion);
}

fn player_bounds_system(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    mut player_query: Query<(&Player, &mut Translation, &mut Velocity)>,
    mut pipe_query: Query<(&Pipe, &Translation, &Collider, &Sprite, Entity)>,
    mut score_collider_query: Query<(&Translation, &Collider, Entity)>,
    mut end_screen_query: Query<(&EndScreen, &mut Draw)>,
) {
    let half_screen_size = 1280.0 * 0.5;
    let player_size = 32.0 * 6.0;
    for (_p, mut translation, mut velocity) in &mut player_query.iter() {
        // bounce against ceiling
        if translation.0.y() > half_screen_size - player_size {
            velocity.0.set_y(-3.0);
            translation.0.set_y(half_screen_size - player_size);
        }
        // death on bottom touch
        if translation.0.y() < -half_screen_size {
            trigger_death(
                &mut commands,
                &mut game_data,
                &mut pipe_query,
                &mut score_collider_query,
                &mut end_screen_query,
            );
        }
    }
}

fn player_collision_system(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    mut worlds: Query<(&mut World)>,
    mut player_query: Query<(&Player, &Translation)>,
    mut pipe_query: Query<(&Pipe, &Translation, &Collider, &Sprite, Entity)>,
    mut score_collider_query: Query<(&Translation, &Collider, Entity)>,
    mut end_screen_query: Query<(&EndScreen, &mut Draw)>,
) {
    // Player size can't be fetched from AtlasTextureSprite, so I'm hard coding it here...
    let mut player_size = 6.0 * 32.0;
    // Make player hitbox half size, to feel more fair
    player_size *= 0.4;
    let player_size_vec = (player_size, player_size);
    for (_player, player_translation) in &mut player_query.iter() {
        for (translation, collider, entity) in &mut score_collider_query.iter() {
            if *collider != Collider::ScoreGiver {
                continue;
            }
            let collision = collide(
                player_translation.0,
                player_size_vec.into(),
                translation.0,
                Vec2::new(10.0, 1280.0),
            );
            if collision.is_some() {
                game_data.score += 1;
                println!("got score!: {}", game_data.score);
                // Remove coin collider, quick simple solution
            for world in &mut worlds.iter() {
                if !world.contains(entity) {
                    commands.despawn(entity);
                }
            }

            }
        }
        // Check for collision
        let mut did_collide = false;
        for (_pipe, pipe_translation, _collider, pipe_sprite, _pipe_entity) in
            &mut pipe_query.iter()
        {
            let collision = collide(
                player_translation.0,
                player_size_vec.into(),
                pipe_translation.0,
                pipe_sprite.size * 6.0,
            );
            if collision.is_some() {
                did_collide = true;
                break;
            }
        }
        if did_collide {
            trigger_death(
                &mut commands,
                &mut game_data,
                &mut pipe_query,
                &mut score_collider_query,
                &mut end_screen_query,
            );
        }
    }
}

fn trigger_death(
    commands: &mut Commands,
    game_data: &mut ResMut<GameData>,
    mut pipe_query: &mut Query<(&Pipe, &Translation, &Collider, &Sprite, Entity)>,
    mut score_query: &mut Query<(&Translation, &Collider, Entity)>,
    mut end_screen_query: &mut Query<(&EndScreen, &mut Draw)>,
) {
    game_data.game_state = GameState::Dead;
    game_data.score = 0;
    // Despawn all pipes
    for (_p, _pt, _c, _ps, pipe_entity) in &mut pipe_query.iter() {
        commands.despawn(pipe_entity);
    }
    // Despawn score colliders
    for (_t, collider, score_entity) in &mut score_query.iter() {
        if *collider == Collider::ScoreGiver {
            commands.despawn(score_entity);
        }
    }
    for (_es, mut draw) in &mut end_screen_query.iter() {
        draw.is_visible = true;
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/bird.png")
        .unwrap();
    asset_server
        .load_sync(&mut textures, "assets/pipe.png")
        .unwrap();
    //materials.add(pipe_texture_handle.into());

    asset_server.watch_for_changes().unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 2, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let start_texture_handle = asset_server.load("assets/SpaceToStart.png").unwrap();
    let game_over_texture_handle = asset_server.load("assets/GameOverText.png").unwrap();
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(start_texture_handle.into()),
            ..Default::default()
        })
        .with(StartScreen)
        .spawn(SpriteComponents {
            material: materials.add(game_over_texture_handle.into()),
            draw: Draw {
                is_transparent: true,
                is_visible: false,
                render_commands: Vec::new(),
            },
            ..Default::default()
        })
        .with(EndScreen)
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            scale: Scale(6.0),
            translation: Translation::new(0.0, 0.0, 100.0),
            draw: Draw {
                is_transparent: true,
                is_visible: true,
                render_commands: Vec::new(),
            },
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))
        .with(Player)
        .with(AffectedByGravity)
        .with(VelocityRotator {
            angle_up: std::f32::consts::PI * 0.5 * 0.7,
            angle_down: -std::f32::consts::PI * 0.5 * 0.5,
            velocity_max: 400.0,
        })
        .with(Velocity(Vec2::zero()))
        .with(Animations {
            animations: vec![
                Animation {
                    current_frame: 0,
                    frames: vec![
                        AnimationFrame {
                            index: 0,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 1,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 2,
                            time: 0.3,
                        },
                        AnimationFrame {
                            index: 1,
                            time: 0.1,
                        },
                    ],
                },
                Animation {
                    current_frame: 0,
                    frames: vec![AnimationFrame {
                        index: 3,
                        time: 0.2,
                    }],
                },
            ],
            current_animation: 0,
        });
}
