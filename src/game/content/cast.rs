use std::time::Duration;

use bevy::prelude::*;

use crate::game::content::{Ability, AbilityModifier, CastModifier, MultiCast};

#[derive(Clone)]
pub enum Cast {
    Multi(MultiCast),
}

impl Cast {
    pub fn new_multi(count: u32, arc: f32, ability: Ability) -> Self {
        Cast::Multi(MultiCast {
            count,
            arc,
            ability,
        })
    }

    pub fn spawn(
        &self,
        position: Vec2,
        direction: Vec2,
        progress: Duration,
        cast_modifier: Option<CastModifier>,
        ability_modifier: Option<AbilityModifier>,
        commands: &mut Commands,
    ) {
        match self {
            Self::Multi(m) => m.spawn(
                position,
                direction,
                progress,
                cast_modifier,
                ability_modifier,
                commands,
            ),
        }
    }
}
