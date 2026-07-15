use std::collections::HashMap;
use global_hotkey::hotkey::HotKey;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};

const DEFAULT_TOGGLE_COMBO: &str = "ctrl+shift+period";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HotkeyAction {
    ToggleVisibility,
    PlayPause,
    Next,
    Previous,
}

impl HotkeyAction {
    pub const ALL: [HotkeyAction; 4] = [
        HotkeyAction::ToggleVisibility,
        HotkeyAction::PlayPause,
        HotkeyAction::Next,
        HotkeyAction::Previous,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            HotkeyAction::ToggleVisibility => "Show/Hide widget",
            HotkeyAction::PlayPause => "Play/Pause",
            HotkeyAction::Next => "Next track",
            HotkeyAction::Previous => "Previous track",
        }
    }

    pub fn env_key(&self) -> &'static str {
        match self {
            HotkeyAction::ToggleVisibility => "HOTKEY_COMBO",
            HotkeyAction::PlayPause => "HOTKEY_PLAY_PAUSE",
            HotkeyAction::Next => "HOTKEY_NEXT",
            HotkeyAction::Previous => "HOTKEY_PREVIOUS",
        }
    }

    /// Whether this action must always have a binding (the toggle can't
    /// be left unbound, or you'd have no way to show the widget at all).
    pub fn required(&self) -> bool {
        matches!(self, HotkeyAction::ToggleVisibility)
    }
}

struct Binding {
    hotkey: HotKey,
    combo: String,
}

/// Manages an arbitrary set of independent global hotkeys, each mapped to
/// an action. Actions other than ToggleVisibility are optional — leaving
/// their combo unset (empty string / None) just means that shortcut isn't
/// registered at all.
pub struct HotkeyListener {
    manager: GlobalHotKeyManager,
    bindings: HashMap<HotkeyAction, Binding>,
    id_to_action: HashMap<u32, HotkeyAction>,
}

impl HotkeyListener {
    pub fn new(combos: &HashMap<HotkeyAction, Option<String>>) -> Self {
        let manager = GlobalHotKeyManager::new().expect("failed to create hotkey manager");
        let mut bindings = HashMap::new();
        let mut id_to_action = HashMap::new();

        for action in HotkeyAction::ALL {
            let requested = combos.get(&action).cloned().flatten();
            let combo = match requested {
                Some(c) if !c.trim().is_empty() => c,
                _ => continue, // left unbound
            };

            match Self::try_register(&manager, &combo) {
                Ok(hotkey) => {
                    eprintln!("[hotkey] {:?} -> '{combo}' (id={})", action, hotkey.id());
                    id_to_action.insert(hotkey.id(), action);
                    bindings.insert(action, Binding { hotkey, combo });
                }
                Err(e) => {
                    eprintln!("[hotkey] Failed to register {:?} combo '{combo}': {e}", action);
                }
            }
        }

        // Guarantee the toggle always has *some* working binding, even if
        // config was missing/invalid — otherwise there'd be no way to
        // ever show the widget.
        if !bindings.contains_key(&HotkeyAction::ToggleVisibility) {
            if let Ok(hotkey) = Self::try_register(&manager, DEFAULT_TOGGLE_COMBO) {
                eprintln!("[hotkey] Falling back to default toggle '{DEFAULT_TOGGLE_COMBO}'");
                id_to_action.insert(hotkey.id(), HotkeyAction::ToggleVisibility);
                bindings.insert(
                    HotkeyAction::ToggleVisibility,
                    Binding { hotkey, combo: DEFAULT_TOGGLE_COMBO.to_string() },
                );
            }
        }

        Self { manager, bindings, id_to_action }
    }

    fn try_register(manager: &GlobalHotKeyManager, combo: &str) -> Result<HotKey, String> {
        let hotkey: HotKey = combo.parse().map_err(|_| format!("'{combo}' isn't a recognized key combination"))?;
        manager.register(hotkey).map_err(|e| e.to_string())?;
        Ok(hotkey)
    }

    /// Drains pending hotkey events, returning which actions were
    /// triggered since the last call (there can be more than one per
    /// frame if multiple keys were pressed close together).
    pub fn poll_actions(&self) -> Vec<HotkeyAction> {
        let mut actions = Vec::new();
        while let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            if event.state == HotKeyState::Pressed {
                if let Some(action) = self.id_to_action.get(&event.id) {
                    actions.push(*action);
                }
            }
        }
        actions
    }

    pub fn combo_for(&self, action: HotkeyAction) -> Option<&str> {
        self.bindings.get(&action).map(|b| b.combo.as_str())
    }

    /// Rebinds `action` to `new_combo`. Passing an empty string unbinds
    /// it (not allowed for `ToggleVisibility`, which must stay bound).
    pub fn update_combo(&mut self, action: HotkeyAction, new_combo: &str) -> Result<(), String> {
        if action.required() && new_combo.trim().is_empty() {
            return Err("Show/Hide widget must have a hotkey — it can't be cleared.".to_string());
        }

        if let Some(old) = self.bindings.remove(&action) {
            self.id_to_action.remove(&old.hotkey.id());
            let _ = self.manager.unregister(old.hotkey);
        }

        if new_combo.trim().is_empty() {
            return Ok(()); // now unbound
        }

        match Self::try_register(&self.manager, new_combo) {
            Ok(hotkey) => {
                self.id_to_action.insert(hotkey.id(), action);
                self.bindings.insert(action, Binding { hotkey, combo: new_combo.to_string() });
                eprintln!("[hotkey] {:?} -> '{new_combo}' (id={})", action, hotkey.id());
                Ok(())
            }
            Err(e) => Err(format!(
                "failed to register '{new_combo}': {e} (it may already be used by another app or action)"
            )),
        }
    }
}