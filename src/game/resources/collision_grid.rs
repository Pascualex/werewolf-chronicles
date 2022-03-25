use std::{collections::hash_map::Entry, marker::PhantomData};

use bevy::{prelude::*, utils::HashMap};
use rand::Rng;

use crate::game::components::{Position, Size};

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

    pub fn get_collisions(&self, position: &Position, size: &Size) -> Vec<(Entity, Vec2)> {
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
                    let pos = position.value;
                    let size = size.value;
                    if let Some(force) = collide(pos, size, *col_pos, *col_size) {
                        collisions.push((*col_entity, force));
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

pub fn collide(a_pos: Vec2, a_size: Vec2, b_pos: Vec2, b_size: Vec2) -> Option<Vec2> {
    let a_min = a_pos - a_size / 2.0;
    let a_max = a_pos + a_size / 2.0;

    let b_min = b_pos - b_size / 2.0;
    let b_max = b_pos + b_size / 2.0;

    let mut rng = rand::thread_rng();
    if a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y {
        let right_depth = a_max.x - b_min.x;
        let left_depth = b_max.x - a_min.x;
        let x_force = if right_depth < left_depth {
            -right_depth
        } else if left_depth < right_depth {
            left_depth
        } else {
            rng.gen_range(-right_depth..left_depth)
        };

        let up_depth = (a_max.y - b_min.y) * rng.gen_range(0.99..1.01);
        let down_depth = b_max.y - a_min.y;
        let y_force = if up_depth < down_depth {
            -up_depth
        } else if down_depth < up_depth {
            down_depth
        } else {
            rng.gen_range(-up_depth..down_depth)
        };

        Some(Vec2::new(x_force, y_force))
    } else {
        None
    }
}
