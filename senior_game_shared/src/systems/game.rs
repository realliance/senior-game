use bevy::prelude::*;

use crate::components::assets::*;
use crate::components::game::*;

pub fn load_source_model(commands: &mut Commands, query: Query<(Entity, &BuildSourceModel, &ManaSource)>) {
  for (entity, _, mana_source) in query.iter() {
    if let Some(path) = mana_source.source_type().get_path() {
      let ent = commands
        .spawn((LoadAsset {
          path: path.to_string(),
          ..Default::default()
        },))
        .current_entity()
        .unwrap();

      commands.push_children(entity, &[ent]);
    }

    commands.remove_one::<BuildSourceModel>(entity);
  }
}

pub struct GameSystemsPlugin;

impl Plugin for GameSystemsPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .register_type::<ManaSource>()
      .register_type::<SourceType>()
      .register_type::<BuildSourceModel>()
      .add_system(load_source_model.system());
  }
}
