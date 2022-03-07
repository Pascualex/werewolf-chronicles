use bevy::prelude::*;

use crate::game::components::*;

#[derive(Bundle, Default)]
pub struct CreatureBundle {
    pub movement_stats: MovementStats,
    pub velocity: Velocity,
    #[bundle]
    pub sprite_bundle: SpriteBundle,
}
