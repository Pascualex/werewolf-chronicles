use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Position {
    pub value: Vec2,
}

impl Position {
    pub fn from_vec2(value: Vec2) -> Self {
        Self { value }
    }
}
