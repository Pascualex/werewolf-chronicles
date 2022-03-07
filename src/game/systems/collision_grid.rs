use bevy::prelude::*;

use crate::game::{components::*, resources::*};

pub fn collision_grid_system(
    mut collision_grid: ResMut<CollisionGrid<Ai>>,
    query: Query<(Entity, &Transform), With<Ai>>,
) {
    collision_grid.clear();
    for (entity, transform) in query.iter() {
        collision_grid.insert(entity, transform);
    }
}
