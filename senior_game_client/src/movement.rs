use bevy::prelude::*;
use bevy_mod_picking::*;
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
    if input_map.is_action_active("MOVE") {
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
      .register_type::<NavigateTo>();
  }
}
