use bevy::prelude::*;

use crate::game::{components::*, TIME_STEP};

pub fn velocity_system(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        position.value += velocity.value * TIME_STEP;
    }
}
