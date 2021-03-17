use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
#[reflect(PartialEq, Serialize, Deserialize)]
pub enum SourceType {
  None = 0,
  Red = 1,
  Blue = 2,
  Green = 3,
}

impl SourceType {
  pub fn get_path(&self) -> Option<&str> {
    match self {
      SourceType::None => None,
      SourceType::Red => Some("models/redcrystal.gltf"),
      SourceType::Blue => Some("models/bluemanasource.gltf"),
      SourceType::Green => Some("models/greencrystal.gltf"),
    }
  }
}

impl Default for SourceType {
  fn default() -> SourceType {
    SourceType::None
  }
}

#[derive(Clone, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct BuildSourceModel;

#[derive(Clone, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct ManaSource {
  id: u8,
  source_type: SourceType,
  max_capacity: u32,
  current_capacity: u32,
}

impl ManaSource {
  pub fn new(id: u8, capacity: u32, source_type: SourceType) -> ManaSource {
    ManaSource {
      id,
      source_type,
      max_capacity: capacity,
      current_capacity: capacity,
    }
  }

  pub fn id(&self) -> u8 {
    self.id
  }

  pub fn source_type(&self) -> SourceType {
    self.source_type
  }

  pub fn capacity(&self) -> u32 {
    self.max_capacity
  }

  pub fn remaining_mana(&self) -> u32 {
    self.current_capacity
  }

  pub fn set_remaining_mana(&mut self, amount: u32) {
    self.current_capacity = amount.clamp(0, self.max_capacity);
  }

  pub fn is_empty(&self) -> bool {
    self.current_capacity == 0
  }

  pub fn set_source_type(&mut self, source_type: SourceType) {
    self.source_type = source_type;
  }
}
