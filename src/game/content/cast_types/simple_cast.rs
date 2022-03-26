use std::time::Duration;

use bevy::prelude::*;

use crate::game::content::Ability;

#[derive(Clone)]
pub struct SimpleCast {
    pub ability: Ability,
}

impl SimpleCast {
    pub fn spawn(
        &self,
        position: Vec2,
        direction: Vec2,
        progress: Duration,
        commands: &mut Commands,
    ) {
        let dir = direction.normalize_or_zero();
        let new_pos = position + (dir * self.ability.speed * progress.as_secs_f32());
        self.ability.spawn(new_pos, dir, commands);
    }
}
