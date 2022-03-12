use std::{collections::hash_map::Entry, marker::PhantomData};

use bevy::{prelude::*, sprite::collide_aabb::collide, utils::HashMap};

use crate::game::components::{Size, *};

pub struct CollisionGrid<T: Component> {
    cell_size: f32,
    grid: HashMap<IVec2, Vec<(Entity, Vec2, Vec2)>>,
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

    pub fn insert(&mut self, entity: Entity, position: &Position, size: &Size) {
        let idx = (position.value / self.cell_size).as_ivec2();
        let bucket = match self.grid.entry(idx) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(Vec::new()),
        };
        bucket.push((entity, position.value, size.value));
    }

    pub fn get_collisions(&self, position: &Position, size: &Size) -> Vec<Entity> {
        let idx = (position.value / self.cell_size).as_ivec2();
        let mut collisions = Vec::new();
        for y in [-1, 0, 1] {
            for x in [-1, 0, 1] {
                let neighbor_idx = idx + IVec2::new(x, y);
                let bucket = match self.grid.get(&neighbor_idx) {
                    Some(s) => s,
                    None => continue,
                };
                for (col_entity, col_pos, col_size) in bucket.iter() {
                    let pos = position.value.extend(0.0);
                    let size = size.value;
                    if collide(pos, size, col_pos.extend(0.0), col_size.to_owned()).is_some() {
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
