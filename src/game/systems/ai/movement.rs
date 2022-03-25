use bevy::prelude::*;

use crate::game::components::{Ai, MovementStats, Player, Position, Velocity};

pub fn ai_movement_system(
    mut ai_query: Query<(&Position, &mut Velocity, &MovementStats), With<Ai>>,
    player_query: Query<&Position, With<Player>>,
) {
    let player_pos = match player_query.get_single() {
        Ok(single) => single,
        Err(_) => return,
    };

    for (ai_pos, mut ai_vel, ai_stats) in ai_query.iter_mut() {
        let dir = (player_pos.value - ai_pos.value).normalize_or_zero();
        ai_vel.variable += dir * ai_stats.speed;
    }
}
