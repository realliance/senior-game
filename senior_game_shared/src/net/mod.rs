use bevy::prelude::*;
use bevy_prototype_networking_laminar::NetworkEvent;
use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum NetworkMessage {
  Introduction(String),

  // Previous Message Recieved
  Pong,
}

impl NetworkMessage {
  pub fn encode(&self) -> Vec<u8> {
    serialize(&self).unwrap()
  }

  pub fn decode(bytes: &[u8]) -> NetworkMessage {
    deserialize(bytes).unwrap()
  }
}

#[derive(Default)]
pub struct NetworkListenerState {
  pub network_events: EventReader<NetworkEvent>,
}
