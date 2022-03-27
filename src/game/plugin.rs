use bevy::{core::FixedTimestep, prelude::*};

use crate::game::{components::*, resources::*, setup_system, systems::*};

pub const TIME_STEP_ID: &str = "game_time_step";
pub const TIME_STEP: f32 = 1.0 / 50.0;

#[derive(Default)]
pub struct GamePlugin;

#[derive(StageLabel, Clone, PartialEq, Eq, Hash, Debug)]
enum CustomStage {
    Render,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            CoreStage::Update,
            CustomStage::Render,
            SystemStage::parallel(),
        )
        .add_startup_system(setup_system)
        .add_system_set(
            SystemSet::new()
                .label("velocity")
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64).with_label(TIME_STEP_ID))
                .with_system(velocity_system),
        )
        .add_system_set(
            SystemSet::new()
                .after("velocity")
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(overlap_system.after("collisions"))
                .with_system(ai_movement_system)
                .with_system(caster_system)
                .with_system(collision_grid_system.label("collisions"))
                .with_system(death_system.label("death"))
                .with_system(impact_system.after("collisions").before("death"))
                .with_system(lifetime_system)
                .with_system(player_movement_system)
                .with_system(spawner_system),
        )
        .add_system_set_to_stage(
            CustomStage::Render,
            SystemSet::new()
                .with_system(transform_system.label("transform"))
                .with_system(camera_system.after("transform")),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(diagnostics_system),
        )
        .insert_resource(CollisionGrid::<Ai>::new(50.0))
        .insert_resource(Spawner::new(1000.0));
    }
}
