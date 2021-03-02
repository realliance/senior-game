use bevy::prelude::*;
pub mod login;

pub struct LoginUIState {
  pub visible: bool,
  pub username: String,
  pub password: String,
}

impl Default for LoginUIState {
  fn default() -> Self {
    LoginUIState {
      visible: true,
      username: String::default(),
      password: String::default()
    }
 }
}

pub struct UISystemPlugin;

impl Plugin for UISystemPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<LoginUIState>()
      .add_system(login::login_ui.system());
  }
}
