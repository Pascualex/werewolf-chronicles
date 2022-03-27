use bevy::prelude::*;

#[derive(Component, Default)]
pub struct MovementStats {
    pub speed: f32,
}

impl MovementStats {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}
