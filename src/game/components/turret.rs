use bevy::prelude::*;

#[derive(Component)]
pub struct Turret {
    pub timer: Timer,
}

impl Turret {
    pub fn new(fire_rate: f32) -> Self {
        Self {
            timer: Timer::from_seconds(1.0 / fire_rate, true),
        }
    }
}
