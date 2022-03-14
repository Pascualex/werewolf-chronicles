use bevy::prelude::*;

use crate::game::components::*;

pub fn death_system(query: Query<(Entity, &Health)>, mut commands: Commands) {
    for (entity, health) in query.iter() {
        if health.is_dead() {
            commands.entity(entity).despawn();
        }
    }
}
