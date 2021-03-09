use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use crate::ui::LoginUIState;
use crate::http::{LoginRequestTag, HttpRequest, WebRequestVerb, LOGIN_URL};
use serde_json::json;

pub fn login_ui(
  windows: Res<Windows>,
  mut login_state : ResMut<LoginUIState>,
  mut egui_ctx : ResMut<EguiContext>,
  commands: &mut Commands) {

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
      stroke: egui::Stroke { width: 1.0, color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 255) },
      shadow: egui::paint::Shadow { extrusion: 10.0, color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 220)}
    })

    .fixed_rect(egui::Rect {
      min: egui::pos2(width / 12.0, height / 3.0),
      max: egui::pos2(width / 4.25, height / 2.0)
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

        ui.add(egui::TextEdit::singleline(&mut login_state.username)
        .text_style(egui::TextStyle::Body)
        .desired_width(500.0));

        login_state.username.truncate(25);
      });

      ui.vertical_centered(|ui| {
        ui.label("Password");

        //TODO: Rerender * for password
        ui.add(egui::TextEdit::singleline(&mut login_state.password)
        .text_style(egui::TextStyle::Body)
        .desired_width(500.0));

        login_state.password.truncate(35);
      });

      ui.vertical_centered(|ui| {
        if ui.button("Log In").clicked(){

          let request_body = json!({
            "username": login_state.username.clone(),
            "password": login_state.password.clone(),
        });

          commands.spawn((LoginRequestTag, HttpRequest {
            verb: WebRequestVerb::Post,
            url: LOGIN_URL.to_string(),
            body: request_body,
          }));
        }
      });

      ui.separator();

      ui.vertical(|ui| {
        ui.add(egui::Hyperlink::new("https://accounts.senior.realliance.net/register").text("Register Account").small());
      });

      ui.vertical(|ui| {
        //TODO: Update link to password recovery endpoint
        ui.add(egui::Hyperlink::new("https://github.com/realliance/senior-game").text("Recover Account").small());
      });
    });
  });
}
