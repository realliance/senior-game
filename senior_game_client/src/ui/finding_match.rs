use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use chrono::{Utc};

use crate::ui::FindingMatchUiState;

pub fn finding_match_ui(
  windows: Res<Windows>,
  finding_match_state: ResMut<FindingMatchUiState>,
  mut egui_ctx: ResMut<EguiContext>,
) {
  if !finding_match_state.visible {
    return;
  }

  let window = windows.get_primary().unwrap();

  let width = window.width();
  let height = window.height();

  let ctx = &mut egui_ctx.ctx;

  egui::Window::new("Finding Match")
    .title_bar(false)
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
      max: egui::pos2(width / 4.25, height / 3.1),
    })
    .show(ctx, |ui| {

      ui.spacing_mut().item_spacing = egui::vec2(10.0, 15.0);

      ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
        ui.vertical_centered_justified(|ui| {
          ui.heading("Finding Match");
          ui.separator();
        });


      let time_elapsed = Utc::now() - finding_match_state.start_time;

      ui.add(egui::Label::new(format!(
        "{:2}:{:02}",
        time_elapsed.num_minutes(),
        time_elapsed.num_seconds() % 60
      ))
      .text_style(bevy_egui::egui::TextStyle::Heading));

    });
  });
}
