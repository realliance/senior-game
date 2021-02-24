use std::path::Path;

use bevy::asset::{AssetServer, Handle};
use bevy::log::*;
use bevy::ecs::{Commands, Entity, Query, Res, ResMut};
use bevy::prelude::BuildChildren;
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
    info!(target: "load_scene_system", "Load Scene Triggered: {}", &load_scene.path);

    let scene_handle: Handle<DynamicScene> = asset_server.load(Path::new(&load_scene.path));
    scene_spawner.spawn_dynamic(scene_handle);

    if load_scene.watch {
      asset_server.watch_for_changes().unwrap();
    }

    commands.despawn(entity);
  }
}

pub fn load_physics(query: Query<(Entity, &CreatePhysics)>, commands: &mut Commands) {
  for (entity, bundle) in query.iter() {
    info!(target: "load_physics", "Load Rigidbody Triggered");
    let trans = bundle.rigidbody_transform.translation;

    let rigidbody = match bundle.rigidbody_type {
      RigidbodyType::Dynamic => RigidBodyBuilder::new_dynamic(),
      RigidbodyType::Static => RigidBodyBuilder::new_static(),
      RigidbodyType::Kinematic => RigidBodyBuilder::new_kinematic(),
    }
    .translation(trans.x, trans.y, trans.z);

    commands.insert(entity, (rigidbody,));

    for c in bundle.colliders.iter() {
      info!(target: "load_physics", "Load Collider Triggered");

      let collider = match c.collider_shape {
        ShapeType::Cube => ColliderBuilder::cuboid(
          c.collider_shape_size.x,
          c.collider_shape_size.y,
          c.collider_shape_size.z,
        ),
        ShapeType::Ball => ColliderBuilder::ball(c.collider_shape_size.x),
      }
      .translation(
        c.collider_transform_position.x,
        c.collider_transform_position.y,
        c.collider_transform_position.z,
      );

      let child = commands.spawn((collider,)).current_entity().unwrap();

      commands.push_children(entity, &[child]);
    }
    commands.remove_one::<CreatePhysics>(entity);
  }
}

pub fn load_asset(
  query: Query<(Entity, &AssetChild)>,
  commands: &mut Commands,
  asset_server: ResMut<AssetServer>,
  mut scene_spawner: ResMut<SceneSpawner>,
) {
  for (entity, asset) in query.iter() {
    info!(target: "load_asset", "Load Asset Triggered");
    scene_spawner.spawn_as_child(asset_server.load(Path::new(&asset.path)), entity);

    commands.remove_one::<AssetChild>(entity);
  }
}
