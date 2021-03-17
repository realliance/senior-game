use bevy::prelude::*;
use bevy::reflect::TypeRegistry;
use senior_game_shared::components::assets::*;
use senior_game_shared::components::game::*;
use senior_game_shared::components::input::*;

use crate::scenes::destination_helper::Destination;

// Main Match Scene

fn build_rock(world: &mut World, id: u8, capacity: u32, source_type: SourceType, trans: Transform) {
  world.spawn((
    Transform::default(),
    GlobalTransform::default(),
    CreatePhysics {
      rigidbody_transform: trans,
      rigidbody_type: RigidbodyType::Static,
      colliders: vec![CreateCollider {
        collider_transform_position: Vec3::new(0.0, 0.75, 0.0),
        collider_transform_rotation: Transform::identity().rotation,
        collider_shape_size: Vec3::new(1.2, 1.5, 1.2),
        collider_shape: ShapeType::Cube,
      }],
    },
    BuildSourceModel::default(),
    ManaSource::new(id, capacity, source_type),
    LoadAsset {
      path: "models/rockbase.gltf".to_string(),
      ..Default::default()
    },
  ));
}

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

  const ROCK_Y: f32 = 0.5;
  const SOURCE_CAPACITY: u32 = 25000;

  let mirrored_rock_locations = [
    Vec3::new(30.0, ROCK_Y, -112.0),
    Vec3::new(-10.0, ROCK_Y, -108.0),
    Vec3::new(-69.0, ROCK_Y, -74.0),
    Vec3::new(91.0, ROCK_Y, -85.0),
    Vec3::new(8.0, ROCK_Y, -8.0),
    Vec3::new(-8.0, ROCK_Y, -8.0),
  ];

  let mut i = 0;

  for location in mirrored_rock_locations.iter() {
    let rock_trans = Transform::from_translation(*location);
    let mirrored_rock_trans = Transform::from_translation(*location * Vec3::new(1.0, 1.0, -1.0));

    build_rock(
      &mut scene_world,
      i,
      SOURCE_CAPACITY,
      SourceType::Blue,
      rock_trans,
    );
    i += 1;

    build_rock(
      &mut scene_world,
      i,
      SOURCE_CAPACITY,
      SourceType::Red,
      mirrored_rock_trans,
    );
    i += 1;
  }

  build_rock(
    &mut scene_world,
    i,
    SOURCE_CAPACITY,
    SourceType::Green,
    Transform::from_translation(Vec3::new(0.0, ROCK_Y, 0.0)),
  );

  let scene = DynamicScene::from_world(&scene_world, &type_registry);

  scene.serialize_ron(&type_registry).unwrap()
}
