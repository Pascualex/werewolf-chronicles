use bevy::prelude::*;

use crate::game::{
    components::{Ai, Handler, Health, Position, Size, Velocity},
    content::OnImpact,
    resources::CollisionGrid,
};

pub fn impact_system(
    impact_query: Query<(Entity, &Position, &Size, &Velocity, &Handler<OnImpact>)>,
    mut ai_query: Query<&mut Health, With<Ai>>,
    collision_grid: Res<CollisionGrid<Ai>>,
    mut commands: Commands,
) {
    for (impact_entity, impact_pos, impact_size, impact_vel, on_impact) in impact_query.iter() {
        let ai_collisions = collision_grid.get_collisions(impact_pos, impact_size);
        for (ai_entity, _) in ai_collisions.iter() {
            let mut ai_health = match ai_query.get_mut(*ai_entity) {
                Ok(result) => result,
                Err(_) => continue,
            };

            if !ai_health.is_dead() {
                if let Some(damage) = on_impact.effect.damage {
                    ai_health.damage(damage);
                }

                for cast in on_impact.effect.casts.iter() {
                    cast.cast(impact_pos.value, impact_vel.total(), 0.0, &mut commands);
                }

                if on_impact.effect.self_destroy {
                    commands.entity(impact_entity).despawn();
                }

                break;
            }
        }
    }
}
