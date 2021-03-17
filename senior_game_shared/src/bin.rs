use std::fs;
use std::path::Path;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::reflect::TypeRegistry;
use senior_game_shared::components::assets::*;
use senior_game_shared::components::input::*;

use crate::scenes::destination_helper::Destination;
use crate::scenes::*;

mod scenes;

pub fn main() {
  App::build()
    .add_plugins(DefaultPlugins)
    .register_type::<CreatePhysics>()
    .register_type::<CreateCollider>()
    .register_type::<RigidbodyType>()
    .register_type::<LoadAsset>()
    .register_type::<ShapeType>()
    .register_type::<BuildFlyCamera>()
    .register_type::<CreateAssetCollider>()
    .register_type::<CreatePickSource>()
    .register_type::<CreatePickMesh>()
    .register_type::<CubeFollow>()
    .add_startup_system(build_scenes.system())
    .add_system(exit_system.system())
    .run();
}

fn exit_system(mut exit: ResMut<Events<AppExit>>) {
  exit.send(AppExit);
}

fn build_scenes(type_registry: Res<TypeRegistry>) {
  #[allow(clippy::type_complexity)]
  const SCENES: &[(
    Destination,
    &str,
    fn(Destination, &Res<TypeRegistry>) -> String,
  )] = &[
    (Destination::Both, "physics_test.scn", physics_test::build),
    (Destination::Both, "platform.scn", platform::build),
    (Destination::Both, "game.scn", game::build),
  ];

  let client_prefix = Path::new("../senior_game_client/assets/scenes");
  let server_prefix = Path::new("../senior_game_server/assets/scenes");

  for (dest, name, build_fn) in SCENES {
    println!("Saving Scene {}", name);
    if *dest == Destination::Client || *dest == Destination::Both {
      fs::write(
        client_prefix.join(name),
        build_fn(Destination::Client, &type_registry),
      )
      .expect("Unable to write file");
    }

    if *dest == Destination::Server || *dest == Destination::Both {
      fs::write(
        server_prefix.join(name),
        build_fn(Destination::Server, &type_registry),
      )
      .expect("Unable to write file");
    }
  }

  println!("Scenes Saved");
}
