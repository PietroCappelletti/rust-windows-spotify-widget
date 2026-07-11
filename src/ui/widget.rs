use egui::{Context, RichText};
use crate::spotify::Track;

pub enum WidgetAction {
  None,
  Play,
  Pause,
  Next,
  Previous,
}

/// Draws the widget's contents (track info + control buttons) inside a
/// central panel. Returns which button (if any) was clicked this frame.
pub fn draw_widget(ctx: &Context, track: Option<&Track>, error: Option<&str>) -> WidgetAction {
  let mut action = WidgetAction::None;

  egui::CentralPanel::default().show(ctx, |ui| {
    match track {
      Some(t) => {
        ui.label(RichText::new(&t.name).strong());
        ui.label(&t.artist);
      }
      None => {
        ui.label("Nothing is currently playing.");
      }
    }

    ui.add_space(6.0);

    ui.horizontal(|ui| {
      if ui.button("⏮").clicked() {
        action = WidgetAction::Previous;
      }

      let play_pause_label = match track {
        Some(t) if t.is_playing => "⏸",
        _ => "▶",
      };
      if ui.button(play_pause_label).clicked() {
        action = match track {
          Some(t) if t.is_playing => WidgetAction::Pause,
          _ => WidgetAction::Play,
        };
      }

      if ui.button("⏭").clicked() {
        action = WidgetAction::Next;
      }
    });

    if let Some(err) = error {
      ui.add_space(4.0);
      ui.colored_label(egui::Color32::RED, err);
    }
  });

  action
}