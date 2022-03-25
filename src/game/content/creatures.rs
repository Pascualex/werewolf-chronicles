use bevy::prelude::*;

use crate::game::{
    bundles::CreatureBundle,
    components::{Health, MovementStats, Position, Size},
};

pub fn hero(position: Vec2) -> CreatureBundle {
    CreatureBundle {
        position: Position::from_vec2(position),
        size: Size::new(50.0, 50.0),
        health: Health::new(1),
        movement_stats: MovementStats { speed: 300.0 },
        sprite_bundle: SpriteBundle {
            transform: Transform::from_scale(Vec3::new(50.0, 50.0, 0.0)),
            sprite: Sprite {
                color: Color::rgb(0.0, 0.3, 0.95),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn zombie(position: Vec2, size: f32, speed: f32, color: Color) -> CreatureBundle {
    CreatureBundle {
        position: Position::from_vec2(position),
        size: Size::new(50.0, 50.0),
        health: Health::new(2),
        movement_stats: MovementStats { speed },
        sprite_bundle: SpriteBundle {
            transform: Transform::from_scale(Vec3::new(size, size, 0.0)),
            sprite: Sprite {
                color,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}
