use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::proto::{CancelQueue, EnterQueue};
use crate::ui::{FindingMatchUiState, QueueUiState};

pub fn queue_ui(
  windows: Res<Windows>,
  commands: &mut Commands,
  queue_state: ResMut<QueueUiState>,
  finding_match_state: Res<FindingMatchUiState>,
  mut egui_ctx: ResMut<EguiContext>,
) {
  if !queue_state.visible {
    return;
  }

  let window = windows.get_primary().unwrap();

  let width = window.width();
  let height = window.height();

  let ctx = &mut egui_ctx.ctx;

  egui::Window::new("Find Match")
    .title_bar(false)
    .frame(egui::Frame {
      margin: egui::Vec2 { x: 0.0, y: 0.0 },
      corner_radius: 0.0,
      fill: egui::Color32::from_rgba_premultiplied(0, 0, 0, 0),
      stroke: egui::Stroke {
        width: 1.0,
        color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 255),
      },
      shadow: egui::paint::Shadow {
        extrusion: 20.0,
        color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 255),
      },
    })
    .fixed_rect(egui::Rect {
      min: egui::pos2(width / 24.0, height / 1.1),
      max: egui::pos2(width / 4.25, height / 1.1),
    })
    .show(ctx, |ui| {
      ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
        ui.horizontal(|ui| {
          if !finding_match_state.visible {
            if ui
              .add(egui::Button::new("Find Match").text_style(bevy_egui::egui::TextStyle::Heading))
              .clicked()
            {
              commands.spawn((EnterQueue,));
            }
          } else if ui
            .add(egui::Button::new("Cancel").text_style(bevy_egui::egui::TextStyle::Heading))
            .clicked()
          {
            commands.spawn((CancelQueue,));
          }
        });
      });
    });
}
