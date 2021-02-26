use bevy::prelude::*;
use bevy::reflect::TypeRegistry;
use senior_game_shared::components::assets::*;

use crate::scenes::destination_helper::Destination;

pub fn build(target: Destination, type_registry: &Res<TypeRegistry>) -> String {
  let mut scene_world = World::new();

  if target == Destination::Client {
    // light
    scene_world.spawn(LightBundle {
      transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
      ..Default::default()
    });

    // camera
    scene_world.spawn(Camera3dBundle {
      transform: Transform::from_translation(Vec3::new(-10.0, 4.0, 10.0))
        .looking_at(Vec3::default(), Vec3::unit_y()),
      ..Default::default()
    });
  }

  scene_world.spawn((
    Transform::default(),
    GlobalTransform::default(),
    CreatePhysics {
      rigidbody_transform: Transform::identity(),
      rigidbody_type: RigidbodyType::Dynamic,
      colliders: vec![CreateCollider {
        collider_transform_position: Transform::identity().translation,
        collider_transform_rotation: Transform::identity().rotation,
        collider_shape_size: Vec3::new(1.0, 1.0, 1.0),
        collider_shape: ShapeType::Cube,
      }],
    },
    AssetChild {
      path: "models/cube.gltf".to_string(),
    },
  ));

  // cube

  scene_world.spawn((
    Transform::default(),
    GlobalTransform::default(),
    CreatePhysics {
      rigidbody_transform: Transform::from_translation(Vec3::new(-1.75, -5.0, 0.0)),
      rigidbody_type: RigidbodyType::Static,
      colliders: vec![CreateCollider {
        collider_transform_position: Transform::identity().translation,
        collider_transform_rotation: Transform::identity().rotation,
        collider_shape_size: Vec3::new(1.0, 1.0, 1.0),
        collider_shape: ShapeType::Cube,
      }],
    },
    AssetChild {
      path: "models/cube.gltf".to_string(),
    },
  ));

  let scene = DynamicScene::from_world(&scene_world, &type_registry);

  scene.serialize_ron(&type_registry).unwrap()
}
