pub use plugin::GamePlugin;

mod bundles;
mod components;
mod content;
mod plugin;
mod resources;
mod setup;
mod systems;
mod utils;

use plugin::{TIME_STEP, TIME_STEP_ID};
use setup::setup_system;
