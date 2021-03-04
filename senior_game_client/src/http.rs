use bevy::prelude::*;
use std::collections::HashMap;

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

pub struct HttpResponse;

fn make_http_requests(query: Query<(Entity, &HttpRequest), Without<HttpResponse>>, commands: &mut Commands) {
  let client = reqwest::blocking::Client::new();
  for (entity, request) in query.iter() {
    info!(target: "make_http_requests", "Starting Request...");
    let res = match &request.verb {
      WebRequestVerb::Get => client.get(&request.url),
      WebRequestVerb::Post => client.post(&request.url),
    }.json(&request.body).send();

    match res {
      Ok(response) => {
        info!(target: "make_http_requests", "Response: {}", response.status());
      },
      Err(error) => {
        info!(target: "make_http_requests", "Error: {}", error.status().unwrap());
      }
    };

    commands.insert(entity, (HttpResponse,));
  }
}

pub struct HttpSystemPlugin;

impl Plugin for HttpSystemPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_system(make_http_requests.system());
  }
}
