use bevy::prelude::*;
use bevy::render::camera::Camera;

pub fn dev_print_camera_location(query: Query<(Entity, &Transform, &Camera)>) {
  for (_, trans, _) in query.iter() {
    info!(target: "dev_print_camera_location", "{}", trans.translation);
  }
}
