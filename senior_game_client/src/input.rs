use bevy::prelude::*;
use bevy_mod_picking::*;
use kurinji::{KurinjiPlugin, *};
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

pub struct InputPlugin;

impl Plugin for InputPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_plugin(KurinjiPlugin)
    .add_plugin(PickingPlugin)
    // .add_plugin(DebugPickingPlugin)
    .add_startup_system(input_setup.system())
    .add_system(load_input_binding.system())
    .add_asset::<RawBinding>()
    .init_asset_loader::<RawBindingAssetLoader>()
    .register_type::<CreatePickSource>()
    .register_type::<CreatePickMesh>();
  }
}
