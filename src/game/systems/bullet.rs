use std::collections::hash_map::Entry;

use bevy::{
    prelude::*,
    sprite::collide_aabb::collide,
    utils::{HashMap, HashSet},
};

use crate::game::components::*;

pub fn bullet_system(
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    ai_query: Query<(Entity, &Transform), With<Ai>>,
    mut commands: Commands,
) {
    let mut ai_grid = HashMap::default();
    for (ai_entity, ai_transform) in ai_query.iter() {
        let pos = ai_transform.translation.truncate() / 30.0;
        let pos = (pos.x as i32, pos.y as i32);
        let bucket = match ai_grid.entry(pos) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(Vec::new()),
        };
        bucket.push((ai_entity, ai_transform));
    }

    let mut despawn_entities = HashSet::default();
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        let bullet_pos = bullet_transform.translation;
        let bullet_size = bullet_transform.scale.truncate();
        let mut collision = false;

        for offset in [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (0, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ] {
            let pos = bullet_pos / 30.0;
            let pos = (pos.x as i32 + offset.0, pos.y as i32 + offset.1);
            let bucket = match ai_grid.get(&pos) {
                Some(s) => s,
                None => continue,
            };

            for (ai_entity, ai_transform) in bucket.iter() {
                if despawn_entities.contains(ai_entity) {
                    continue;
                }

                let ai_pos = ai_transform.translation;
                let ai_size = ai_transform.scale.truncate();

                if collide(bullet_pos, bullet_size, ai_pos, ai_size).is_some() {
                    commands.entity(bullet_entity).despawn();
                    despawn_entities.insert(ai_entity.to_owned());
                    collision = true;
                    break;
                }
            }

            if collision {
                break;
            }
        }
    }

    despawn_entities
        .iter()
        .for_each(|e| commands.entity(*e).despawn());
}
