use bevy::prelude::*;
use std::collections::HashMap;
use std::pin::Pin;
use std::task::Poll;
use std::future::Future;
use bevy::tasks::TaskPoolBuilder;
use std::sync::{Arc, Mutex};
use futures::poll;
use std::thread;
use reqwest::blocking::{Client, Response, RequestBuilder};
use reqwest::Error;
use tokio::runtime::Runtime;
use lazy_static::lazy_static;

pub enum WebRequestVerb {
  Get,
  Post
}

pub const LOGIN_URL: &str = "https://accounts.senior.realliance.net/session";

pub struct LoginRequestTag;

pub struct HttpRequest {
  pub verb: WebRequestVerb,
  pub url: String,
  pub body: HashMap<String, String>,
}

#[derive(Default)]
struct HttpRequestCompleted(bool, Option<Result<Response, Error>>);

lazy_static! {
  static ref HTTP_IN_PROGRESS: Arc<Mutex<Vec<HttpRequestCompleted>>> = Arc::new(Mutex::new(vec![]));
}

//Box<dyn Future<Output = Result<Response, Error>>>
pub struct HttpInProgress(usize);

pub struct HttpResponse;

fn handle_http_response(query: Query<(Entity, &HttpRequest, &HttpInProgress), Without<HttpResponse>>,commands: &mut Commands) {
  for (entity, _, in_progress) in query.iter() {
    let lock = HTTP_IN_PROGRESS.lock().unwrap();

    let element = lock.get(in_progress.0).unwrap();

    if element.0 {
      let result = element.1.as_ref();
      match result.unwrap() {
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

fn send_request(request: RequestBuilder, index: usize) {
  thread::spawn(move || {
    let res = request.send();
    *HTTP_IN_PROGRESS.lock().unwrap().get_mut(index).unwrap() = HttpRequestCompleted(true, Some(res));
    info!(target: "make_http_requests", "Request Complete!");
  });
}

fn make_http_request(query: Query<(Entity, &HttpRequest), Without<HttpInProgress>>, commands: &mut Commands) {
  let client = Client::new();
  let mut lock = HTTP_IN_PROGRESS.lock().unwrap();

  for (entity, request) in query.iter() {
    info!(target: "make_http_requests", "Starting Request...");
    let res = match &request.verb {
      WebRequestVerb::Get => client.get(&request.url),
      WebRequestVerb::Post => client.post(&request.url),
    }.json(&request.body);

    lock.push(HttpRequestCompleted::default());
    send_request(res, lock.len() - 1);

    commands.insert(entity, (HttpInProgress(lock.len() - 1),));
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
