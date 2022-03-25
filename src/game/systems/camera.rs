use bevy::prelude::*;

use crate::game::components::Player;

pub fn camera_system(
    mut camera_query: Query<(&mut Transform, &Camera), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let camera_query_single = camera_query
        .iter_mut()
        .find(|(_, c)| c.name.as_deref() == Some("camera_2d"));

    let (mut camera_transform, _) = match camera_query_single {
        Some(single) => single,
        None => return,
    };

    let player_transform = match player_query.get_single() {
        Ok(single) => single,
        Err(_) => return,
    };

    // let diff = (camera_transform.translation - player_transform.translation).truncate();
    // if diff.x.abs() >= 15.0 {
    //     let new_diff_x = diff.x.signum() * 15.0;
    //     camera_transform.translation.x = player_transform.translation.x + new_diff_x;
    // }
    // if diff.y.abs() >= 15.0 {
    //     let new_diff_y = diff.y.signum() * 15.0;
    //     camera_transform.translation.y = player_transform.translation.y + new_diff_y;
    // }

    let diff = (player_transform.translation - camera_transform.translation).truncate();
    let dir = diff.normalize_or_zero();
    let const_vel = dir * 50.0;
    let var_vel = diff * 5.0;
    let delta = (const_vel + var_vel) * time.delta_seconds();
    if delta.length() < diff.length() {
        camera_transform.translation += delta.extend(0.0);
    } else {
        camera_transform.translation = player_transform.translation;
    }
}
