use bevy::prelude::*;
use mockito::{mock, Mock};
use serde::{Deserialize, Serialize};
use serde_json::to_string;

mod http;
mod login;

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

#[derive(Serialize, Deserialize, Clone)]
struct TestJSONResponse {
  token: String,
}

fn create_test_endpoint(
  method: &str,
  path: &str,
  response: usize,
  body: Option<&TestJSONResponse>,
) -> Mock {
  let mut m = mock(method, path).with_status(response);

  if let Some(b) = body {
    m = m
      .with_header("content-type", "application/json; charset=utf-8")
      .with_body(to_string(&b).unwrap());
  }

  return m.create();
}
