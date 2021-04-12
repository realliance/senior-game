use bevy::prelude::*;
pub mod background;
pub mod login;
pub mod setup;

pub struct LoginUiState {
  pub visible: bool,
  pub username: String,
  pub password: String,
  pub has_error: bool,
  pub error_message: String,
}

impl LoginUiState {
  pub fn set_error(&mut self, msg: String) {
    self.error_message = msg;
    self.has_error = true;
  }

  pub fn clear_error(&mut self) {
    self.has_error = false;
  }
}

pub struct BackgroundUiState {
  pub visible: bool,
}

impl Default for LoginUiState {
  fn default() -> Self {
    LoginUiState {
      visible: true,
      username: String::default(),
      password: String::default(),
      has_error: false,
      error_message: String::default(),
    }
  }
}

impl Default for BackgroundUiState {
  fn default() -> Self {
    BackgroundUiState { visible: true }
  }
}

pub struct UiSystemPlugin;

impl Plugin for UiSystemPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<BackgroundUiState>()
      .init_resource::<LoginUiState>()
      .add_startup_system(setup::setup_ui.system())
      .add_system(background::background_ui.system())
      .add_system(login::login_ui.system())
      .add_system(login::handle_login_response.system());
  }
}
