use bevy::prelude::*;

use crate::game::{
    components::{Size, *},
    resources::*,
};

pub fn collision_grid_system(
    mut collision_grid: ResMut<CollisionGrid<Ai>>,
    query: Query<(Entity, &Position, &Size), With<Ai>>,
) {
    collision_grid.clear();
    for (entity, pos, size) in query.iter() {
        collision_grid.insert(entity, pos, size);
    }
}
