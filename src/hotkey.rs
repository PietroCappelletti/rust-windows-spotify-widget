use global_hotkey::hotkey::HotKey;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};

const DEFAULT_HOTKEY_COMBO: &str = "ctrl+shift+period";

/// Registers a global hotkey and exposes whether it was just pressed.
/// Must be kept alive for the hotkey to remain registered — dropping it
/// unregisters everything.
pub struct HotkeyListener {
    _manager: GlobalHotKeyManager,
    hotkey_id: u32,
}

impl HotkeyListener {
    /// Parses `combo` (e.g. "ctrl+shift+period") and registers it as a
    /// global hotkey. Falls back to Ctrl+Shift+. if `combo` is invalid,
    /// so a typo in the user's config can't crash the app on startup.
    pub fn new(combo: &str) -> Self {
        let manager = GlobalHotKeyManager::new().expect("failed to create hotkey manager");

        let hotkey: HotKey = combo.parse().unwrap_or_else(|_| {
            eprintln!(
                "[hotkey] Couldn't parse HOTKEY_COMBO '{combo}', falling back to '{DEFAULT_HOTKEY_COMBO}'"
            );
            DEFAULT_HOTKEY_COMBO
                .parse()
                .expect("default hotkey combo must always be valid")
        });

        manager
            .register(hotkey)
            .expect("failed to register global hotkey");

        eprintln!("[hotkey] Registered '{combo}' successfully (id={})", hotkey.id());

        Self {
            _manager: manager,
            hotkey_id: hotkey.id(),
        }
    }

    /// Returns true if this hotkey was pressed since the last check.
    pub fn was_pressed(&self) -> bool {
        let mut pressed = false;
        while let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            eprintln!("[hotkey] Received event: id={}, state={:?}", event.id, event.state);
            if event.id == self.hotkey_id && event.state == HotKeyState::Pressed {
                pressed = true;
            }
        }
        pressed
    }
}