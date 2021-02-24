#[cfg(debug_assertions)]
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_prototype_networking_laminar::NetworkingPlugin;
use bevy_rapier3d::physics::RapierPhysicsPlugin;
#[cfg(debug_assertions)]
use bevy_rapier3d::render::RapierRenderPlugin;
use senior_game_shared::components::assets::*;
use senior_game_shared::net::NetworkListenerState;
use senior_game_shared::systems::loadscene::*;

use crate::net::{handle_network_events, server_connection_system, StartServerConnection};

mod net;

fn main() {
  App::build()
    .add_resource(Msaa::default())
    .add_plugins(DefaultPlugins)
    .add_plugins(FlaggedPlugins)
    .add_plugin(RapierPhysicsPlugin)
    .add_plugin(NetworkingPlugin)
    .init_resource::<NetworkListenerState>()
    .register_type::<CreateCollider>()
    .register_type::<CreatePhysics>()
    .register_type::<RigidbodyType>()
    .register_type::<AssetChild>()
    .register_type::<ShapeType>()
    .add_startup_system(manual_load_scene.system())
    .add_startup_system(manual_start_server_connection.system())
    .add_system(load_scene_system.system())
    .add_system(server_connection_system.system())
    .add_system(handle_network_events.system())
    .add_system_to_stage(stage::POST_UPDATE, load_asset.system())
    .add_system_to_stage(stage::POST_UPDATE, load_physics.system())
    .run();
}

pub struct FlaggedPlugins;

impl PluginGroup for FlaggedPlugins {
  fn build(&mut self, group: &mut PluginGroupBuilder) {
    let args: Vec<String> = env::args().collect();
    // Debug Build Only Flags
    if cfg!(debug_assertions) {
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

  info!(target: "manual_start_server_connection", "Crated Server Connection Component");
}

fn manual_load_scene(commands: &mut Commands) {
  commands.spawn(()).with(LoadScene {
    path: "scenes/physics_test.scn".to_string(),
    watch: false,
  });

  info!(target: "manual_load_scene", "Scene Manually Loaded");
}
