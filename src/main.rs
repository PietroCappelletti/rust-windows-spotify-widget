mod app;
mod config;
mod hotkey;
mod tray;
mod spotify;
mod ui;

use std::sync::Arc;

use app::WidgetApp;
use config::Config;
use hotkey::HotkeyListener;
use spotify::SpotifyClient;
use tray::TrayHandle;

fn main() -> eframe::Result<()> {
  let config = Config::load();
  let start_visible = false;

  let hotkey = HotkeyListener::new();
  let tray = TrayHandle::new();

  // Lives for the whole app's lifetime; used to run Spotify API calls
  // on background threads without blocking the UI thread.
  let runtime = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");

  let spotify_result = runtime.block_on(SpotifyClient::connect(
    config.spotify_client_id.clone(),
    config.spotify_client_secret.clone(),
    config.spotify_redirect_uri.clone(),
  ));

  let spotify = match spotify_result {
    Ok(client) => Arc::new(client),
    Err(e) => {
      eprintln!("[auth] Login failed, exiting: {e}");
      std::process::exit(1);
    }
  };

  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
      .with_inner_size([250.0, 76.0])
      .with_decorations(false)
      .with_always_on_top()
      .with_resizable(false)
      .with_visible(true)
      .with_position(egui::pos2(-10000.0, -10000.0))
      .with_taskbar(false),
    ..Default::default()
  };

  eframe::run_native(
    "rust-windows-spotify-widget",
    options,
    Box::new(move |_cc| {
      Ok(Box::new(WidgetApp::new(
        start_visible,
        hotkey,
        tray,
        config,
        runtime,
        spotify,
      )))
    }),
  )
}