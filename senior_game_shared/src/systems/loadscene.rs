use std::path::Path;

use bevy::asset::{AssetServer, Handle, HandleId};
use bevy::ecs::{Commands, Entity, Query, Res, ResMut};
use bevy::prelude::{BuildChildren, *};
use bevy::render::mesh::{Indices, VertexAttributeValues};
use bevy::scene::{DynamicScene, SceneSpawner};
use bevy_mod_picking::*;
use bevy_rapier3d::na::Point3;
use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier3d::rapier::geometry::ColliderBuilder;

use crate::components::assets::*;
use crate::components::input::*;

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

pub fn load_physics(commands: &mut Commands, physics_query: Query<(Entity, &CreatePhysics)>) {
  for (entity, bundle) in physics_query.iter() {
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

pub fn load_asset_physics(
  commands: &mut Commands,
  asset_server: Res<AssetServer>,
  meshes: Res<Assets<Mesh>>,
  mut query: Query<(Entity, &mut CreateAssetCollider)>,
) {
  for (entity, mut asset) in query.iter_mut() {
    let path = Path::new(&asset.path);

    if !asset.loading_started {
      info!(target: "load_asset_physics", "Load Asset Physics Triggered {}", asset.path);
      let handle: Handle<Mesh> = asset_server.load(path);
      asset.handle_id = handle.id;
      asset.loading_started = true;
    }

    if let HandleId::AssetPathId(asset_path_id) = asset.handle_id {
      let source_path_id = asset_path_id.source_path_id();

      for (id, mesh) in meshes.iter() {
        if let HandleId::AssetPathId(mesh_asset) = id {
          if mesh_asset.source_path_id() == source_path_id {
            let collider = create_collider_for_mesh(mesh);
            let child = commands
              .spawn((RigidBodyBuilder::new_static(), collider))
              .current_entity()
              .unwrap();
            commands.push_children(entity, &[child]);
            commands.remove_one::<CreateAssetCollider>(entity);
            info!(target: "load_asset_physics", "Load Asset Physics Finished {}", asset.path);
            break;
          }
        }
      }
    } else {
      panic!("Attempted to load asset but found a uuid asset!");
    }
  }
}

fn create_collider_for_mesh(mesh: &Mesh) -> ColliderBuilder {
  let verts = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();

  let verts: &Vec<[f32; 3]> = match verts {
    VertexAttributeValues::Float3(vert) => Some(vert),
    _ => None,
  }
  .unwrap();

  let verts: Vec<Point3<f32>> = verts
    .iter()
    .map(|vert| Point3::new(vert[0], vert[1], vert[2]))
    .collect();

  let indices: Vec<[u32; 3]> = match mesh.indices().unwrap() {
    Indices::U32(i) => Some(i),
    _ => None,
  }
  .unwrap()
  .chunks(3)
  .map(|tri| [tri[0], tri[1], tri[2]])
  .collect();

  ColliderBuilder::trimesh(verts.clone(), indices.clone())
}

pub fn load_pick_source(query: Query<(Entity, &CreatePickSource)>, commands: &mut Commands) {
  for (entity, _) in query.iter() {
    info!(target: "load_pick_source", "Load PickSource Triggered");
    commands.insert(entity, (PickSource::default(),));
    commands.remove_one::<CreatePickSource>(entity);
  }
}

// TODO: Only make tagged entities pickable
pub fn load_pick_mesh(query: Query<(Entity, &Handle<Mesh>, &CreatePickMesh)>, commands: &mut Commands) {
  for (entity, handle, _) in query.iter() {
    info!(target: "load_pick_mesh", "Load PickMesh Triggered");
    commands.insert(entity, (PickableMesh::default().with_bounding_sphere(handle.clone()),));
    commands.remove_one::<CreatePickMesh>(entity);
  }
}

pub fn load_asset(
  mut query: Query<(Entity, &GlobalTransform, &Transform, &mut LoadAsset)>,
  commands: &mut Commands,
  asset_server: ResMut<AssetServer>,
  scenes: Res<Assets<Scene>>,
) {
  for (entity, global_trans, _trans, mut asset) in query.iter_mut() {
    info!(target: "load_asset", "Load Asset Triggered");

    if !asset.loading {
      let handle: Handle<Scene> = asset_server.load(Path::new(&asset.path));
      asset.loading = true;
      asset.handle_id = handle.id;
    }

    for (id, scene) in scenes.iter() {
      if id == asset.handle_id {
        let world = &scene.world;
        let ents = world
          .query::<(Entity, &Handle<Mesh>, &Handle<StandardMaterial>)>()
          .map(|(e, m, mat)| (e, m, mat))
          .collect::<Vec<_>>();
        let (_, mesh, mat) = ents.get(asset.mesh_index as usize).expect("Invalid mesh index");
        commands.insert(
          entity,
          PbrBundle {
            global_transform: global_trans.clone(),
            mesh: (*mesh).clone(),
            material: (*mat).clone(),
            ..Default::default()
          },
        );
        commands.remove_one::<LoadAsset>(entity);
      }
    }
  }
}
