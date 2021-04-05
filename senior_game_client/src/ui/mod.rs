use bevy::prelude::*;
pub mod background;
pub mod login;
pub mod setup;
pub mod queue;
pub mod finding_match;
pub mod match_found;

use chrono::{DateTime, Utc};

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

pub struct QueueUiState {
  pub visible: bool,
}

pub struct FindingMatchUiState {
  pub visible: bool,
  pub start_time: DateTime::<Utc>
}

pub struct MatchFoundUiState {

  pub visible: bool,
  pub accepted: bool
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

impl Default for QueueUiState {
  fn default() -> Self {
    QueueUiState { visible: false }
  }
}

impl Default for FindingMatchUiState {
  fn default() -> Self {
    FindingMatchUiState {
      visible: false,
      start_time: Utc::now()
    }
  }
}

impl Default for MatchFoundUiState {
  fn default() -> Self {
    MatchFoundUiState {
      visible: true,
      accepted: false,
    }
  }
}

pub struct UiSystemPlugin;

impl Plugin for UiSystemPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<BackgroundUiState>()
      .init_resource::<LoginUiState>()
      .init_resource::<QueueUiState>()
      .init_resource::<FindingMatchUiState>()
      .init_resource::<MatchFoundUiState>()
      .add_startup_system(setup::setup_ui.system())
      .add_system(background::background_ui.system())
      .add_system(login::login_ui.system())
      .add_system(login::handle_login_response.system())
      .add_system(queue::queue_ui.system())
      .add_system(finding_match::finding_match_ui.system())
      .add_system(match_found::match_found_ui.system());
  }
}
