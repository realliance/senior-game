use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::ui::MatchFoundUiState;
use crate::ui::FindingMatchUiState;

use crate::proto::ConfirmMatch;

pub fn match_found_ui(
    windows: Res<Windows>,
    commands: &mut Commands,
    mut match_found_state: ResMut<MatchFoundUiState>,
    mut finding_match_state: ResMut<FindingMatchUiState>,
    mut egui_ctx: ResMut<EguiContext>,
  ) {
    if !match_found_state.visible {
      return;
    }

    let window = windows.get_primary().unwrap();

    let width = window.width();
    let height = window.height();

    let ctx = &mut egui_ctx.ctx;

    egui::Window::new("Match Found")
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
        min: egui::pos2(width / 2.4, height / 3.0),
        max: egui::pos2(width / 1.7, height / 3.1),
      })
      .show(ctx, |ui| {

        ui.spacing_mut().item_spacing = egui::vec2(10.0, 15.0);

        ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
          ui.vertical_centered_justified(|ui| {
            ui.heading("Match Found");
            ui.separator();
          });

          ui.centered_and_justified(|ui| {

            if ui.add(egui::Button::new("Accept")
            .text_style(bevy_egui::egui::TextStyle::Heading))
            .clicked() {
              commands.spawn((ConfirmMatch,));
            }

            if ui.add(egui::Button::new("Decline")
            .text_style(bevy_egui::egui::TextStyle::Heading))
            .clicked() {
                finding_match_state.visible = false;
                match_found_state.visible = false;
            }
          });

      });
    });
  }
