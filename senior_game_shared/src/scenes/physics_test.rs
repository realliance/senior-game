use bevy::prelude::*;
use bevy::reflect::TypeRegistry;
use senior_game_shared::components::assets::*;

pub fn build(type_registry: &Res<TypeRegistry>) -> String {
  let mut scene_world = World::new();

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

  scene_world.spawn((
    Transform::default(),
    GlobalTransform::default(),
    PhysicsBuilder {
      collider_shape: ShapeType::Cube,
      collider_shape_size: Vec3::new(1.0, 1.0, 1.0),
      collider_transform: Transform::identity(),
      rigidbody_transform: Transform::identity(),
      rigidbody_type: RigidbodyType::Dynamic,
    },
    AssetChild {
      path: "models/cube.gltf".to_string(),
    },
  ));

  // cube

  scene_world.spawn((
    Transform::default(),
    GlobalTransform::default(),
    PhysicsBuilder {
      collider_shape: ShapeType::Cube,
      collider_shape_size: Vec3::new(1.0, 1.0, 1.0),
      collider_transform: Transform::identity(),
      rigidbody_transform: Transform::from_translation(Vec3::new(-1.75, -5.0, 0.0)),
      rigidbody_type: RigidbodyType::Static,
    },
    AssetChild {
      path: "models/cube.gltf".to_string(),
    },
  ));

  let scene = DynamicScene::from_world(&scene_world, &type_registry);

  return scene.serialize_ron(&type_registry).unwrap();
}
