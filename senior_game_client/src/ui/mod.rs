use bevy::prelude::*;
pub mod setup;
pub mod background;
pub mod login;

pub struct LoginUIState {
  pub visible: bool,
  pub username: String,
  pub password: String,
}

pub struct BackgroundUIState {
  pub visible: bool
}

impl Default for LoginUIState {
  fn default() -> Self {
    LoginUIState {
      visible: true,
      username: String::default(),
      password: String::default(),
    }
  }
}

impl Default for BackgroundUIState {
  fn default() -> Self {
    BackgroundUIState {
      visible: true
    }
  }
}

pub struct UISystemPlugin;

impl Plugin for UISystemPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<BackgroundUIState>()
      .init_resource::<LoginUIState>()
      .add_startup_system(setup::setup_ui.system())
      .add_system(background::background_ui.system())
      .add_system(login::login_ui.system());
  }
}
