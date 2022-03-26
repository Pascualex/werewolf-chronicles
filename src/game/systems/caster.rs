use std::time::Duration;

use bevy::prelude::*;

use crate::game::{
    components::{Ai, Casters, Position},
    TIME_STEP,
};

pub fn caster_system(
    mut query: Query<(&Position, &mut Casters)>,
    ai_query: Query<&Position, With<Ai>>,
    mut commands: Commands,
) {
    let delta = Duration::from_secs_f32(TIME_STEP);
    for (pos, mut casters) in query.iter_mut() {
        let min_difference = ai_query
            .iter()
            .map(|p| p.value - pos.value)
            .min_by(|a, b| a.length().partial_cmp(&b.length()).unwrap());

        let dir = match min_difference {
            Some(s) => s,
            None => continue,
        };

        for (caster, timer) in casters.casters.iter_mut() {
            let cast_cd = Duration::from_secs_f32(1.0 / caster.cast_rate);
            let (cast_count, next_cast_percent) = timer.tick(delta, cast_cd);

            for i in 0..cast_count {
                let progress = cast_cd.mul_f32(next_cast_percent + i as f32);
                caster.cast.spawn(pos.value, dir, progress, &mut commands);
            }
        }
    }
}
