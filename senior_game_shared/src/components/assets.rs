use bevy::prelude::*;
use bevy::asset::HandleId;
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

#[derive(Clone, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct CreatePhysics {
  pub rigidbody_type: RigidbodyType,
  pub rigidbody_transform: Transform,
  pub colliders: Vec<CreateCollider>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect_value(PartialEq, Serialize, Deserialize)]
pub struct CreateCollider {
  pub collider_transform_position: Vec3,
  pub collider_transform_rotation: Quat,
  pub collider_shape_size: Vec3,
  pub collider_shape: ShapeType,
}

#[derive(Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct CreateAssetCollider {
  pub path: String,
  pub loading_started: bool,
  pub handle_id: HandleId,
}

impl Default for CreateAssetCollider {
  fn default() -> CreateAssetCollider {
    CreateAssetCollider {
      path: String::default(),
      loading_started: bool::default(),
      handle_id: HandleId::default::<Mesh>(),
    }
  }
}

pub struct LoadScene {
  pub path: String,
  pub watch: bool,
}

pub struct LoadSound {
  pub path: String,
  pub watch: bool,
}
#[derive(Clone, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct BuildFlyCamera;
