use bevy::prelude::*;

pub struct Velocity(pub Vec2);

pub struct Gravity(pub f32);
pub struct AffectedByGravity;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(velocity_system.system())
            .add_system(gravity_system.system());
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

fn velocity_system(time: Res<Time>, mut position: Mut<Translation>, velocity: Mut<Velocity>) {
    let y = position.0.y();
    let x = position.0.x();
    let delta = time.delta_seconds;
    position.0.set_y(y + velocity.0.y() * delta);
    position.0.set_x(x + velocity.0.x() * delta);
}
