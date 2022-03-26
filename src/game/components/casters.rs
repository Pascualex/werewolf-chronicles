use bevy::prelude::*;

use crate::game::{content::Caster, utils::VariableTimer};

#[derive(Component)]
pub struct Casters {
    pub casters: Vec<(Caster, VariableTimer)>,
}

impl Casters {
    pub fn new(casters: Vec<Caster>) -> Self {
        Self {
            casters: casters
                .into_iter()
                .map(|c| (c, VariableTimer::new()))
                .collect(),
        }
    }
}
