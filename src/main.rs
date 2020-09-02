use bevy::prelude::*;
mod animation;
mod bird;
mod bounds_deletion;
mod clouds;
mod gamedata;
mod gamestate;
mod mountains;
mod physics;
mod pipes;
mod screens;

use animation::*;
use bird::*;
use clouds::*;
use gamedata::*;
use gamestate::*;
use mountains::*;
use physics::*;
use pipes::*;
use screens::*;

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
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dComponents::default());
    bird::spawn_bird(
        &mut commands,
        &mut asset_server,
        &mut textures,
        &mut texture_atlases,
    );
}
