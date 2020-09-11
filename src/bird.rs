use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
    sprite::collide_aabb::{collide},
};

use crate::animation;
use crate::gamedata;
use crate::gamestate;
use crate::physics;
use crate::pipes;
use crate::screens;

use animation::*;
use gamedata::*;
use gamestate::*;
use physics::*;
use pipes::*;
use screens::*;

pub struct Player;
pub struct JumpHeight(pub f32);

// data for animating rotation
pub struct VelocityRotator {
    pub angle_up: f32,
    pub angle_down: f32,
    // The amount of velocity to reach the min or max angle
    pub velocity_max: f32,
}

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(player_input.system())
            .add_system(player_bounds_system.system())
            .add_system(player_collision_system.system())
            .add_system(velocity_rotator_system.system())
            .add_system(velocity_animator_system.system());
    }
}

fn player_input(
    game_data: Res<GameData>,
    jump_height: Res<JumpHeight>,
    keyboard_input: Res<Input<KeyCode>>,
    _player: Mut<Player>,
    translation: Mut<Translation>,
    velocity: Mut<Velocity>,
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

fn handle_jump(
    keyboard_input: Res<Input<KeyCode>>,
    jump_height: Res<JumpHeight>,
    mut velocity: Mut<Velocity>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        velocity.0.set_y(jump_height.0);
    }
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
    mut worlds: Query<&mut World>,
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
    pipe_query: &mut Query<(&Pipe, &Translation, &Collider, &Sprite, Entity)>,
    score_query: &mut Query<(&Translation, &Collider, Entity)>,
    end_screen_query: &mut Query<(&EndScreen, &mut Draw)>,
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

fn velocity_animator_system(mut query: Query<(&mut Animations, &Velocity)>) {
    for (mut animations, velocity) in &mut query.iter() {
        if velocity.0.y() > 0.0 {
            animations.current_animation = 0;
        } else {
            animations.current_animation = 1;
        }
    }
}

pub fn spawn_bird(
    commands: &mut Commands,
    asset_server: &mut Res<AssetServer>,
    mut textures: &mut ResMut<Assets<Texture>>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/bird.png")
        .unwrap();

    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 2, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
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
