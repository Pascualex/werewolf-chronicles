use bevy::{prelude::*, utils::HashSet};

use crate::game::{components::*, resources::CollisionGrid};

pub fn bullet_system(
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    ai_query: Query<Entity, With<Ai>>,
    collision_grid: Res<CollisionGrid<Ai>>,
    mut commands: Commands,
) {
    let mut despawned_entities = HashSet::default();
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        let ai_result = collision_grid
            .get_collisions(bullet_transform)
            .iter()
            .filter(|e| !despawned_entities.contains(*e))
            .find_map(|e| ai_query.get(*e).ok());
        if let Some(ai_entity) = ai_result {
            commands.entity(bullet_entity).despawn();
            commands.entity(ai_entity).despawn();
            despawned_entities.insert(ai_entity);
        }
    }
}
