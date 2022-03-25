use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Handler<E> {
    pub effect: E,
}

impl<E> Handler<E> {
    pub fn new(effect: E) -> Self {
        Self { effect }
    }
}
