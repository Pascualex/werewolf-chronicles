use bevy::prelude::*;

use crate::game::components::*;

pub fn ai_movement_system(
    mut query: Query<(&Transform, &MovementStats, &mut Velocity), With<Ai>>,
    mut player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = match player_query.get_single_mut() {
        Ok(single) => single,
        Err(_) => return,
    };

    for (transform, stats, mut velocity) in query.iter_mut() {
        let raw_dir = player_transform.translation - transform.translation;
        let dir = raw_dir.truncate().normalize_or_zero();
        velocity.translation = dir * stats.speed;
    }
}
