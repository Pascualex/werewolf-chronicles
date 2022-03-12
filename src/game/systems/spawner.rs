use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use rand::Rng;

use crate::game::{components::*, content::creatures, resources::*, TIME_STEP};

pub fn spawner_system(
    mut spawner: ResMut<Spawner>,
    mut query: Query<&Position, With<Player>>,
    mut commands: Commands,
) {
    spawner.timer.tick(Duration::from_secs_f32(TIME_STEP));
    if !spawner.timer.just_finished() {
        return;
    }

    let player_pos = match query.get_single_mut() {
        Ok(single) => single,
        Err(_) => return,
    };

    let mut rng = rand::thread_rng();
    for _ in 0..spawner.timer.times_finished() {
        let angle: f32 = rng.gen_range(0.0..(2.0 * PI));
        let (y, x) = angle.sin_cos();
        let offset = Vec2::new(x, y) * 1000.0;

        let pos = player_pos.value + offset;
        let seed: f32 = rng.gen_range(0.0..=1.0);
        let size = 50.0 - 20.0 * (1.0 - seed).powi(2);
        let speed = 100.0 + (1.0 - seed).powi(2) * 400.0;
        let color = Color::rgb(0.9 - seed * 0.7, 0.1 + seed * 0.5, 1.0 - seed * 0.8);
        commands
            .spawn()
            .insert(Ai)
            .insert_bundle(creatures::zombie(pos, size, speed, color));
    }
}
