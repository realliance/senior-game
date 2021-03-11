use bevy::prelude::*;
use serde_json::to_value;
use reqwest::StatusCode;
use crate::http::{LoginRequestTag, HttpResponse};
use crate::ui::LoginUiState;
use crate::ui::login::{handle_login_response, format_status_error, unknown_error};
use crate::state::ClientState;
use super::run_system;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct ErrorObject {
  error: String
}

#[derive(Serialize, Deserialize, Clone)]
struct SuccessObject {
  token: String
}

#[test]
fn test_successful_login() {
  let test_object = SuccessObject {
    token: "1234".to_string()
  };

  let response = HttpResponse {
    is_error: false,
    status: StatusCode::from_u16(200).ok(),
    response_body: to_value(&test_object).ok(),
  };

  let mut world = World::default();
  let mut resources = Resources::default();

  resources.insert(ClientState::default());
  resources.insert(LoginUiState::default());

  world.spawn((response,LoginRequestTag));

  run_system(&mut world, &mut resources, handle_login_response.system());

  let client_state = resources.get::<ClientState>().unwrap();
  let login_ui_state = resources.get::<LoginUiState>().unwrap();

  assert_eq!(client_state.token, "1234".to_string());
  assert_eq!(login_ui_state.visible, false);
}

#[test]
fn test_invalid_password() {
  let test_object = ErrorObject {
    error: "1234".to_string()
  };

  let response = HttpResponse {
    is_error: false,
    status: StatusCode::from_u16(400).ok(),
    response_body: to_value(&test_object).ok(),
  };

  let mut world = World::default();
  let mut resources = Resources::default();

  resources.insert(ClientState::default());
  resources.insert(LoginUiState::default());

  world.spawn((response,LoginRequestTag));

  run_system(&mut world, &mut resources, handle_login_response.system());

  let client_state = resources.get::<ClientState>().unwrap();
  let login_ui_state = resources.get::<LoginUiState>().unwrap();

  assert_eq!(client_state.token, String::default());
  assert!(login_ui_state.visible);
  assert!(login_ui_state.has_error);
  assert_eq!(login_ui_state.error_message, "1234".to_string());
}

#[test]
fn test_unknown_error_with_status() {
  let response = HttpResponse {
    is_error: true,
    status: StatusCode::from_u16(123).ok(),
    response_body: None,
  };

  let mut world = World::default();
  let mut resources = Resources::default();

  resources.insert(ClientState::default());
  resources.insert(LoginUiState::default());

  world.spawn((response,LoginRequestTag));

  run_system(&mut world, &mut resources, handle_login_response.system());

  let client_state = resources.get::<ClientState>().unwrap();
  let login_ui_state = resources.get::<LoginUiState>().unwrap();

  assert_eq!(client_state.token, String::default());
  assert!(login_ui_state.visible);
  assert!(login_ui_state.has_error);
  assert_eq!(login_ui_state.error_message, format_status_error(123));
}

#[test]
fn test_unknown_error() {
  let response = HttpResponse {
    is_error: true,
    status: None,
    response_body: None,
  };

  let mut world = World::default();
  let mut resources = Resources::default();

  resources.insert(ClientState::default());
  resources.insert(LoginUiState::default());

  world.spawn((response,LoginRequestTag));

  run_system(&mut world, &mut resources, handle_login_response.system());

  let client_state = resources.get::<ClientState>().unwrap();
  let login_ui_state = resources.get::<LoginUiState>().unwrap();

  assert_eq!(client_state.token, String::default());
  assert!(login_ui_state.visible);
  assert!(login_ui_state.has_error);
  assert_eq!(login_ui_state.error_message, unknown_error());
}
