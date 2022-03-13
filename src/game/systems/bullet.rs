use bevy::{prelude::*, utils::HashSet};

use crate::game::{
    components::{Size, *},
    resources::CollisionGrid,
};

pub fn bullet_system(
    bullet_query: Query<(Entity, &Position, &Size), With<Bullet>>,
    ai_query: Query<Entity, With<Ai>>,
    collision_grid: Res<CollisionGrid<Ai>>,
    mut commands: Commands,
) {
    let mut despawned_entities = HashSet::default();
    for (bullet_entity, bullet_pos, bullet_size) in bullet_query.iter() {
        let ai_collisions = collision_grid.get_collisions(bullet_pos, bullet_size);
        for (ai_entity, _) in ai_collisions.iter() {
            let ai_entity = match ai_query.get(*ai_entity) {
                Ok(o) => o,
                Err(_) => continue,
            };

            if !despawned_entities.contains(&ai_entity) {
                commands.entity(bullet_entity).despawn();
                commands.entity(ai_entity).despawn();
                despawned_entities.insert(ai_entity);

                break;
            }
        }
    }
}
