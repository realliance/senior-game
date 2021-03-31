#[cfg(not(debug_assertions))]
use std::borrow::Cow;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::option::Option::Some;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_4x_camera::FourXCameraPlugin;
use bevy_egui::EguiPlugin;
use bevy_mod_picking::*;
use bevy_prototype_networking_laminar::NetworkingPlugin;
use bevy_rapier3d::physics::RapierPhysicsPlugin;
#[cfg(debug_assertions)]
use bevy_rapier3d::render::RapierRenderPlugin;
use kurinji::KurinjiPlugin;
use senior_game_shared::components::assets::*;
use senior_game_shared::components::input::*;
use senior_game_shared::net::NetworkListenerState;
use senior_game_shared::systems::game::GameSystemsPlugin;
use senior_game_shared::systems::loadscene::*;
use senior_game_shared::systems::loadsound::*;

use crate::http::HttpSystemPlugin;
use crate::input::{input_handler, input_setup, load_input_binding};
use crate::net::{handle_network_events, server_connection_system, StartServerConnection};
use crate::state::ClientState;
use crate::ui::UiSystemPlugin;

mod http;
mod input;
mod net;
mod state;
mod ui;

#[cfg(test)]
mod tests;

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
    .add_plugin(FourXCameraPlugin)
    .add_plugin(KurinjiPlugin::default())
    .add_plugin(PickingPlugin)
    .add_plugin(GameSystemsPlugin)
    .init_resource::<NetworkListenerState>()
    .add_asset::<RawBinding>()
    .init_asset_loader::<RawBindingAssetLoader>()
    .add_plugin(EguiPlugin)
    .add_plugin(UiSystemPlugin)
    .add_plugin(HttpSystemPlugin)
    .init_resource::<NetworkListenerState>()
    .init_resource::<ClientState>()
    .register_type::<CreateCollider>()
    .register_type::<CreatePhysics>()
    .register_type::<RigidbodyType>()
    .register_type::<LoadAsset>()
    .register_type::<ShapeType>()
    .register_type::<Build4xCamera>()
    .register_type::<CreateAssetCollider>()
    .register_type::<CreatePickSource>()
    .register_type::<CreatePickMesh>()
    .register_type::<CubeFollow>()
    .add_startup_system(manual_load_scene.system())
    .add_startup_system(manual_start_server_connection.system())
    .add_startup_system(input_setup.system())
    .add_startup_system(load_login_sound.system())
    .add_system(load_sound_system.system())
    .add_system(load_scene_system.system())
    .add_system(server_connection_system.system())
    .add_system(handle_network_events.system())
    .add_system(load_4x_camera.system())
    .add_system(load_asset_physics.system())
    .add_system(load_input_binding.system())
    .add_system(input_handler.system())
    .add_system_to_stage(stage::POST_UPDATE, load_asset.system())
    .add_system_to_stage(stage::POST_UPDATE, load_physics.system())
    .add_system_to_stage(stage::POST_UPDATE, load_pick_source.system())
    //.add_system_to_stage(stage::POST_UPDATE, load_pick_mesh.system())
    .run();
}

pub struct FlaggedPlugins;

impl PluginGroup for FlaggedPlugins {
  fn build(&mut self, group: &mut PluginGroupBuilder) {
    #[cfg(debug_assertions)]
    {
      let args: Vec<String> = env::args().collect();

      // Physics Debug Renderer
      if args.contains(&"--render-collider-bounds".to_string()) {
        info!(target: "app_startup", "Render Debug Activated");
        group.add(RapierRenderPlugin);
      }
    }
  }
}

// Allow as debug tool
#[allow(dead_code)]
fn manual_start_server_connection(commands: &mut Commands) {
  commands.spawn(()).with(StartServerConnection {
    addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 12350),
  });

  info!(target: "manual_start_server_connection", "Created Server Connection Component");
}

// Allow as debug tool
#[allow(dead_code)]
fn manual_load_scene(commands: &mut Commands) {
  commands.spawn(()).with(LoadScene {
    path: "scenes/game.scn".to_string(),
    watch: false,
  });

  info!(target: "manual_load_scene", "Scene Manually Loaded");
}

fn load_login_sound(commands: &mut Commands) {
  commands.spawn(()).with(LoadSound {
    path: "sounds/Komiku - A Tale is never forgotten - 01 The main reason we are here.mp3".to_string(),
    watch: false,
  });
  info!(target: "load_login_sound", "Music Loaded");
}
