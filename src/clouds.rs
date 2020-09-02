use crate::physics;
use bevy::prelude::*;
use physics::*;
use rand::{thread_rng, Rng};

pub struct CloudTimer(Timer);

pub struct CloudPlugin;

impl Plugin for CloudPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(cloud_spawn_system.system())
            .add_resource(CloudTimer(Timer::from_seconds(1.0, true)));
    }
}

fn cloud_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut cloud_timer: ResMut<CloudTimer>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = thread_rng();
    let cloud_texture = match rng.gen_bool(0.5) {
        true => asset_server.load("assets/cloud_1.png").unwrap(),
        false => asset_server.load("assets/cloud_2.png").unwrap(),
    };

    cloud_timer.0.tick(time.delta_seconds);
    if cloud_timer.0.finished {
        commands
            .spawn(SpriteComponents {
                material: materials.add(cloud_texture.into()),
                scale: Scale(rng.gen_range(6.0, 30.0)),
                translation: Translation(Vec3::new(
                    1920.0 * 0.5 + 30.0 * 43.0,
                    rng.gen_range(-1280.0 * 0.5, 1280.0 * 0.5),
                    2.0,
                )),
                ..Default::default()
            })
            .with(Velocity(Vec2::new(
                rng.gen_range(-700.0, -400.0),
                rng.gen_range(-10.0, 10.0),
            )));
    }
}
