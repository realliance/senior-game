use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use serde_json::json;

use crate::http::{login_route, HttpRequest, HttpResponse, LoginRequestTag, WebRequestVerb};
use crate::state::ClientState;
use crate::ui::LoginUiState;

pub fn format_status_error(status: u16) -> String {
  format!("An unknown error has occured with status {}", status)
}

pub fn unknown_error() -> String {
  "An unknown error has occured with no status".to_string()
}

pub fn handle_login_response(
  query: Query<(Entity, &HttpResponse, &LoginRequestTag)>,
  mut login_state: ResMut<LoginUiState>,
  mut client_state: ResMut<ClientState>,
  commands: &mut Commands,
) {
  for (entity, response, _) in query.iter() {
    if !response.is_error {
      let response_code = response.status.unwrap().as_u16();
      match response_code {
        200 => {
          if let Some(token) = response.get_value("token") {
            client_state.username = login_state.username.clone();
            client_state.token = token.to_string();
            login_state.visible = false;
          } else {
            login_state.set_error(unknown_error());
          }
        },
        400 => {
          if let Some(error) = response.get_value("error") {
            login_state.set_error(error.to_string());
          } else {
            login_state.set_error(format_status_error(400));
          }
        },
        x => {
          login_state.set_error(format_status_error(x));
        },
      }
    } else if let Some(status) = response.status {
      login_state.set_error(format_status_error(status.as_u16()));
    } else {
      login_state.set_error(unknown_error());
    }

    commands.despawn(entity);
  }
}

pub fn login_ui(
  windows: Res<Windows>,
  mut login_state: ResMut<LoginUiState>,
  mut egui_ctx: ResMut<EguiContext>,
  commands: &mut Commands,
) {
  if !login_state.visible {
    return;
  }

  let window = windows.get_primary().unwrap();

  let width = window.width();
  let height = window.height();

  let ctx = &mut egui_ctx.ctx;

  egui::Window::new("login")
    .title_bar(false)
    .scroll(false)
    .open(&mut true)
    .frame(egui::Frame {
      margin: egui::Vec2 { x: 10.0, y: 10.0 },
      corner_radius: 0.0,
      fill: egui::Color32::from_rgba_premultiplied(0, 0, 0, 0),
      stroke: egui::Stroke {
        width: 1.0,
        color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 255),
      },
      shadow: egui::paint::Shadow {
        extrusion: 10.0,
        color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 220),
      },
    })
    .fixed_rect(egui::Rect {
      min: egui::pos2(width / 12.0, height / 3.0),
      max: egui::pos2(width / 4.25, height / 2.0),
    })
    .show(ctx, |ui| {
      ui.spacing_mut().item_spacing = egui::vec2(10.0, 15.0);

      ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
        ui.vertical_centered_justified(|ui| {
          ui.heading("Account Login");
          ui.separator();
        });

        ui.spacing_mut().item_spacing = egui::vec2(10.0, 11.0);

        ui.vertical_centered(|ui| {
          ui.label("Username");

          ui.add(
            egui::TextEdit::singleline(&mut login_state.username)
              .text_style(egui::TextStyle::Body)
              .desired_width(500.0),
          );

          login_state.username.truncate(32);
        });

        ui.vertical_centered(|ui| {
          ui.label("Password");

          ui.add(
            egui::TextEdit::singleline(&mut login_state.password)
              .text_style(egui::TextStyle::Monospace)
              .desired_width(500.0),
          );

          login_state.password.truncate(72);
        });

        ui.vertical_centered(|ui| {
          if ui.button("Log In").clicked() {
            login_state.clear_error();

            let request_body = json!({
                "username": login_state.username.clone(),
                "password": login_state.password.clone(),
            });

            commands.spawn((
              LoginRequestTag,
              HttpRequest {
                verb: WebRequestVerb::Post,
                url: login_route(),
                body: Some(request_body),
              },
            ));
          }
        });

        if login_state.has_error {
          ui.centered_and_justified(|ui| {
            ui.add(
              egui::Label::new(&login_state.error_message)
                .text_color(egui::Color32::RED)
                .wrap(true)
                .text_style(egui::TextStyle::Small),
            );
          });
        }

        ui.separator();

        ui.vertical(|ui| {
          ui.add(
            egui::Hyperlink::new("https://accounts.senior.realliance.net/register")
              .text("Register Account")
              .small(),
          );
        });

        ui.vertical(|ui| {
          ui.add(
            egui::Hyperlink::new("https://accounts.senior.realliance.net/password/recovery")
              .text("Forgot Password?")
              .small(),
          );
        });
      });
    });
}
