use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::game::{
    components::{Size, *},
    TIME_STEP,
};

pub fn turret_system(
    mut query: Query<(&Position, &mut Turret)>,
    ai_query: Query<&Position, With<Ai>>,
    mut commands: Commands,
) {
    let mut rng = rand::thread_rng();
    for (pos, mut turret) in query.iter_mut() {
        turret.timer.tick(Duration::from_secs_f32(TIME_STEP));
        if !turret.timer.just_finished() {
            continue;
        }

        let min_difference = ai_query
            .iter()
            .map(|p| p.value - pos.value)
            .min_by(|a, b| a.length().partial_cmp(&b.length()).unwrap());

        if let Some(difference) = min_difference {
            let dir = match difference.try_normalize() {
                Some(dir) => dir,
                None => Vec2::Y,
            };
            let speed = 1000.0;
            let dist_per_shot = speed * turret.timer.duration().as_secs_f32();
            let dist_offset = dist_per_shot * turret.timer.percent();
            let angle = Vec2::X.angle_between(dir);
            for i in 0..turret.timer.times_finished() {
                let angle_offset: f32 = rng.gen_range(-15.0..=15.0);
                let new_angle = angle + angle_offset.to_radians();
                let (y, x) = new_angle.sin_cos();
                let new_dir = Vec2::new(x, y);
                let new_pos = pos.value + new_dir * dist_per_shot * (dist_offset + i as f32);
                spawn_bullet(new_pos, new_dir, speed, &mut commands);
            }
        }
    }
}

fn spawn_bullet(position: Vec2, direction: Vec2, speed: f32, commands: &mut Commands) {
    commands
        .spawn()
        .insert(Bullet)
        .insert(Position::from_vec2(position))
        .insert(Size::new(10.0, 10.0))
        .insert(Velocity::from_vec2(direction * speed))
        .insert(Lifetime::new(3.0))
        .insert_bundle(SpriteBundle {
            transform: Transform::from_scale(Vec3::new(10.0, 10.0, 0.0)),
            sprite: Sprite {
                color: Color::rgb(0.85, 0.1, 0.1),
                ..Default::default()
            },
            ..Default::default()
        });
}
