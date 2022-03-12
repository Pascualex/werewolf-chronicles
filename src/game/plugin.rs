use bevy::{core::FixedTimestep, prelude::*, transform::TransformSystem};

use super::{components::*, resources::*, setup_system, systems::*};

pub const TIME_STEP_ID: &str = "game_time_step";
pub const TIME_STEP: f32 = 1.0 / 50.0;

#[derive(Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system)
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new()
                    .with_run_criteria(
                        FixedTimestep::step(TIME_STEP as f64).with_label(TIME_STEP_ID),
                    )
                    .with_system(velocity_system),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(ai_movement_system)
                    .with_system(bullet_system.label("collisions"))
                    .with_system(collision_grid_system.before("collisions"))
                    .with_system(lifetime_system)
                    .with_system(player_movement_system)
                    .with_system(spawner_system)
                    .with_system(turret_system),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                transform_system.before(TransformSystem::ParentUpdate),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(diagnostics_system),
            )
            .insert_resource(CollisionGrid::<Ai>::new(50.0))
            .insert_resource(Spawner::new(100.0));
    }
}
