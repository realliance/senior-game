use bevy::prelude::*;
use bevy_mod_picking::*;
use bevy_rapier3d::na::{Isometry3, Vector3};
use bevy_rapier3d::physics::RigidBodyHandleComponent;
use bevy_rapier3d::rapier::dynamics::RigidBodySet;
use kurinji::*;
use senior_game_shared::components::input::*;

pub fn player(
  input_map: Res<Kurinji>,
  pick_state: Res<PickState>,
  mut rigidbody_set: ResMut<RigidBodySet>,
  query: Query<(Entity, &RigidBodyHandleComponent, &CubeFollow)>,
) {
  for (_, rigidbody_handle, _) in query.iter() {
    if input_map.is_action_active("SHOOT") {
      if let Some(result) = pick_state.top(Group::default()) {
        let (_, intersection) = result;
        let rigidbody = rigidbody_set.get_mut(rigidbody_handle.handle()).unwrap();
        let pos = *intersection.position();
        if pos.y > 0.1 || (1. - intersection.normal().y) > 0.01 {
          continue;
        }
        rigidbody.set_position(
          Isometry3::new(Vector3::new(pos.x, pos.y, pos.z), Vector3::y()),
          false,
        );
        // println!("{:?}", intersection.position());
      }
    }
  }
}
