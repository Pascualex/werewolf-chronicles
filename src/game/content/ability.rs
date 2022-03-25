use bevy::prelude::*;

use crate::game::{
    components::{Handler, Lifetime, Position, Size, Velocity},
    content::OnImpact,
};

#[derive(Clone)]
pub struct Ability {
    pub size: Vec2,
    pub speed: f32,
    pub lifetime: f32,
    pub color: Color,
    pub on_impact: Option<OnImpact>,
}

impl Ability {
    pub fn spawn(&self, position: Vec2, direction: Vec2, commands: &mut Commands) {
        let mut entity_commands = commands.spawn();

        let normalized_dir = direction.normalize_or_zero();
        entity_commands
            .insert(Position::from_vec2(position))
            .insert(Size::from_vec2(self.size))
            .insert(Velocity::from_vec2(normalized_dir * self.speed))
            .insert(Lifetime::new(self.lifetime))
            .insert_bundle(SpriteBundle {
                transform: Transform::from_scale(self.size.extend(0.0)),
                sprite: Sprite {
                    color: self.color,
                    ..Default::default()
                },
                ..Default::default()
            });

        if let Some(on_impact) = self.on_impact.clone() {
            entity_commands.insert(Handler::<OnImpact>::new(on_impact));
        }
    }
}
