use std::time::Duration;

use bevy::prelude::*;

use crate::game::{
    components::{Ai, Position, Turret},
    TIME_STEP,
};

pub fn turret_system(
    mut query: Query<(&Position, &mut Turret)>,
    ai_query: Query<&Position, With<Ai>>,
    mut commands: Commands,
) {
    for (pos, mut turret) in query.iter_mut() {
        turret.timer.tick(Duration::from_secs_f32(TIME_STEP));
        if !turret.timer.just_finished() {
            continue;
        }

        let min_difference = ai_query
            .iter()
            .map(|p| p.value - pos.value)
            .min_by(|a, b| a.length().partial_cmp(&b.length()).unwrap());

        if let Some(dir) = min_difference {
            let turret_progress = turret.timer.percent();
            let delay_per_shot = turret.timer.duration().as_secs_f32();
            for i in 0..turret.timer.times_finished() {
                let shot_delay = (turret_progress + i as f32) * delay_per_shot;
                turret.cast.cast(pos.value, dir, shot_delay, &mut commands);
            }
        }
    }
}
