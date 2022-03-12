use bevy::prelude::*;

use crate::game::components::*;

pub fn ai_movement_system(
    mut ai_query: Query<(&Position, &mut Velocity, &MovementStats), With<Ai>>,
    mut player_query: Query<&Position, With<Player>>,
) {
    let player_pos = match player_query.get_single_mut() {
        Ok(single) => single,
        Err(_) => return,
    };

    for (ai_pos, mut ai_vel, ai_stats) in ai_query.iter_mut() {
        let dir = (player_pos.value - ai_pos.value).normalize_or_zero();
        ai_vel.value = dir * ai_stats.speed;
    }
}
