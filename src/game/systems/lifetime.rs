use std::time::Duration;

use bevy::prelude::*;

use crate::game::{components::Lifetime, TIME_STEP};

pub fn lifetime_system(mut query: Query<(Entity, &mut Lifetime)>, mut commands: Commands) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.timer.tick(Duration::from_secs_f32(TIME_STEP));
        if lifetime.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
