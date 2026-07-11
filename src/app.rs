use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant};

use crate::config::Config;
use crate::hotkey::HotkeyListener;
use crate::spotify::{SpotifyClient, Track};
use crate::tray::{TrayCommand, TrayHandle};
use crate::ui::{draw_widget, WidgetAction};

const OFFSCREEN_POS: egui::Pos2 = egui::pos2(-10000.0, -10000.0);
const TRACK_POLL_INTERVAL: Duration = Duration::from_secs(3);

/// Messages sent from background Spotify tasks back to the UI thread.
enum SpotifyEvent {
  TrackUpdated(Option<Track>),
  ActionFailed(String),
}

pub struct WidgetApp {
  visible: bool,
  last_interaction: Instant,
  auto_hide_after: Duration,
  hotkey: HotkeyListener,
  tray: TrayHandle,
  runtime: tokio::runtime::Runtime,
  spotify: Arc<SpotifyClient>,
  current_track: Option<Track>,
  last_error: Option<String>,
  event_tx: mpsc::Sender<SpotifyEvent>,
  event_rx: mpsc::Receiver<SpotifyEvent>,
}

impl WidgetApp {
  pub fn new(
    start_visible: bool,
    hotkey: HotkeyListener,
    tray: TrayHandle,
    config: Config,
    runtime: tokio::runtime::Runtime,
    spotify: Arc<SpotifyClient>,
  ) -> Self {
    let auto_hide_after = Duration::from_secs(config.auto_hide_seconds);
    let (event_tx, event_rx) = mpsc::channel();

    let app = Self {
      visible: start_visible,
      last_interaction: Instant::now(),
      auto_hide_after,
      hotkey,
      tray,
      runtime,
      spotify,
      current_track: None,
      last_error: None,
      event_tx,
      event_rx,
    };

    app.spawn_track_polling_loop();
    app
  }

  /// Background task: periodically fetches the currently playing track
  /// and reports it back via the event channel.
  fn spawn_track_polling_loop(&self) {
    let spotify = self.spotify.clone();
    let tx = self.event_tx.clone();

    self.runtime.spawn(async move {
      loop {
        match spotify.get_current_track().await {
          Ok(track) => {
            let _ = tx.send(SpotifyEvent::TrackUpdated(track));
          }
          Err(e) => {
            let _ = tx.send(SpotifyEvent::ActionFailed(e));
          }
        }
        tokio::time::sleep(TRACK_POLL_INTERVAL).await;
      }
    });
  }

  /// Spawns a one-shot background task for a button action (play/pause/etc),
  /// then triggers an immediate track refresh once it completes.
  fn spawn_action<F, Fut>(&self, action: F)
  where
    F: FnOnce(Arc<SpotifyClient>) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = Result<(), String>> + Send + 'static,
  {
    let spotify = self.spotify.clone();
    let tx = self.event_tx.clone();

    self.runtime.spawn(async move {
      if let Err(e) = action(spotify.clone()).await {
        let _ = tx.send(SpotifyEvent::ActionFailed(e));
        return;
      }
      // Give Spotify a brief moment to update its playback state.
      tokio::time::sleep(Duration::from_millis(300)).await;
      match spotify.get_current_track().await {
        Ok(track) => {
          let _ = tx.send(SpotifyEvent::TrackUpdated(track));
        }
        Err(e) => {
          let _ = tx.send(SpotifyEvent::ActionFailed(e));
        }
      }
    });
  }

  fn handle_widget_action(&mut self, action: WidgetAction) {
    match action {
      WidgetAction::Play => self.spawn_action(|c| async move { c.play().await }),
      WidgetAction::Pause => self.spawn_action(|c| async move { c.pause().await }),
      WidgetAction::Next => self.spawn_action(|c| async move { c.next_track().await }),
      WidgetAction::Previous => self.spawn_action(|c| async move { c.previous_track().await }),
      WidgetAction::None => {}
    }
  }

  fn drain_events(&mut self) {
    while let Ok(event) = self.event_rx.try_recv() {
      match event {
        SpotifyEvent::TrackUpdated(track) => {
          self.current_track = track;
          self.last_error = None;
        }
        SpotifyEvent::ActionFailed(e) => {
          self.last_error = Some(e);
        }
      }
    }
  }

  fn set_shown(&mut self, ctx: &egui::Context, shown: bool) {
    self.visible = shown;
    if shown {
      let cmd = egui::ViewportCommand::center_on_screen(ctx)
        .unwrap_or(egui::ViewportCommand::OuterPosition(egui::pos2(100.0, 100.0)));
      ctx.send_viewport_cmd(cmd);
      self.last_interaction = Instant::now();
    } else {
      ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(OFFSCREEN_POS));
    }
  }

  fn toggle_visibility(&mut self, ctx: &egui::Context) {
    let now_shown = !self.visible;
    self.set_shown(ctx, now_shown);
  }

  fn check_auto_hide(&mut self, ctx: &egui::Context) {
    if self.visible && self.last_interaction.elapsed() > self.auto_hide_after {
      self.set_shown(ctx, false);
    }
  }
}

impl eframe::App for WidgetApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    if self.hotkey.was_pressed() {
      self.toggle_visibility(ctx);
    }

    match self.tray.poll() {
      TrayCommand::ToggleVisibility => self.toggle_visibility(ctx),
      // Exit immediately rather than dropping the tokio Runtime gracefully —
      // for a small desktop widget, an abrupt exit is simpler than waiting
      // on background tasks (like the polling loop) to wind down.
      TrayCommand::Quit => std::process::exit(0),
      TrayCommand::None => {}
    }

    self.drain_events();
    self.check_auto_hide(ctx);

    if self.visible {
      let action = draw_widget(ctx, self.current_track.as_ref(), self.last_error.as_deref());
      if !matches!(action, WidgetAction::None) {
        self.last_interaction = Instant::now();
        self.handle_widget_action(action);
      }

      if ctx.input(|i| i.pointer.any_down() || i.pointer.is_moving()) {
        self.last_interaction = Instant::now();
      }
    }

    ctx.request_repaint_after(Duration::from_millis(200));
  }
}