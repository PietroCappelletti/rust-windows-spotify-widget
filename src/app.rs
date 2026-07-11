use std::time::{Duration, Instant};

/// Main application state, driving the eframe update loop.
pub struct WidgetApp {
  visible: bool,
  last_interaction: Instant,
  auto_hide_after: Duration,
}

impl WidgetApp {
  pub fn new(start_visible: bool) -> Self {
    Self {
      visible: start_visible,
      last_interaction: Instant::now(),
      auto_hide_after: Duration::from_secs(6),
    }
  }

  /// Toggles widget visibility and resets the auto-hide timer.
  fn toggle_visibility(&mut self) {
    self.visible = !self.visible;
    self.last_interaction = Instant::now();
  }

  /// Hides the widget if the auto-hide timeout has elapsed.
  fn check_auto_hide(&mut self) {
    if self.visible && self.last_interaction.elapsed() > self.auto_hide_after {
      self.visible = false;
    }
  }
}

impl eframe::App for WidgetApp {
  fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    // TEMPORARY: pressing Space simulates the future global hotkey.
    // This will be replaced by hotkey.rs + global-hotkey crate in the next step.
    if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
      self.toggle_visibility();
    }

    self.check_auto_hide();

    // Sync actual OS window visibility with our `visible` flag.
    ctx.send_viewport_cmd(egui::ViewportCommand::Visible(self.visible));

    if self.visible {
      egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Spotify Widget");
        ui.label("Press Space to hide. It will auto-hide after a few seconds of inactivity.");

        // Any interaction resets the auto-hide timer.
        if ui.ui_contains_pointer() {
          self.last_interaction = Instant::now();
        }
      });
    }

    // Keep repainting periodically so check_auto_hide() gets re-evaluated
    // even without user input.
    ctx.request_repaint_after(Duration::from_millis(500));
  }
}