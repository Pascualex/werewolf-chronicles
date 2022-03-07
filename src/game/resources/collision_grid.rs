use std::{collections::hash_map::Entry, marker::PhantomData};

use bevy::{prelude::*, sprite::collide_aabb::collide, utils::HashMap};

pub struct CollisionGrid<T: Component> {
    cell_size: f32,
    grid: HashMap<IVec2, Vec<(Entity, Vec3, Vec2)>>,
    phantom_data: PhantomData<T>,
}

impl<T: Component> CollisionGrid<T> {
    pub fn new(cell_size: f32) -> Self {
        Self {
            grid: HashMap::default(),
            cell_size,
            phantom_data: PhantomData::default(),
        }
    }

    pub fn insert(&mut self, entity: Entity, transform: &Transform) {
        let pos = transform.translation;
        let size = transform.scale.truncate();
        let idx = (pos.truncate() / self.cell_size).as_ivec2();
        let bucket = match self.grid.entry(idx) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(Vec::new()),
        };
        bucket.push((entity, pos, size));
    }

    pub fn get_collisions(&self, transform: &Transform) -> Vec<Entity> {
        let pos = transform.translation;
        let size = transform.scale.truncate();
        let idx = (pos.truncate() / self.cell_size).as_ivec2();
        let mut collisions = Vec::new();
        for y in [-1, 0, 1] {
            for x in [-1, 0, 1] {
                let neighbor_idx = idx + IVec2::new(x, y);
                let bucket = match self.grid.get(&neighbor_idx) {
                    Some(s) => s,
                    None => continue,
                };
                for (col_entity, col_pos, col_size) in bucket.iter() {
                    if collide(pos, size, col_pos.to_owned(), col_size.to_owned()).is_some() {
                        collisions.push(col_entity.to_owned());
                    }
                }
            }
        }
        collisions
    }

    pub fn clear(&mut self) {
        self.grid.clear();
    }
}
