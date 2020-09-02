use bevy::prelude::*;

pub struct StartScreen;
pub struct EndScreen;

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let start_texture_handle = asset_server.load("assets/SpaceToStart.png").unwrap();
    let game_over_texture_handle = asset_server.load("assets/GameOverText.png").unwrap();
    commands
        // Start Screen
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
        .with(EndScreen);
}
