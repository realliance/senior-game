#[cfg(not(debug_assertions))]
use std::borrow::Cow;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::option::Option::Some;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_fly_camera::FlyCameraPlugin;
use bevy_mod_picking::*;
use bevy_prototype_networking_laminar::NetworkingPlugin;
use bevy_rapier3d::physics::RapierPhysicsPlugin;
#[cfg(debug_assertions)]
use bevy_rapier3d::render::RapierRenderPlugin;
use kurinji::KurinjiPlugin;
use senior_game_shared::components::assets::*;
use senior_game_shared::components::input::*;
use senior_game_shared::net::NetworkListenerState;
use senior_game_shared::systems::loadscene::*;

use crate::input::{input_handler, input_setup, load_input_binding};
use crate::net::{handle_network_events, server_connection_system, StartServerConnection};

mod input;
mod net;

fn main() {
  #[cfg(not(debug_assertions))]
  {
    // Sentry Guard (pushes to sentry on drop)
    // Picks up DSN from SENTRY_DSN environment variable
    //
    // If you think you want to change this, you're probably wrong
    // It *must* be the first thing in main
    // It *cannot* be extracted into a function
    // Yes, embedding the DSN is intentional
    let _guard = sentry::init((
      env!("SENTRY_DSN"),
      sentry::ClientOptions {
        release: Some(Cow::Borrowed(env!("RELEASE"))),
        ..Default::default()
      },
    ));
  }

  App::build()
    .add_resource(Msaa::default())
    .add_plugins(DefaultPlugins)
    .add_plugins(FlaggedPlugins)
    .add_plugin(RapierPhysicsPlugin)
    .add_plugin(NetworkingPlugin)
    .add_plugin(FlyCameraPlugin)
    .add_plugin(KurinjiPlugin::default())
    .add_plugin(PickingPlugin)
    .add_plugin(InteractablePickingPlugin)
    .init_resource::<NetworkListenerState>()
    .add_asset::<RawBinding>()
    .init_asset_loader::<RawBindingAssetLoader>()
    .register_type::<CreateCollider>()
    .register_type::<CreatePhysics>()
    .register_type::<RigidbodyType>()
    .register_type::<AssetChild>()
    .register_type::<ShapeType>()
    .register_type::<BuildFlyCamera>()
    .register_type::<CreateAssetCollider>()
    .register_type::<CreatePickSource>()
    .register_type::<CubeFollow>()
    .add_startup_system(manual_load_scene.system())
    .add_startup_system(manual_start_server_connection.system())
    .add_startup_system(input_setup.system())
    .add_system(load_scene_system.system())
    .add_system(server_connection_system.system())
    .add_system(handle_network_events.system())
    .add_system(load_fly_camera.system())
    .add_system(load_asset_physics.system())
    .add_system(load_input_binding.system())
    .add_system(input_handler.system())
    .add_system_to_stage(stage::POST_UPDATE, load_asset.system())
    .add_system_to_stage(stage::POST_UPDATE, load_physics.system())
    .add_system_to_stage(stage::POST_UPDATE, load_pick_source.system())
    .add_system_to_stage(stage::POST_UPDATE, load_pick_mesh.system())
    .run();
}

pub struct FlaggedPlugins;

impl PluginGroup for FlaggedPlugins {
  fn build(&mut self, group: &mut PluginGroupBuilder) {
    let args: Vec<String> = env::args().collect();

    #[cfg(debug_assertions)]
    {
      // Physics Debug Renderer
      if args.contains(&"--render-collider-bounds".to_string()) {
        info!(target: "app_startup", "Render Debug Activated");
        group.add(RapierRenderPlugin);
      }
    }
  }
}

fn manual_start_server_connection(commands: &mut Commands) {
  commands.spawn(()).with(StartServerConnection {
    addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 12350),
  });

  info!(target: "manual_start_server_connection", "Created Server Connection Component");
}

fn manual_load_scene(commands: &mut Commands) {
  commands.spawn(()).with(LoadScene {
    path: "scenes/game.scn".to_string(),
    watch: false,
  });

  info!(target: "manual_load_scene", "Scene Manually Loaded");
}
