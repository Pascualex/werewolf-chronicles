use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use rand::Rng;

use crate::game::{components::*, content::creatures, resources::*, TIME_STEP};

pub fn spawner_system(
    mut spawner: ResMut<Spawner>,
    mut query: Query<&Transform, With<Player>>,
    mut commands: Commands,
) {
    spawner.timer.tick(Duration::from_secs_f32(TIME_STEP));
    if !spawner.timer.just_finished() {
        return;
    }

    let player_transform = match query.get_single_mut() {
        Ok(single) => single,
        Err(_) => return,
    };

    let mut rng = rand::thread_rng();
    for _ in 0..spawner.timer.times_finished() {
        let angle: f32 = rng.gen_range(0.0..(2.0 * PI));
        let (y, x) = angle.sin_cos();
        let offset = Vec2::new(x, y) * 1000.0;

        let pos = player_transform.translation.truncate() + offset;
        let color = Color::rgb(
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
        );
        commands
            .spawn()
            .insert(Ai)
            .insert_bundle(creatures::zombie(pos, color));
    }
}
