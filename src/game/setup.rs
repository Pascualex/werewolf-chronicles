use bevy::prelude::*;

use super::{components::*, content::creatures};

pub fn setup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn()
        .insert(Player)
        .insert_bundle(creatures::hero(Vec2::ZERO))
        .insert(Turret::new(10_500.0));
}
