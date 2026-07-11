use crate::config::Config;
use crate::hotkey::HotkeyListener;
use crate::spotify::SpotifyClient;

/// Main application state, driving the eframe update loop.
pub struct WidgetApp {
  visible: bool,
  last_interaction: std::time::Instant,
  config: Config,
  hotkey: HotkeyListener,
  spotify: SpotifyClient,
}

impl WidgetApp {
  pub fn new(config: Config, hotkey: HotkeyListener, spotify: SpotifyClient) -> Self {
    todo!("initialize WidgetApp with visible = false")
  }

  /// Toggles widget visibility (called on hotkey press).
  fn toggle_visibility(&mut self) {
    todo!()
  }

  /// Hides the widget if the auto-hide timeout has elapsed.
  fn check_auto_hide(&mut self) {
    todo!()
  }
}

impl eframe::App for WidgetApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // TODO:
    // 1. check hotkey.was_pressed() -> toggle_visibility()
    // 2. check_auto_hide()
    // 3. if visible, draw ui::widget
    todo!()
  }
}