use std::f32;

use bevy::prelude::*;
use bevy::reflect::TypeRegistry;
use bevy::render::camera::PerspectiveProjection;
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
    CreatePickMesh::default(),
  ));
}

pub fn build(target: Destination, type_registry: &Res<TypeRegistry>) -> String {
  let mut scene_world = World::new();

  if target == Destination::Client {
    // camera

    let camera = scene_world.spawn(Camera3dBundle {
      perspective_projection: PerspectiveProjection {
        fov: 0.5,
        ..Default::default()
      },
      transform: Transform::from_translation(Vec3::new(-20.0, 30., 0.0))
        .looking_at(Vec3::zero(), Vec3::unit_y()),
      ..Default::default()
    });
    scene_world
      .insert(camera, (CreatePickSource::default(),))
      .expect("Adding PickingSource failed in scene creation");

    scene_world
      .insert(
        camera,
        (CameraRig {
          move_sensitivity: 67.,
          zoom_sensitivity: 7.,
          active_edge: 0.1,
          min_zoom: 1.,
          max_zoom: 16.,
          zoom_lvl: 1.,
          zoom_mod: 1.,
        },),
      )
      .expect("Adding CameraRig failed in scene creation");

    scene_world.spawn(LightBundle {
      transform: Transform::from_translation(Vec3::new(0.0, 25.0, 0.0)),
      ..Default::default()
    });
  }

  let theta = 45.0 * (f32::consts::PI / 180.0);

  scene_world.spawn((
    Transform::from_rotation(Quat::from_rotation_y(theta)),
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
    Vec3::new(-57.98, ROCK_Y, -100.41),
    Vec3::new(-83.44, ROCK_Y, -69.30),
    Vec3::new(-101.12, ROCK_Y, -3.54),
    Vec3::new(4.24, ROCK_Y, -124.45),
    Vec3::new(8.88, ROCK_Y, -1.13),
    Vec3::new(-8.88, ROCK_Y, 1.13),
  ];

  let mut i = 0;

  for location in mirrored_rock_locations.iter() {
    let rock_trans = Transform::from_translation(*location);

    let reflected = Transform::from_translation(
      Quat::from_rotation_y(-theta).mul_vec3(*location) * Vec3::new(1.0, 1.0, -1.0),
    );
    let mirrored_rock_trans =
      Transform::from_translation(Quat::from_rotation_y(theta).mul_vec3(reflected.translation));

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

  scene_world.spawn((
    Transform::default(),
    GlobalTransform::default(),
    CreatePhysics {
      rigidbody_transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
      rigidbody_type: RigidbodyType::Static,
      colliders: vec![CreateCollider {
        collider_transform_position: Vec3::new(0.0, 0.0, 0.0),
        collider_transform_rotation: Transform::identity().rotation,
        collider_shape_size: Vec3::new(1.0, 1.0, 1.0),
        collider_shape: ShapeType::Cube,
      }],
    },
    BuildSourceModel::default(),
    LoadAsset {
      path: "models/cube.gltf".to_string(),
      ..Default::default()
    },
    PlayerEntity::default(),
  ));

  let scene = DynamicScene::from_world(&scene_world, &type_registry);

  scene.serialize_ron(&type_registry).unwrap()
}
