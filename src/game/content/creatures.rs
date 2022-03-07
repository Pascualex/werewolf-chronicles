use bevy::prelude::*;

use crate::game::{bundles::CreatureBundle, components::MovementStats};

pub fn hero(pos: Vec2) -> CreatureBundle {
    CreatureBundle {
        movement_stats: MovementStats { speed: 300.0 },
        sprite_bundle: SpriteBundle {
            transform: Transform {
                translation: pos.extend(0.0),
                scale: Vec3::new(50.0, 50.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::BLUE,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn zombie(pos: Vec2, color: Color) -> CreatureBundle {
    CreatureBundle {
        movement_stats: MovementStats { speed: 100.0 },
        sprite_bundle: SpriteBundle {
            transform: Transform {
                translation: pos.extend(0.0),
                scale: Vec3::new(30.0, 30.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}
