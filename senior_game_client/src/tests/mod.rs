use bevy::prelude::*;

mod http;

// https://github.com/bevyengine/bevy/blob/v0.4.0/crates/bevy_ecs/src/system/into_system.rs
fn run_system<S: System<In = (), Out = ()>>(
  world: &mut World,
  resources: &mut Resources,
  system: S,
) {
  let mut schedule = Schedule::default();
  let mut update = SystemStage::serial();
  update.add_system(system);
  schedule.add_stage("update", update);
  schedule.initialize_and_run(world, resources);
}

