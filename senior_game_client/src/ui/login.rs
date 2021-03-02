use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use crate::ui::LoginUIState;

pub fn login_ui(windows: Res<Windows>, mut ui_state : ResMut<LoginUIState>, mut egui_ctx : ResMut<EguiContext>) {
  if !ui_state.visible {
    return;
  }

  let window = windows.get_primary().unwrap();

  let width = window.width();
  let height = window.height();

  let ctx = &mut egui_ctx.ctx;

  egui::Window::new("login")
    .title_bar(false)
    .resizable(false)
    .fixed_rect(egui::Rect {
      min: egui::pos2(width / 3.0, height / 4.0),
      max: egui::pos2(2.0 * (width / 3.0), 3.0 * (height / 4.0))
    })
    .show(ctx, |ui| {
      ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
        ui.heading("Wizard Connect 3");
        
        ui.horizontal(|ui| {
          ui.label("Email: ");
          ui.text_edit_singleline(&mut ui_state.username);
        });

        ui.horizontal(|ui| {
          ui.label("Password: ");
          ui.text_edit_singleline(&mut ui_state.username);
        });
      });
    });
}
