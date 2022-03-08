use bevy::{core::FixedTimestep, prelude::*};

use super::{components::*, resources::*, setup_system, systems::*, TIME_STEP};

#[derive(Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(ai_movement_system)
                    .with_system(bullet_system.after("collisions"))
                    .with_system(collision_grid_system.label("collisions"))
                    .with_system(lifetime_system)
                    .with_system(movement_system)
                    .with_system(player_movement_system)
                    .with_system(spawner_system)
                    .with_system(turret_system),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(diagnostics_system),
            )
            .insert_resource(CollisionGrid::<Ai>::new(30.0))
            .insert_resource(Spawner::new(10_000.0));
    }
}
