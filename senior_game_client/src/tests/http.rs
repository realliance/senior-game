use bevy::prelude::*;
use mockito::{mock, Mock};
use serde::{Deserialize, Serialize};
use serde_json::{to_string, to_value};
use std::time::Instant;
use reqwest::StatusCode;
use crate::http::{login_route, HttpRequest, WebRequestVerb, HttpInProgress, HttpResponse, make_http_request, handle_http_response};
use super::run_system;

#[derive(Serialize, Deserialize, Clone)]
struct TestJSONResponse {
  value: String
}

fn create_test_endpoint(method: &str, path: &str, response: usize, body: Option<&TestJSONResponse>) -> Mock {
  let mut m = mock(method, path)
  .with_status(response);

  if let Some(b) = body {
    m = m.with_header("content-type", "application/json; charset=utf-8")
      .with_body(to_string(&b).unwrap());
  }

  return m.create();
}

#[test]
fn test_get_request_is_processed() {
  let test_object = TestJSONResponse {
    value: 2.to_string()
  };

  let _m = create_test_endpoint("GET", "/session", 200, Some(&test_object));

  let mut world = World::default();
  let mut resources = Resources::default();

  world.spawn((HttpRequest {
    url: login_route(),
    verb: WebRequestVerb::Get,
    body: None
  },));

  run_system(&mut world, &mut resources, make_http_request.system());

  let ents = world.query::<(Entity, &HttpInProgress)>().map(|(e, h)| (e, h)).collect::<Vec<_>>();

  // Entity was tagged with HttpInProgress
  assert_eq!(ents.len(), 1);
}

#[test]
fn test_post_request_is_processed() {
  let test_object = TestJSONResponse {
    value: 2.to_string()
  };

  let _m = create_test_endpoint("POST", "/session", 200, Some(&test_object));

  let mut world = World::default();
  let mut resources = Resources::default();

  world.spawn((HttpRequest {
    url: login_route(),
    verb: WebRequestVerb::Post,
    body: None
  },));

  run_system(&mut world, &mut resources, make_http_request.system());

  let ents = world.query::<(Entity, &HttpInProgress)>().map(|(e, h)| (e, h)).collect::<Vec<_>>();

  // Entity was tagged with HttpInProgress
  assert_eq!(ents.len(), 1);
}

#[test]
fn test_request_response_ok() {
  let test_object = TestJSONResponse {
    value: 2.to_string()
  };

  let _m = create_test_endpoint("POST", "/session", 200, Some(&test_object));

  let mut world = World::default();
  let mut resources = Resources::default();

  let e = world.spawn((HttpRequest {
    url: login_route(),
    verb: WebRequestVerb::Post,
    body: None
  },));

  // Start Request
  run_system(&mut world, &mut resources, make_http_request.system());

  let start = Instant::now();
  let expected_body = to_value(&test_object).ok();

  let expected_component = HttpResponse {
    is_error: false,
    status: Some(StatusCode::from_u16(200).unwrap()),
    response_body: expected_body
  };

  loop {
    run_system(&mut world, &mut resources, handle_http_response.system());

    // Check if response completed
    let ents = world.query::<(Entity, &HttpResponse)>().map(|(e, h)| (e, h)).collect::<Vec<_>>();

    // Leave Loop of Task Completed
    if ents.len() > 0 {
      assert_eq!(ents.len(), 1);
      assert!(ents.contains(&(e, &expected_component)));
      break;
    }

    if start.elapsed().as_secs() > 2 {
      panic!("Request look longer than expected.");
    }
  }
}

#[test]
fn test_404_result() {
  // Verify does not panic on empty body
  let _m = create_test_endpoint("POST", "/session", 404, None);

  let mut world = World::default();
  let mut resources = Resources::default();

  let e = world.spawn((HttpRequest {
    url: login_route(),
    verb: WebRequestVerb::Post,
    body: None
  },));

  // Start Request
  run_system(&mut world, &mut resources, make_http_request.system());

  let start = Instant::now();

  let expected_component = HttpResponse {
    is_error: false,
    status: Some(StatusCode::from_u16(404).unwrap()),
    response_body: None
  };

  loop {
    run_system(&mut world, &mut resources, handle_http_response.system());

    // Check if response completed
    let ents = world.query::<(Entity, &HttpResponse)>().map(|(e, h)| (e, h)).collect::<Vec<_>>();

    // Leave Loop of Task Completed
    if ents.len() > 0 {
      assert_eq!(ents.len(), 1);
      assert!(ents.contains(&(e, &expected_component)), "Did not contain component, ents: {:?}", ents);
      break;
    }

    if start.elapsed().as_secs() > 2 {
      panic!("Request look longer than expected.");
    }
  }
}
