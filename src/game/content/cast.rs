use std::time::Duration;

use bevy::prelude::*;

use crate::game::content::{MultiCast, SimpleCast};

use super::Ability;

#[derive(Clone)]
pub enum Cast {
    Simple(SimpleCast),
    Multi(MultiCast),
}

impl Cast {
    pub fn new_simple(ability: Ability) -> Self {
        Cast::Simple(SimpleCast { ability })
    }

    pub fn new_multi(shots: u32, arc: f32, ability: Ability) -> Self {
        Cast::Multi(MultiCast {
            shots,
            arc,
            ability,
        })
    }

    pub fn spawn(
        &self,
        position: Vec2,
        direction: Vec2,
        progress: Duration,
        commands: &mut Commands,
    ) {
        match self {
            Self::Simple(s) => s.spawn(position, direction, progress, commands),
            Self::Multi(m) => m.spawn(position, direction, progress, commands),
        }
    }
}
