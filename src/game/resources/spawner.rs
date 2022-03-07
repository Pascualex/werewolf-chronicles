use bevy::prelude::*;

pub struct Spawner {
    pub timer: Timer,
}

impl Spawner {
    pub fn new(spawn_rate: f32) -> Self {
        Self {
            timer: Timer::from_seconds(1.0 / spawn_rate, true),
        }
    }
}
