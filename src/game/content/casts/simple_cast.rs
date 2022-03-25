use bevy::prelude::*;

use crate::game::content::Ability;

#[derive(Clone)]
pub struct SimpleCast {
    pub ability: Ability,
}

impl SimpleCast {
    pub fn cast(&self, position: Vec2, direction: Vec2, delay: f32, commands: &mut Commands) {
        let dir = direction.normalize_or_zero();
        let new_pos = position + (dir * self.ability.speed * delay);
        self.ability.spawn(new_pos, dir, commands);
    }
}
