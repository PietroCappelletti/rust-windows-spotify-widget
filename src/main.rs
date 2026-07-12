// Suppresses the console window in release builds (so double-clicking the
// .exe just shows the widget, not a terminal). Kept in debug builds so
// `cargo run` during development still shows eprintln!/println! output.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod config;
mod hotkey;
mod tray;
mod spotify;
mod ui;

use rust_windows_spotify_widget::icon;

use std::sync::Arc;

use app::WidgetApp;
use config::Config;
use hotkey::HotkeyListener;
use spotify::SpotifyClient;
use tray::TrayHandle;

fn main() -> eframe::Result<()> {
    let instance = single_instance::SingleInstance::new("rust-windows-spotify-widget-lock")
        .expect("failed to check for existing instance");

    if !instance.is_single() {
        rfd::MessageDialog::new()
            .set_title("Spotify Widget")
            .set_description("Spotify Widget is already running.\n\nCheck your system tray — right-click the icon there to show it or quit it.")
            .set_level(rfd::MessageLevel::Info)
            .show();
        std::process::exit(0);
    }
    
    let config = match Config::load() {
        Ok(config) => config,
        Err(message) => {
            rfd::MessageDialog::new()
                .set_title("Spotify Widget — Setup Required")
                .set_description(&message)
                .set_level(rfd::MessageLevel::Error)
                .show();
            std::process::exit(1);
        }
    };

    let start_visible = false;

    let hotkey = HotkeyListener::new(&config.hotkey_combo);
    let tray = TrayHandle::new();

    let runtime = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");

    let spotify_result = runtime.block_on(SpotifyClient::connect(
        config.spotify_client_id.clone(),
        config.spotify_client_secret.clone(),
        config.spotify_redirect_uri.clone(),
    ));

    let spotify = match spotify_result {
        Ok(client) => Arc::new(client),
        Err(e) => {
            rfd::MessageDialog::new()
                .set_title("Spotify Widget — Login Failed")
                .set_description(&format!(
                    "Couldn't connect to Spotify:\n\n{e}\n\nCheck your .env credentials and that the Redirect URI matches your Spotify Developer App settings."
                ))
                .set_level(rfd::MessageLevel::Error)
                .show();
            std::process::exit(1);
        }
    };

  let options = eframe::NativeOptions {
  viewport: egui::ViewportBuilder::default()
    .with_inner_size([290.0, 80.0])
    .with_decorations(false)
    .with_always_on_top()
    .with_resizable(false)
    .with_visible(true)
    .with_position(egui::pos2(-10000.0, -10000.0))
    .with_taskbar(false)
    .with_icon(egui::IconData {
      rgba: icon::generate_icon_rgba(256),
      width: 256,
      height: 256,
    }),
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