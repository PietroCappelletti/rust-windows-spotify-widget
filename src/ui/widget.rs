use crate::spotify::Track;

/// Draws the widget's contents (track info + control buttons).
/// Returns which button (if any) was clicked this frame.
pub enum WidgetAction {
  None,
  Play,
  Pause,
  Next,
  Previous,
}

pub fn draw_widget(ctx: &egui::Context, track: Option<&Track>) -> WidgetAction {
  todo!("draw album art, track name/artist, and control buttons with egui")
}