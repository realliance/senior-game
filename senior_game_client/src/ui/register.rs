use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use crate::ui::RegisterUIState;
use crate::http::{RegisterRequestTag, HttpRequest, WebRequestVerb, REGISTER_URL};
use serde_json::json;

pub fn register_ui(
  windows: Res<Windows>,
  mut register_state : ResMut<RegisterUIState>,
  mut egui_ctx : ResMut<EguiContext>,
  commands: &mut Commands){

  if !register_state.visible {
    return;
  }

  let window = windows.get_primary().unwrap();

  let width = window.width();
  let height = window.height();

  let ctx = &mut egui_ctx.ctx;

  egui::Window::new("Account Registration")
   .title_bar(true)
   //.collapsible(true)
   .resizable(false)
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

      ui.spacing_mut().item_spacing = egui::vec2(10.0, 11.0);

      ui.vertical(|ui| {
        ui.label("Email");

        ui.add(egui::TextEdit::singleline(&mut register_state.email)
        .text_style(egui::TextStyle::Body)
        .desired_width(500.0));

        register_state.email.truncate(35);
      });

      ui.vertical(|ui| {
        ui.label("Username");

        ui.add(egui::TextEdit::singleline(&mut register_state.username)
        .text_style(egui::TextStyle::Body)
        .desired_width(500.0));

        register_state.username.truncate(25);
      });

      ui.vertical(|ui| {
        ui.label("Password");

        //TODO: Rerender * for password
        ui.add(egui::TextEdit::singleline(&mut register_state.password)
        .text_style(egui::TextStyle::Body)
        .desired_width(500.0));

        register_state.password.truncate(35);
      });

      ui.vertical(|ui| {
        if ui.button("Register").clicked(){

          let request_body = json!({
            "user": {
              "username": register_state.username.clone(),
              "email": register_state.email.clone(),
              "password": register_state.password.clone(),
            }
          });

          commands.spawn((RegisterRequestTag, HttpRequest {
            verb: WebRequestVerb::Post,
            url: REGISTER_URL.to_string(),
            body: request_body,
          }));
        }
      });
    });
  });
}
