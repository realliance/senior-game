use bevy::prelude::*;
pub mod setup;
pub mod background;
pub mod login;

pub struct LoginUIState {
  pub visible: bool,
  pub username: String,
  pub password: String,
  pub has_error: bool,
  pub error_message: String,
}

impl LoginUIState {
  pub fn set_error(&mut self, msg: String) {
    self.error_message = msg;
    self.has_error = true;
  }

  pub fn clear_error(&mut self) {
    self.has_error = false;
  }
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
      has_error: false,
      error_message: String::default(),
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
      .add_system(login::login_ui.system())
      .add_system(login::handle_login_response.system());
  }
}
