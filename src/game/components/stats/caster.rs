use bevy::prelude::*;

use crate::game::content::{AbilityModifier, CastModifier};

#[derive(Component)]
pub struct CasterStats {
    pub cast_rate_multiplier: f32,
    pub cast_modifier: CastModifier,
    pub ability_modifier: AbilityModifier,
}
