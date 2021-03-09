use bevy::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use reqwest::blocking::{Client, Response, RequestBuilder};
use reqwest::Error;

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

pub struct HttpResponse;

fn handle_http_response(query: Query<(Entity, &HttpRequest, &HttpInProgress), Without<HttpResponse>>,commands: &mut Commands) {
  for (entity, _, in_progress) in query.iter() {
    let element = in_progress.0.lock().unwrap();

    if let Some(result) = &*element {
      match result {
        Ok(response) => {
          info!(target: "make_http_requests", "Response: {}", response.status());
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
        }
      };
      commands.insert(entity, (HttpResponse,));
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
