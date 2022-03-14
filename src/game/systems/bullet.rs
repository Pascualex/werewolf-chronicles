use bevy::prelude::*;

use crate::game::{
    components::{Size, *},
    resources::CollisionGrid,
};

pub fn bullet_system(
    bullet_query: Query<(Entity, &Position, &Size), With<Bullet>>,
    mut ai_query: Query<&mut Health, With<Ai>>,
    collision_grid: Res<CollisionGrid<Ai>>,
    mut commands: Commands,
) {
    for (bullet_entity, bullet_pos, bullet_size) in bullet_query.iter() {
        let ai_collisions = collision_grid.get_collisions(bullet_pos, bullet_size);
        for (ai_entity, _) in ai_collisions.iter() {
            let mut ai_health = match ai_query.get_mut(*ai_entity) {
                Ok(result) => result,
                Err(_) => continue,
            };

            if !ai_health.is_dead() {
                commands.entity(bullet_entity).despawn();
                ai_health.damage(1);
                break;
            }
        }
    }
}
