use bevy::{core::FixedTimesteps, prelude::*};

use crate::game::{components::*, TIME_STEP, TIME_STEP_ID};

pub fn transform_system(
    mut query: Query<(&Position, Option<&Velocity>, &mut Transform)>,
    fixed_timesteps: Res<FixedTimesteps>,
) {
    let time_step_progress = match fixed_timesteps.get(TIME_STEP_ID) {
        Some(state) => state.overstep_percentage() as f32,
        None => return,
    };

    for (pos, vel, mut transform) in query.iter_mut() {
        transform.translation.x = pos.value.x;
        transform.translation.y = pos.value.y;

        if let Some(vel) = vel {
            transform.translation.x += vel.value.x * time_step_progress * TIME_STEP;
            transform.translation.y += vel.value.y * time_step_progress * TIME_STEP;
        }
    }
}
