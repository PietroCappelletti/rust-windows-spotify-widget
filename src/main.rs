mod app;
mod config;
mod hotkey;
mod tray;
mod spotify;
mod ui;

use app::WidgetApp;

fn main() -> eframe::Result<()> {
    let start_visible = false;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 100.0])
            .with_decorations(false)
            .with_always_on_top()
            .with_resizable(false)
            .with_visible(start_visible),
        ..Default::default()
    };

    eframe::run_native(
        "rust-windows-spotify-widget",
        options,
        Box::new(move |_cc| Ok(Box::new(WidgetApp::new(start_visible)))),
    )
}