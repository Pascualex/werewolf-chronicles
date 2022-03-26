use crate::game::content::Cast;

pub struct Caster {
    pub cast_rate: f32,
    pub cast: Cast,
}

impl Caster {
    pub fn new(cast_rate: f32, cast: Cast) -> Self {
        Self { cast_rate, cast }
    }
}
