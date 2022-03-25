use bevy::prelude::*;

use crate::game::{components::*, content::*};

pub fn setup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let bullet = Ability {
        size: Vec2::new(15.0, 15.0),
        speed: 1200.0,
        lifetime: 3.0,
        color: Color::rgb(0.9, 0.1, 0.1),
        on_impact: Some(OnImpact::new_damage(5, true)),
    };

    commands
        .spawn()
        .insert(Player)
        .insert_bundle(creatures::hero(Vec2::ZERO))
        .insert(Turret::new(1.0, Cast::new_multi(50, 180.0, bullet)));
}
