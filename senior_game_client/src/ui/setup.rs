use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub fn setup_ui(mut egui_ctx: ResMut<EguiContext>) {
  let ctx = &mut egui_ctx.ctx;

  let mut fonts = egui::FontDefinitions::default();

  fonts.font_data.insert(
    "LibreBaskerville-Regular".to_owned(),
    std::borrow::Cow::Borrowed(include_bytes!("../../assets/fonts/LibreBaskerville-Regular.ttf")),
  );

  fonts.font_data.insert(
    "hidden".to_owned(),
    std::borrow::Cow::Borrowed(include_bytes!("../../assets/fonts/hidden.ttf")),
  );

  fonts.fonts_for_family.insert(
    egui::FontFamily::Proportional,
    vec!["LibreBaskerville-Regular".to_owned()],
  );

  fonts
    .fonts_for_family
    .insert(egui::FontFamily::Monospace, vec!["hidden".to_owned()]);

  fonts
    .family_and_size
    .insert(egui::TextStyle::Heading, (egui::FontFamily::Proportional, 30.0));

  fonts
    .family_and_size
    .insert(egui::TextStyle::Body, (egui::FontFamily::Proportional, 22.0));

  fonts
    .family_and_size
    .insert(egui::TextStyle::Button, (egui::FontFamily::Proportional, 22.0));

  fonts
    .family_and_size
    .insert(egui::TextStyle::Small, (egui::FontFamily::Proportional, 15.0));

  fonts
    .family_and_size
    .insert(egui::TextStyle::Monospace, (egui::FontFamily::Monospace, 22.0));

  ctx.set_fonts(fonts);
}
