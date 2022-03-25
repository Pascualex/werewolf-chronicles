use bevy::prelude::*;

use crate::game::components::{MovementStats, Player, Velocity};

pub fn player_movement_system(
    mut query: Query<(&mut Velocity, &MovementStats), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let (mut velocity, stats) = match query.get_single_mut() {
        Ok(single) => single,
        Err(_) => return,
    };

    let mut dir = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::Up) {
        dir.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        dir.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        dir.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        dir.x -= 1.0;
    }

    velocity.variable += dir * stats.speed;
}
