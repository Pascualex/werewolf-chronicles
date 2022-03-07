pub use plugin::GamePlugin;

mod bundles;
mod components;
mod content;
mod plugin;
mod resources;
mod setup;
mod systems;

use setup::setup_system;

pub const TIME_STEP: f32 = 1.0 / 60.0;
