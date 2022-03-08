use std::sync::{Arc, Mutex};

use bevy::{prelude::*, tasks::TaskPool, utils::HashSet};

use crate::game::{components::*, resources::CollisionGrid};

pub fn bullet_system(
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    ai_query: Query<Entity, With<Ai>>,
    collision_grid: Res<CollisionGrid<Ai>>,
    mut commands: Commands,
) {
    let collisions_query = par_get_collision_query(&bullet_query, &collision_grid);

    let mut despawned_entities = HashSet::default();
    for (bullet_entity, bullet_collisions) in collisions_query.iter() {
        let (bullet_entity, _) = match bullet_query.get(*bullet_entity) {
            Ok(o) => o,
            Err(_) => continue,
        };

        for ai_entity in bullet_collisions.iter() {
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

fn par_get_collision_query(
    bullet_query: &Query<(Entity, &Transform), With<Bullet>>,
    collision_grid: &Res<CollisionGrid<Ai>>,
) -> Vec<(Entity, Vec<Entity>)> {
    let task_pool = TaskPool::new();
    let collisions_query = Arc::new(Mutex::new(Vec::new()));
    bullet_query.par_for_each(&task_pool, 100, |(entity, transform)| {
        let bullet_collisions = collision_grid.get_collisions(transform);
        if !bullet_collisions.is_empty() {
            collisions_query.lock().unwrap().push((entity, bullet_collisions));
        }
    });
    let collision_query = collisions_query.lock().unwrap().to_owned();
    collision_query
}
