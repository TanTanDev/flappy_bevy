use crate::bird;
use crate::gamedata;
use crate::physics;
use crate::screens;
use bevy::prelude::*;

use bird::*;
use gamedata::*;
use physics::*;
use screens::*;

#[derive(std::cmp::PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    Dead,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(handle_gamestate_system.system());
    }
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
