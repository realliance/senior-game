use bevy::prelude::*;
use bevy::reflect::TypeRegistry;
use senior_game_shared::components::assets::*;
use senior_game_shared::components::input::*;

use crate::scenes::destination_helper::Destination;

// Main Match Scene

pub fn build(target: Destination, type_registry: &Res<TypeRegistry>) -> String {
  let mut scene_world = World::new();

  if target == Destination::Client {
    // camera

    let camera = scene_world.spawn(Camera3dBundle {
      transform: Transform::from_translation(Vec3::new(-10.0, 4.0, 10.0))
        .looking_at(Vec3::default(), Vec3::unit_y()),
      ..Default::default()
    });
    scene_world
      .insert(camera, (CreatePickSource::default(),))
      .expect("Adding PickingSource failed in scene creation");
    scene_world
      .insert(camera, (BuildFlyCamera::default(),))
      .expect("Failed to add fly camera");

    scene_world.spawn(LightBundle {
      transform: Transform::from_translation(Vec3::new(0.0, 25.0, 0.0)),
      ..Default::default()
    });
  }

  scene_world.spawn((
    Transform::default(),
    GlobalTransform::default(),
    CreateAssetCollider {
      path: "models/map.gltf".to_string(),
      ..Default::default()
    },
    LoadAsset {
      path: "models/map.gltf".to_string(),
      ..Default::default()
    },
    CreatePickMesh::default(),
  ));

  let rock_trans = Transform::from_translation(Vec3::new(-10.0, 0.0, 10.0));

  scene_world.spawn((
    rock_trans,
    GlobalTransform::default(),
    CreatePhysics {
      rigidbody_transform: Transform::default(),
      rigidbody_type: RigidbodyType::Static,
      colliders: vec![CreateCollider {
        collider_transform_position: Transform::identity().translation,
        collider_transform_rotation: Transform::identity().rotation,
        collider_shape_size: Vec3::new(1.0, 1.0, 1.0),
        collider_shape: ShapeType::Cube,
      }],
    },
    LoadAsset {
      path: "models/rocksource.gltf".to_string(),
      ..Default::default()
    },
    // CreatePickMesh::default(),
  ));

  let cube_trans = Transform::from_translation(Vec3::new(10.0, 0.0, -10.0));

  scene_world.spawn((
    cube_trans,
    GlobalTransform::default(),
    CreatePhysics {
      rigidbody_transform: Transform::default(),
      rigidbody_type: RigidbodyType::Static,
      colliders: vec![CreateCollider {
        collider_transform_position: Transform::identity().translation,
        collider_transform_rotation: Transform::identity().rotation,
        collider_shape_size: Vec3::new(1.0, 1.0, 1.0),
        collider_shape: ShapeType::Cube,
      }],
    },
    LoadAsset {
      path: "models/cube.gltf".to_string(),
      ..Default::default()
    },
    CubeFollow::default(),
  ));

  let scene = DynamicScene::from_world(&scene_world, &type_registry);

  scene.serialize_ron(&type_registry).unwrap()
}
