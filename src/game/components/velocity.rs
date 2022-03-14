use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Velocity {
    pub constant: Vec2,
    pub variable: Vec2,
}

impl Velocity {
    pub fn from_vec2(constant: Vec2) -> Self {
        Self {
            constant,
            variable: Vec2::ZERO,
        }
    }

    pub fn total(&self) -> Vec2 {
        self.constant + self.variable
    }
}
