#[cfg(not(debug_assertions))]
use std::borrow::Cow::Owned;
use std::borrow::Cow;
use std::collections::HashMap;

use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy::scene::ScenePlugin;
use bevy::transform::TransformPlugin;
use bevy_prototype_networking_laminar::{Connection, NetworkDelivery, NetworkEvent, NetworkResource, NetworkingPlugin};
use bevy_rapier3d::physics::{EventQueue, RapierPhysicsPlugin};
use senior_game_shared::components::assets::*;
use senior_game_shared::net::{NetworkListenerState, NetworkMessage};
use senior_game_shared::systems::loadscene::*;

pub fn main() {
  #[cfg(not(debug_assertions))]
  {
    // Sentry Guard (pushes to sentry on drop)
    // Picks up DSN from SENTRY_DSN environment variable
    //
    // If you think you want to change this, you're probably wrong
    // It *must* be the first thing in main
    // It *cannot* be extracted into a function
    let _guard = sentry::init(sentry::ClientOptions {
      release: std::env::var("RELEASE").ok().map(Owned),
      ..Default::default()
    });

    let sdk = agones::Sdk::new().unwrap();
    // TODO: move this to a more appropriate step
    // TODO: we should check the return value of this in a less bad way
    sdk.ready().unwrap();

    // TODO: we need to do this constantly
    if sdk.health().1.is_ok() {
      debug!("Health ping sent");
    }
  }

  App::build()
    .init_resource::<State>()
    .add_plugins(MinimalPlugins)
    .add_plugin(RapierPhysicsPlugin)
    .add_plugin(NetworkingPlugin)
    .add_plugin(AssetPlugin)
    .add_plugin(ScenePlugin)
    .add_plugin(TransformPlugin)
    .init_resource::<NetworkListenerState>()
    .register_type::<CreatePhysics>()
    .register_type::<CreateCollider>()
    .register_type::<RigidbodyType>()
    .register_type::<AssetChild>()
    .register_type::<ShapeType>()
    .add_startup_system(start_server.system())
    .add_startup_system(load_game_scene.system())
    .add_system(handle_network_events.system())
    .add_system(load_scene_system.system())
    .add_system(print_events.system())
    .add_system_to_stage(stage::POST_UPDATE, load_physics.system())
    .run();
}

fn start_server(mut net: ResMut<NetworkResource>) {
  net.bind("127.0.0.1:12350").expect("Unable to bind to address");

  println!("Server Started on Port 12350");
}

fn load_game_scene(commands: &mut Commands) {
  commands.spawn(()).with(LoadScene {
    path: "scenes/platform.scn".to_string(),
    watch: false,
  });

  println!("Game Scene Loaded");
}

fn print_events(events: Res<EventQueue>) {
  while let Ok(intersection_event) = events.intersection_events.pop() {
    println!("Received intersection event: {:?}", intersection_event);
  }

  while let Ok(contact_event) = events.contact_events.pop() {
    println!("Received contact event: {:?}", contact_event);
  }
}

#[derive(Default)]
struct State {
  players: HashMap<Connection, Player>,
}

#[derive(Default)]
struct Player {}

fn handle_network_events(
  mut state: ResMut<State>,
  mut network_state: ResMut<NetworkListenerState>,
  network_events: Res<Events<NetworkEvent>>,
  net: Res<NetworkResource>,
) {
  for event in network_state.network_events.iter(&network_events) {
    match event {
      NetworkEvent::Message(conn, msg) => {
        let msg = NetworkMessage::decode(&msg[..]);
        // Temporarily allow single match since this match statement will expand soon
        #[allow(clippy::single_match)]
        match msg {
          NetworkMessage::Introduction(_) => {
            println!("Player Connected");
            state.players.insert(*conn, Player::default());
            let _ = net.send(
              conn.addr,
              &NetworkMessage::Pong.encode()[..],
              NetworkDelivery::ReliableSequenced(Some(2)),
            );
          },
          _ => {},
        }
      },
      NetworkEvent::Disconnected(conn) => {
        if state.players.remove(conn).is_some() {
          println!("Player Disconnected");
        }
      },
      _ => {},
    }
  }
}
