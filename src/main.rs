use bevy:: {
    prelude::*,
    input::{keyboard::KeyCode, Input},
};


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

struct Player;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        //.add_system(animate_sprite_system.system())
        .add_system(animate_system.system())
        .add_system(player_input.system())
        .run();
}

fn player_input(keyboard_input: Res<Input<KeyCode>>, _player: Mut<Player>, mut animations: Mut<Animations>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        animations.current_animation +=1;
        if animations.current_animation as usize >= animations.animations.len() {
            animations.current_animation = 0;
        }
        println!("current animation: {}", animations.current_animation);
    }
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/bird.png")
        .unwrap();
    asset_server.watch_for_changes().unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 2, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            scale: Scale(6.0),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))
        .with(Player)
        .with(Animations {
            animations: vec![Animation {
                current_frame: 0,
                frames: vec![AnimationFrame {
                    index: 0,
                    time: 0.1,
                }, AnimationFrame {
                    index: 1,
                    time: 0.1,
                }, AnimationFrame {
                    index: 2,
                    time: 0.3,
                }, AnimationFrame {
                    index: 1,
                    time: 0.1,
                }],
            }, Animation {
                current_frame: 0,
                frames: vec![AnimationFrame {
                    index: 0,
                    time: 0.2,
                },],
            }],
            current_animation: 0,
        });
}
