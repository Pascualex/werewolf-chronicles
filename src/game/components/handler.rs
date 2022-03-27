use bevy::prelude::*;

use crate::game::content::AbilityModifier;

#[derive(Component)]
pub struct Handler<E> {
    pub effect: E,
    pub ability_modifier: Option<AbilityModifier>,
}

impl<E> Handler<E> {
    pub fn new(effect: E, ability_modifier: Option<AbilityModifier>) -> Self {
        Self {
            effect,
            ability_modifier,
        }
    }
}
