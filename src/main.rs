use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use rand::{thread_rng, Rng};
mod clouds;
mod physics;
mod pipes;
mod bounds_deletion;
mod gamestate;
mod gamedata;
mod mountains;
mod animation;
mod screens;
mod bird;

use animation::*;
use gamedata::*;
use clouds::*;
use pipes::*;
use physics::*;
use bounds_deletion::*;
use gamestate::*;
use mountains::*;
use screens::*;
use bird::*;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(PipePlugin)
        .add_plugin(BirdPlugin)
        .add_plugin(CloudPlugin)
        .add_plugin(MountainPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(ScreensPlugin)
        .add_plugin(GameStatePlugin)
        .add_startup_system(setup.system())
        .add_resource(ClearColor(Color::rgb(0.34, 0.75, 0.79)))
        .add_resource(JumpHeight(23.0 * 40.0))
        .add_resource(Gravity(45.0 * 40.0))
        .add_resource(GameData {
            game_state: GameState::Menu,
            score: 0,
        })
        .run();
}

fn setup(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn(Camera2dComponents::default());
    bird::spawn_bird(&mut commands, &mut asset_server, &mut materials, &mut textures, &mut texture_atlases);
}
