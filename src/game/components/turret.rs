use bevy::prelude::*;

use crate::game::content::Cast;

#[derive(Component)]
pub struct Turret {
    pub timer: Timer,
    pub cast: Cast,
}

impl Turret {
    pub fn new(fire_rate: f32, cast: Cast) -> Self {
        Self {
            timer: Timer::from_seconds(1.0 / fire_rate, true),
            cast,
        }
    }
}
