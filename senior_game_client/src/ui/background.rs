use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use crate::ui::BackgroundUIState;

pub fn background_ui(
  windows: Res<Windows>,
  background_state : ResMut<BackgroundUIState>,
  mut egui_ctx : ResMut<EguiContext>,
  assets: ResMut<AssetServer>) {

  if !background_state.visible {
    return;
  }

  let window = windows.get_primary().unwrap();

  let width = window.width();
  let height = window.height();

  let texture_handle = assets.load("images/wizard.png");
  egui_ctx.set_egui_texture(1, texture_handle);

  let ctx = &mut egui_ctx.ctx;

  egui::CentralPanel::default()

  .frame(egui::Frame {
    margin: egui::Vec2 { x: 0.0, y: 0.0 },
    corner_radius: 0.0,
    fill: egui::Color32::from_rgba_premultiplied(0, 0, 0, 0),
    stroke: egui::Stroke { width: 0.0, color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 0) },
    shadow: egui::paint::Shadow { extrusion: 0.0, color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 0)}
  })

  .show(ctx, |ui| {
      ui.image(egui::TextureId::User(1), [width, height]);
  });
}
