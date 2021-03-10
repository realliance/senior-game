use bevy::prelude::*;
use std::path::Path;
use bevy::asset::{AssetServer};
use bevy::ecs::{Commands, Entity, Query, Res, ResMut};

use crate::components::assets::*;

pub fn load_sound_system(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    audio: ResMut<Audio>,
    query: Query<(Entity, &LoadSound)>,
  ) {
    for (entity, load_sound) in query.iter() {
      info!(target: "load_sound_system", "Load Sound Triggered: {}", &load_sound.path);

      let sound = asset_server.load(Path::new(&load_sound.path));
      audio.play(sound);

      if load_sound.watch {
        asset_server.watch_for_changes().unwrap();
      }

      commands.despawn(entity);
    }
  }

pub fn load_sound(
  query: Query<(Entity, &AssetChild)>,
  asset_server: ResMut<AssetServer>,
  audio: ResMut<Audio>,
) {
  for (_entity, asset) in query.iter() {
    info!(target: "load_asset", "Load Asset Triggered");
    audio.play(asset_server.load(Path::new(&asset.path)));
  }
}
