use bevy::prelude::*;

use crate::game::content::Ability;

#[derive(Clone)]
pub struct MultiCast {
    pub shots: u32,
    pub arc: f32,
    pub ability: Ability,
}

impl MultiCast {
    pub fn cast(&self, position: Vec2, direction: Vec2, delay: f32, commands: &mut Commands) {
        if self.shots == 0 {
            return;
        }

        let cast_angle = Vec2::X.angle_between(direction);
        let angle_per_shot = (self.arc.to_radians() * 2.0) / self.shots as f32;
        let left_angle = cast_angle - self.arc.to_radians() + (angle_per_shot / 2.0);

        for i in 0..self.shots {
            let shot_angle = left_angle + angle_per_shot * i as f32;
            let (y, x) = shot_angle.sin_cos();
            let shot_dir = Vec2::new(x, y);
            let shot_pos = position + (shot_dir * self.ability.speed * delay);

            self.ability.spawn(shot_pos, shot_dir, commands);
        }
    }
}
