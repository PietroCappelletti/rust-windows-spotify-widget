use global_hotkey::{
  hotkey::{Code, HotKey, Modifiers},
  GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};

/// Registers the global Ctrl+Shift+. hotkey and lets the app check
/// whether it was just pressed. Works even when the app window is
/// hidden or unfocused, since this hooks in at the OS level.
pub struct HotkeyListener {
  _manager: GlobalHotKeyManager,
  hotkey_id: u32,
}

impl HotkeyListener {
  pub fn new() -> Self {
    let manager =
      GlobalHotKeyManager::new().expect("failed to create global hotkey manager");

    let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::Period);
    let hotkey_id = hotkey.id();

    match manager.register(hotkey) {
      Ok(_) => eprintln!("[hotkey] Registered Ctrl+Shift+. successfully (id={})", hotkey_id),
      Err(e) => eprintln!("[hotkey] FAILED to register: {:?}", e),
    }

    Self {
      _manager: manager,
      hotkey_id,
    }
  }

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