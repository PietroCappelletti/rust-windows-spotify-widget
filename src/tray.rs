use tray_icon::{
  menu::{Menu, MenuEvent, MenuId, MenuItem},
  Icon, TrayIcon, TrayIconBuilder,
};

/// Commands the tray menu can produce, polled once per frame by `app.rs`.
pub enum TrayCommand {
    None,
    ToggleVisibility,
    OpenSettings,
    Quit,
}

/// Owns the Windows system tray icon and its right-click menu
/// (Show/Hide, Quit). Must be created on the same thread that will
/// run the native event loop (i.e. before `eframe::run_native`).
pub struct TrayHandle {
    _tray_icon: TrayIcon,
    toggle_id: MenuId,
    settings_id: MenuId,
    quit_id: MenuId,
}

impl TrayHandle {
    pub fn new() -> Self {
        let icon = build_app_icon();

        let menu = Menu::new();
        let toggle_item = MenuItem::new("Show/Hide Widget", true, None);
        let settings_item = MenuItem::new("Settings", true, None);
        let quit_item = MenuItem::new("Quit", true, None);

        menu.append(&toggle_item).expect("failed to append tray menu item");
        menu.append(&settings_item).expect("failed to append tray menu item");
        menu.append(&quit_item).expect("failed to append tray menu item");

        let toggle_id = toggle_item.id().clone();
        let settings_id = settings_item.id().clone();
        let quit_id = quit_item.id().clone();

        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("Spotify Widget")
            .with_icon(icon)
            .build()
            .expect("failed to build tray icon");

        Self {
            _tray_icon: tray_icon,
            toggle_id,
            settings_id,
            quit_id,
        }
    }

    pub fn poll(&self) -> TrayCommand {
        while let Ok(event) = MenuEvent::receiver().try_recv() {
            if event.id == self.toggle_id {
                return TrayCommand::ToggleVisibility;
            }
            if event.id == self.settings_id {
                return TrayCommand::OpenSettings;
            }
            if event.id == self.quit_id {
                return TrayCommand::Quit;
            }
        }
        TrayCommand::None
    }
}

/// Builds a flat-color placeholder icon (32x32, Spotify green) so we
/// don't depend on an image asset yet. Swap this out for a real
/// `.ico`/`.png` loaded via `include_bytes!` once you have one.
fn build_app_icon() -> Icon {
    const SIZE: u32 = 32;
    let rgba = rust_windows_spotify_widget::icon::generate_icon_rgba(SIZE);
    Icon::from_rgba(rgba, SIZE, SIZE).expect("failed to build tray icon from rgba data")
}