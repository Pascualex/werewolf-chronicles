use std::time::Duration;

use bevy::prelude::*;

use crate::game::{
    components::{Ai, CasterStats, Casters, Position},
    TIME_STEP,
};

pub fn caster_system(
    mut query: Query<(&Position, &mut Casters, Option<&CasterStats>)>,
    ai_query: Query<&Position, With<Ai>>,
    mut commands: Commands,
) {
    let delta = Duration::from_secs_f32(TIME_STEP);
    for (pos, mut casters, stats) in query.iter_mut() {
        let min_difference = ai_query
            .iter()
            .map(|p| p.value - pos.value)
            .min_by(|a, b| a.length().partial_cmp(&b.length()).unwrap());

        let dir = match min_difference {
            Some(s) => s,
            None => continue,
        };

        let cast_rate_multiplier = &stats.map_or(1.0, |s| s.cast_rate_multiplier);
        for (caster, timer) in casters.casters.iter_mut() {
            let cast_rate = caster.cast_rate * cast_rate_multiplier;
            let cast_cd = Duration::from_secs_f32(1.0 / cast_rate);
            let (cast_count, next_cast_percent) = timer.tick(delta, cast_cd);

            for i in 0..cast_count {
                let progress = cast_cd.mul_f32(next_cast_percent + i as f32);
                caster.cast.spawn(
                    pos.value,
                    dir,
                    progress,
                    stats.map(|s| s.cast_modifier.clone()),
                    stats.map(|s| s.ability_modifier.clone()),
                    &mut commands,
                );
            }
        }
    }
}
