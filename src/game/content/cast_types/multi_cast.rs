use std::time::Duration;

use bevy::prelude::*;

use crate::game::content::{Ability, AbilityModifier, CastModifier};

#[derive(Clone)]
pub struct MultiCast {
    pub count: u32,
    pub arc: f32,
    pub ability: Ability,
}

impl MultiCast {
    pub fn spawn(
        &self,
        position: Vec2,
        direction: Vec2,
        progress: Duration,
        cast_modifier: Option<CastModifier>,
        ability_modifier: Option<AbilityModifier>,
        commands: &mut Commands,
    ) {
        let count = self.count + cast_modifier.map_or(0, |m| m.extra_casts);

        if count == 0 {
            return;
        }

        let cast_angle = Vec2::X.angle_between(direction);
        let angle_per_ability = (self.arc.to_radians() * 2.0) / count as f32;
        let left_angle = cast_angle - self.arc.to_radians() + (angle_per_ability / 2.0);

        for i in 0..count {
            let ability_angle = left_angle + angle_per_ability * i as f32;
            let (y, x) = ability_angle.sin_cos();
            let ability_dir = Vec2::new(x, y);
            let ability_offset = ability_dir * self.ability.speed * progress.as_secs_f32();
            let ability_pos = position + ability_offset;

            self.ability
                .spawn(ability_pos, ability_dir, ability_modifier.clone(), commands);
        }
    }
}
