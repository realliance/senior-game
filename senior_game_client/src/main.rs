#[cfg(debug_assertions)]
use std::env;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_rapier3d::physics::RapierPhysicsPlugin;
#[cfg(debug_assertions)]
use bevy_rapier3d::render::RapierRenderPlugin;
use senior_game_shared::components::assets::*;
use senior_game_shared::systems::loadscene::*;

fn main() {
  App::build()
    .add_resource(Msaa::default())
    .add_plugins(DefaultPlugins)
    .add_plugins(FlaggedPlugins)
    .add_plugin(RapierPhysicsPlugin)
    .register_type::<PhysicsBuilder>()
    .register_type::<RigidbodyType>()
    .register_type::<AssetChild>()
    .register_type::<ShapeType>()
    .add_startup_system(manual_load_scene.system())
    .add_system(load_scene_system.system())
    .add_system_to_stage(stage::POST_UPDATE, load_asset.system())
    .add_system_to_stage(stage::POST_UPDATE, load_physics.system())
    .run();
}

pub struct FlaggedPlugins;

impl PluginGroup for FlaggedPlugins {
  fn build(&mut self, group: &mut PluginGroupBuilder) {
    let args: Vec<String> = env::args().collect();
    if cfg!(debug_assertions) {
      if args.contains(&"debugrender".to_string()) {
        println!("Render Debug Activated");
        group.add(RapierRenderPlugin);
      }
    }
  }
}

fn manual_load_scene(commands: &mut Commands) {
  commands.spawn(()).with(LoadScene {
    path: "scenes/physics_test.scn".to_string(),
    watch: false,
  });

  println!("Scene Manually Loaded");
}
