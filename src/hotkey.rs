/// Registers the global hotkey (e.g. Ctrl+Shift+.) and notifies
/// the app when it's pressed.
pub struct HotkeyListener;

impl HotkeyListener {
  /// Sets up the global hotkey and returns a listener handle.
  pub fn new() -> Self {
    todo!("register global hotkey with the `global-hotkey` crate")
  }

  /// Returns true if the hotkey was just pressed (called each frame).
  pub fn was_pressed(&self) -> bool {
    todo!("check hotkey event queue")
  }
}