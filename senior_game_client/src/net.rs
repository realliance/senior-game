use std::net::SocketAddr;

use bevy::prelude::*;
use bevy_prototype_networking_laminar::{NetworkDelivery, NetworkEvent, NetworkResource};
use senior_game_shared::net::{NetworkListenerState, NetworkMessage};

#[derive(Debug)]
pub struct StartServerConnection {
  pub addr: SocketAddr,
}

pub fn server_connection_system(
  commands: &mut Commands,
  mut net: ResMut<NetworkResource>,
  query: Query<(Entity, &StartServerConnection)>,
) {
  for (entity, conn_info) in query.iter() {
    println!("Connecting to {}", conn_info.addr.to_string());

    net
      .bind("127.0.0.1:12351")
      .expect("Failed to bind to socket");

    net
      .send(
        conn_info.addr,
        &NetworkMessage::Introduction("Hello Server!".to_string()).encode()[..],
        NetworkDelivery::ReliableSequenced(Some(1)),
      )
      .expect("Failed to send intro message");

    commands.despawn(entity);
  }
}

pub fn handle_network_events(
  mut state: ResMut<NetworkListenerState>,
  network_events: Res<Events<NetworkEvent>>,
) {
  for event in state.network_events.iter(&network_events) {
    match event {
      NetworkEvent::Message(_conn, msg) => {
        let msg = NetworkMessage::decode(&msg[..]);
        match msg {
          NetworkMessage::Pong => {
            println!("Pong Recieved");
          },
          _ => {},
        }
      },
      _ => {},
    }
  }
}
