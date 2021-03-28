use bevy::prelude::*;
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
