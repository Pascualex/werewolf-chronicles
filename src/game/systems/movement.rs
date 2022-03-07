use bevy::prelude::*;

use crate::game::{components::*, TIME_STEP};

pub fn movement_system(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.translation.extend(0.0) * TIME_STEP;
    }
}
