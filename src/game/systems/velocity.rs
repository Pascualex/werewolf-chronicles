use bevy::prelude::*;

use crate::game::{
    components::{Position, Velocity},
    TIME_STEP,
};

pub fn velocity_system(mut query: Query<(&mut Position, &mut Velocity)>) {
    for (mut position, mut velocity) in query.iter_mut() {
        position.value += velocity.total() * TIME_STEP;
        velocity.variable = Vec2::ZERO;
    }
}
