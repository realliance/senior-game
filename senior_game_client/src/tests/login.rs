use bevy::prelude::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::to_value;

use super::run_system;
use crate::http::{HttpResponse, LoginRequestTag};
use crate::state::ClientState;
use crate::ui::login::{format_status_error, handle_login_response, unknown_error};
use crate::ui::{LoginUiState, QueueUiState};

#[derive(Serialize, Deserialize, Clone)]
struct ErrorObject {
  error: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct SuccessObject {
  token: String,
}

#[test]
fn test_successful_login() {
  let test_object = SuccessObject {
    token: "1234".to_string(),
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
  resources.insert(QueueUiState::default());

  world.spawn((response, LoginRequestTag));

  run_system(&mut world, &mut resources, handle_login_response.system());

  let client_state = resources.get::<ClientState>().unwrap();

  assert_eq!(client_state.token, "1234".to_string());
}

#[test]
fn test_invalid_password() {
  let test_object = ErrorObject {
    error: "1234".to_string(),
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
  resources.insert(QueueUiState::default());

  world.spawn((response, LoginRequestTag));

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
  resources.insert(QueueUiState::default());

  world.spawn((response, LoginRequestTag));

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
  resources.insert(QueueUiState::default());

  world.spawn((response, LoginRequestTag));

  run_system(&mut world, &mut resources, handle_login_response.system());

  let client_state = resources.get::<ClientState>().unwrap();
  let login_ui_state = resources.get::<LoginUiState>().unwrap();

  assert_eq!(client_state.token, String::default());
  assert!(login_ui_state.visible);
  assert!(login_ui_state.has_error);
  assert_eq!(login_ui_state.error_message, unknown_error());
}
