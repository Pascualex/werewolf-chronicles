use bevy::prelude::*;

use crate::game::{
    components::{Size, *},
    resources::CollisionGrid,
};

pub fn overlap_system(
    mut query: Query<(Entity, &Position, &Size, &mut Velocity), With<Ai>>,
    collision_grid: Res<CollisionGrid<Ai>>,
) {
    for (entity, pos, size, mut vel) in query.iter_mut() {
        let collisions = collision_grid.get_collisions(pos, size);
        let mut total_force = Vec2::ZERO;
        for (other_entity, force) in collisions.iter() {
            if *other_entity != entity {
                total_force += *force;
            }
        }
        let dir = total_force.normalize_or_zero();
        let std_force = total_force * 5.0;
        let max_force = dir * size.value * 10.0;
        if std_force.length() < max_force.length() {
            vel.variable += std_force;
        } else {
            vel.variable += max_force;
        }
    }
}
