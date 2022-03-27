use bevy::prelude::*;

use crate::game::{components::*, content::*};

pub fn setup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let mini_bullet = Ability {
        size: Vec2::new(10.0, 10.0),
        speed: 1200.0,
        lifetime: 3.0,
        color: Color::rgb(0.1, 0.9, 0.1),
        on_impact: Some(OnImpact::new_damage(1, true)),
    };

    let bullet = Ability {
        size: Vec2::new(15.0, 15.0),
        speed: 1200.0,
        lifetime: 3.0,
        color: Color::rgb(0.9, 0.1, 0.1),
        on_impact: Some(OnImpact {
            self_destroy: true,
            damage: Some(1),
            casts: vec![Cast::new_multi(3, 30.0, mini_bullet)],
        }),
    };

    commands
        .spawn()
        .insert(Player)
        .insert_bundle(creatures::hero(Vec2::ZERO))
        .insert(CasterStats {
            cast_rate_multiplier: 50.0,
            cast_modifier: CastModifier { extra_casts: 9 },
            ability_modifier: AbilityModifier { extra_damage: 1 },
        })
        .insert(Casters::new(vec![Caster::new(
            1.0,
            Cast::new_multi(1, 30.0, bullet),
        )]));
}
