use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy_mod_picking::*;
use bevy_mouse_tracking_plugin::MousePos;
use bevy_rapier3d::na::{Isometry3, Vector3};
use bevy_rapier3d::physics::RigidBodyHandleComponent;
use bevy_rapier3d::rapier::dynamics::RigidBodySet;
use kurinji::*;
use senior_game_shared::components::game::*;

pub fn player(
  input_map: Res<Kurinji>,
  pick_state: Res<PickState>,
  query: Query<(Entity, &PlayerEntity)>,
  commands: &mut Commands,
) {
  for (entity, _) in query.iter() {
    if input_map.is_action_active("INTERACT") {
      if let Some(result) = pick_state.top(Group::default()) {
        let (_, intersection) = result;
        let pos = *intersection.position();
        if pos.y > 0.1 || (1. - intersection.normal().y) > 0.01 {
          continue;
        }
        commands.insert_one(
          entity,
          NavigateTo {
            x: pos.x,
            y: pos.y,
            z: pos.z,
          },
        );
        // println!("{:?}", intersection.position());
      }
    }
  }
}

pub fn camera(
  input_map: Res<Kurinji>,
  time: Res<Time>,
  mut query: Query<(&CameraRig, &mut Transform)>,
) {
  for (mut rig, mut transform) in query.iter_mut() {
    let mut direction = Vec3::zero();
    if input_map.is_action_active("CAMERA_FORWARD") {
      direction.x += 1.;
    }
    if input_map.is_action_active("CAMERA_BACK") {
      direction.x -= 1.;
    }
    if input_map.is_action_active("CAMERA_LEFT") {
      direction.z -= 1.;
    }
    if input_map.is_action_active("CAMERA_RIGHT") {
      direction.z += 1.;
    }
    if direction != Vec3::zero() {
      transform.translation +=
        direction.normalize() * time.delta_seconds() * rig.move_sensitivity * rig.zoom_mod;
    }
  }
}

pub fn camera_edges(
  mouse: Res<MousePos>,
  windows: Res<Windows>,
  mut query: Query<(&mut CameraRig, &mut Transform)>,
) {
  if let Some(window) = windows.get_primary() {
    for (mut rig, mut transform) in query.iter_mut() {
      let width = window.width();
      let height = window.height();
      if mouse.x < (width * rig.active_edge)
        || mouse.x > (width - (width * rig.active_edge))
        || mouse.y < (height * rig.active_edge)
        || mouse.y > (height - (height * rig.active_edge))
      {
        let mut x = (mouse.x - (width / 2.)) / (width / 2.);
        let mut y = (mouse.y - (height / 2.)) / (height / 2.);
        if x < 0. {
          x += 1. - rig.active_edge * 2.;
        } else {
          x -= 1. - rig.active_edge * 2.;
        }
        x /= rig.active_edge * 2.;
        if y < 0. {
          y += 1. - rig.active_edge * 2.;
        } else {
          y -= 1. - rig.active_edge * 2.;
        }
        y /= rig.active_edge * 2.;
        println!("({:?}, {:?})", x, y);
      }
    }
  }
}

pub fn camera_zoom(
  mut wheel_reader: Local<EventReader<MouseWheel>>,
  mouse_wheel_events: Res<Events<MouseWheel>>,
  mut query: Query<(&mut CameraRig, &mut Transform)>,
) {
  for (mut rig, mut transform) in query.iter_mut() {
    for event in wheel_reader.iter(&mouse_wheel_events) {
      let zoom_lvl_diff = rig.zoom_lvl - event.y;
      if zoom_lvl_diff < rig.min_zoom || zoom_lvl_diff > rig.max_zoom {
        continue;
      }
      let movement = transform.forward() * event.y * rig.zoom_sensitivity;
      transform.translation -= movement;
      rig.zoom_lvl = zoom_lvl_diff;
      rig.zoom_mod = (-event.y / 1.6) * rig.zoom_lvl / 1.8;
      if rig.zoom_mod < 1. {
        rig.zoom_mod = 1.;
      }
    }
  }
}

pub fn navigate(
  mut rigidbody_set: ResMut<RigidBodySet>,
  query: Query<(Entity, &RigidBodyHandleComponent, &NavigateTo)>,
  commands: &mut Commands,
) {
  for (entity, rigidbody_handle, dest) in query.iter() {
    let rigidbody = rigidbody_set.get_mut(rigidbody_handle.handle()).unwrap();
    rigidbody.set_position(
      rigidbody.position().lerp_slerp(
        &Isometry3::new(Vector3::new(dest.x, dest.y, dest.z), Vector3::y()),
        0.1,
      ),
      false,
    );
    // println!("dist: {:?}",
    // rigidbody.position().translation.vector.metric_distance(&Vector3::new(dest.x,
    // dest.y,dest.z)));
    if rigidbody
      .position()
      .translation
      .vector
      .metric_distance(&Vector3::new(dest.x, dest.y, dest.z))
      < 0.1
    {
      commands.remove_one::<NavigateTo>(entity);
    }
  }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_system(player.system())
      .add_system(navigate.system())
      .add_system(camera.system())
      .add_system(camera_zoom.system())
      .add_system(camera_edges.system())
      .register_type::<NavigateTo>()
      .register_type::<PlayerEntity>()
      .register_type::<CameraRig>();
  }
}
