use crate::bounds_deletion;
use crate::physics;
use bevy::prelude::*;
use bounds_deletion::*;
use physics::*;
use rand::{thread_rng, Rng};

// Spawn mountains with a delay
pub struct MountainTimer(pub Timer);

pub struct MountainPlugin;

impl Plugin for MountainPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(mountain_spawn_system.system())
            .add_resource(MountainTimer(Timer::from_seconds(3.0, true)));
    }
}

fn mountain_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut mountain_timer: ResMut<MountainTimer>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = thread_rng();
    let mountain_texture = match rng.gen_bool(0.5) {
        true => asset_server.load("assets/mountain.png").unwrap(),
        false => asset_server.load("assets/mountain.png").unwrap(),
    };

    mountain_timer.0.tick(time.delta_seconds);
    if mountain_timer.0.finished {
        commands
            .spawn(SpriteComponents {
                scale: Scale(3.0),
                material: materials.add(ColorMaterial::modulated_texture(
                    mountain_texture,
                    Color::rgb(0.36, 0.36, 0.36),
                )),
                translation: Translation(Vec3::new(1920.0 * 0.5 + 30.0 * 43.0, -1280.0 * 0.5, 0.2)),
                ..Default::default()
            })
            .with(OffsceenDeletion)
            .with(Velocity(Vec2::new(-200.0, 0.0)));
        commands
            .spawn(SpriteComponents {
                scale: Scale(3.0),
                translation: Translation(Vec3::new(
                    1920.0 * 0.5 + 30.0 * 43.0,
                    -1280.0 * 0.5 - 100.0,
                    0.3,
                )),
                material: materials.add(ColorMaterial::modulated_texture(
                    mountain_texture,
                    Color::rgb(0.26, 0.26, 0.26),
                )),
                ..Default::default()
            })
            .with(OffsceenDeletion)
            .with(Velocity(Vec2::new(-400.0, 0.0)));
    }
}
