use std::path::Path;

use bevy::asset::{AssetServer, Handle};
use bevy::ecs::{Commands, Entity, Query, Res, ResMut};
use bevy::scene::{DynamicScene, SceneSpawner};
use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier3d::rapier::geometry::ColliderBuilder;

use crate::components::assets::*;

pub fn load_scene_system(
  commands: &mut Commands,
  asset_server: Res<AssetServer>,
  mut scene_spawner: ResMut<SceneSpawner>,
  query: Query<(Entity, &LoadScene)>,
) {
  for (entity, load_scene) in query.iter() {
    println!("Load Scene Triggered");
    println!("{}", &load_scene.path);

    let scene_handle: Handle<DynamicScene> = asset_server.load(Path::new(&load_scene.path));
    scene_spawner.spawn_dynamic(scene_handle);

    if load_scene.watch {
      asset_server.watch_for_changes().unwrap();
    }

    commands.despawn(entity);
  }
}

pub fn load_physics(query: Query<(Entity, &PhysicsBuilder)>, commands: &mut Commands) {
  for (entity, bundle) in query.iter() {
    println!("Load Physics Triggered");
    let trans = bundle.rigidbody_transform.translation;

    let rigidbody = match bundle.rigidbody_type {
      RigidbodyType::Dynamic => RigidBodyBuilder::new_dynamic(),
      RigidbodyType::Static => RigidBodyBuilder::new_static(),
      RigidbodyType::Kinematic => RigidBodyBuilder::new_kinematic(),
    }
    .translation(trans.x, trans.y, trans.z);

    let collider = match bundle.collider_shape {
      ShapeType::Cube => ColliderBuilder::cuboid(
        bundle.collider_shape_size.x,
        bundle.collider_shape_size.y,
        bundle.collider_shape_size.z,
      ),
      ShapeType::Ball => ColliderBuilder::ball(bundle.collider_shape_size.x),
    };

    commands.insert(entity, (rigidbody, collider));

    commands.remove_one::<PhysicsBuilder>(entity);
  }
}

pub fn load_asset(
  query: Query<(Entity, &AssetChild)>,
  commands: &mut Commands,
  asset_server: ResMut<AssetServer>,
  mut scene_spawner: ResMut<SceneSpawner>,
) {
  for (entity, asset) in query.iter() {
    println!("Load Asset Triggered");
    scene_spawner.spawn_as_child(asset_server.load(Path::new(&asset.path)), entity);

    commands.remove_one::<AssetChild>(entity);
  }
}
