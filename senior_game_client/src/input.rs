use bevy::prelude::*;
use bevy_mod_picking::*;
use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;
use kurinji::*;
use senior_game_shared::components::input::*;

pub fn input_setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
  let binding_handle: Handle<RawBinding> = asset_server.load("input_map.ron");
  commands.insert_resource(binding_handle);
  asset_server.watch_for_changes().unwrap();
}

pub fn load_input_binding(
  ev_asset: Res<Events<AssetEvent<RawBinding>>>,
  mut evr_asset: Local<EventReader<AssetEvent<RawBinding>>>,
  mut assets: ResMut<Assets<RawBinding>>,
  mut kurinji: ResMut<Kurinji>,
) {
  for ev in evr_asset.iter(&ev_asset) {
    if let AssetEvent::Created { handle } = ev {
      let RawBinding(binding) = assets.get(handle).unwrap();
      kurinji.set_bindings_with_ron(binding);
      assets.remove(handle);
    }
  }
}

pub fn input_handler(
  input_map: Res<Kurinji>,
  pick_state: Res<PickState>,
  mut query: Query<(Entity, &mut Children, &CubeFollow)>,
) {
  for (_, mut cube_trans, _) in query.iter_mut() {
    if input_map.is_action_active("SHOOT") {
      println!("hello");
      if let Some(result) = pick_state.top(Group::default()) {
        let (_, intersection) = result;
        // cube_trans.set_position(*intersection.position(), false);
        println!("{:?}", intersection.position());
      }
    }
  }
}
