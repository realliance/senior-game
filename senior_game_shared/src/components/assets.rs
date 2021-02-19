use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Hash, PartialEq, Eq, Clone, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct AssetChild {
  pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
#[reflect(PartialEq, Serialize, Deserialize)]
pub enum RigidbodyType {
  Dynamic,
  Static,
  Kinematic,
}

impl Default for RigidbodyType {
  fn default() -> RigidbodyType {
    RigidbodyType::Static
  }
}

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize, Reflect)]
#[reflect(PartialEq, Serialize, Deserialize)]
pub enum ShapeType {
  Cube,
  Ball,
}

impl Default for ShapeType {
  fn default() -> ShapeType {
    ShapeType::Cube
  }
}

#[derive(Clone, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct PhysicsBuilder {
  pub rigidbody_type: RigidbodyType,
  pub rigidbody_transform: Transform,
  pub collider_transform: Transform,
  pub collider_shape_size: Vec3,
  pub collider_shape: ShapeType,
}

pub struct LoadScene {
  pub path: String,
  pub watch: bool,
}
