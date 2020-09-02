use bevy::prelude::*;

pub struct OffsceenDeletion;

pub struct BoundsDeletionPlugin;

impl Plugin for BoundsDeletionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(offscreen_remove_system.system());
    }
}

fn offscreen_remove_system(
    mut commands: Commands,
    mut worlds: Query<&mut World>,
    mut pipe_query: Query<(Entity, &mut Translation, &OffsceenDeletion)>,
) {
    let padding = 300.0;
    for (entity, translation, _od) in &mut pipe_query.iter() {
        // Left side of screen
        if translation.0.x() < -1920.0 * 0.5 - padding {
            for world in &mut worlds.iter() {
                // Due to despawning of entity from other systems, avoid despawn panic
                if !world.contains(entity) {
                    commands.despawn(entity);
                }
            }
        }
    }
}
