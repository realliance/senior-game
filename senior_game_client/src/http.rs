use bevy::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use reqwest::blocking::{Client, Response, RequestBuilder};
use reqwest::{StatusCode, Error};
use serde_json::{Map, Value};

pub enum WebRequestVerb {
  Get,
  Post
}

pub const LOGIN_URL: &str = "https://accounts.senior.realliance.net/session";

pub struct LoginRequestTag;

pub struct HttpRequest {
  pub verb: WebRequestVerb,
  pub url: String,
  pub body: serde_json::Value,
}

#[derive(Default)]
pub struct HttpInProgress(Arc<Mutex<Option<Result<Response, Error>>>>);

pub struct HttpResponse {
  pub is_error: bool,
  pub status: Option<StatusCode>,
  pub response_body: Option<Value>,
}

impl HttpResponse {
  pub fn get_json_object(&self) -> &Map<String, Value> {
    return self.response_body.as_ref().unwrap().as_object().unwrap();
  }

  pub fn get_value(&self, field: &str) -> String {
    return self.get_json_object().get(field).unwrap().as_str().unwrap().to_string();
  }
}

fn handle_http_response(mut query: Query<(Entity, &HttpRequest, &mut HttpInProgress), Without<HttpResponse>>, commands: &mut Commands) {
  for (entity, _, mut in_progress) in query.iter_mut() {
    let mut_borrow = Arc::get_mut(&mut in_progress.0);

    // Request thread holds onto mutable Arc until request finishes
    if mut_borrow.is_none() {
      continue;
    }

    let element = mut_borrow.unwrap().get_mut().unwrap();

    if let Some(result) = element.as_mut() {
      match result {
        Ok(response) => {
          info!(target: "make_http_requests", "Response: {}", response.status());
          let mut buf: Vec<u8> = vec![];
          response.copy_to(&mut buf).unwrap();
          let json: Option<Value> = serde_json::from_slice(&buf).unwrap();
          commands.insert(entity, (HttpResponse {
            is_error: false,
            status: Some(response.status()),
            response_body: json,
          },));
        },
        Err(error) => {
          info!(target: "make_http_requests", "Error Occured");
          info!(target: "make_http_requests", "Due To Builder? {}", error.is_builder());
          info!(target: "make_http_requests", "Due To Redirect? {}", error.is_redirect());
          info!(target: "make_http_requests", "Due To Status? {}", error.is_status());
          if error.is_status() {
            info!(target: "make_http_requests", "Status: {}", error.status().unwrap());
          }
          info!(target: "make_http_requests", "Due To Timeout? {}", error.is_timeout());
          info!(target: "make_http_requests", "Due To Request? {}", error.is_request());

          commands.insert(entity, (HttpResponse {
            is_error: true,
            status: error.status(),
            response_body: None,
          },));
        }
      };
    }
  }
}

fn send_request(request: RequestBuilder, in_progress: &HttpInProgress) {
  let cloned = in_progress.0.clone();
  thread::spawn(move || {
    let res = request.send();
    *cloned.lock().unwrap() = Some(res);
    info!(target: "make_http_requests", "Request Complete!");
  });
}

fn make_http_request(query: Query<(Entity, &HttpRequest), Without<HttpInProgress>>, commands: &mut Commands) {
  let client = Client::new();

  for (entity, request) in query.iter() {
    info!(target: "make_http_requests", "Starting Request...");
    let res = match &request.verb {
      WebRequestVerb::Get => client.get(&request.url),
      WebRequestVerb::Post => client.post(&request.url),
    }.json(&request.body);

    let in_progress = HttpInProgress::default();

    send_request(res, &in_progress);

    commands.insert(entity, (in_progress,));
  }
}

pub struct HttpSystemPlugin;

impl Plugin for HttpSystemPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_stage_after(stage::UPDATE, "http_requests", SystemStage::parallel())
      .add_system_to_stage("http_requests", make_http_request.system())
      .add_system_to_stage("http_requests", handle_http_response.system());
  }
}
