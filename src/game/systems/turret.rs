use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::game::{components::*, TIME_STEP};

// pub fn turret_system(mut query: Query<(&mut Turret, &Transform)>, mut commands: Commands) {
//     for (mut turret, transform) in query.iter_mut() {
//         turret.timer.tick(Duration::from_secs_f32(TIME_STEP));
//         if !turret.timer.just_finished() {
//             continue;
//         }

//         let pos = transform.translation.truncate();
//         let mut rng = rand::thread_rng();
//         for _ in 0..turret.timer.times_finished() {
//             for _ in 0..50 {
//                 let angle: f32 = rng.gen_range(0.0..(2.0 * PI));
//                 let (y, x) = angle.sin_cos();
//                 let dir = Vec2::new(x, y);
//                 spawn_bullet(pos, dir, &mut commands);
//             }
//         }
//     }
// }

pub fn turret_system(
    mut query: Query<(&mut Turret, &Transform)>,
    ai_query: Query<&Transform, With<Ai>>,
    mut commands: Commands,
) {
    let mut rng = rand::thread_rng();
    for (mut turret, transform) in query.iter_mut() {
        turret.timer.tick(Duration::from_secs_f32(TIME_STEP));
        if !turret.timer.just_finished() {
            continue;
        }

        let min_difference = ai_query
            .iter()
            .map(|t| (t.translation - transform.translation).truncate())
            .min_by(|a, b| a.length().partial_cmp(&b.length()).unwrap());

        if let Some(difference) = min_difference {
            let pos = transform.translation.truncate();
            let dir = match difference.try_normalize() {
                Some(dir) => dir,
                None => Vec2::Y,
            };
            let angle = Vec2::X.angle_between(dir);
            for _ in 0..turret.timer.times_finished() {
                let offset: f32 = rng.gen_range(-15.0..=15.0);
                let new_angle = angle + offset.to_radians();
                let (y, x) = new_angle.sin_cos();
                let new_dir = Vec2::new(x, y);
                let speed = rng.gen_range(900.0..=1100.0);
                spawn_bullet(pos, new_dir, speed, &mut commands);
            }
        }
    }
}

fn spawn_bullet(pos: Vec2, dir: Vec2, speed: f32, commands: &mut Commands) {
    commands
        .spawn()
        .insert(Bullet)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: pos.extend(0.0),
                scale: Vec3::new(10.0, 10.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Velocity {
            translation: dir * speed,
        })
        .insert(Lifetime {
            timer: Timer::from_seconds(3.0, false),
        });
}
