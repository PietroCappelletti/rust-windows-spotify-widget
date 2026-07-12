# rust-windows-spotify-widget

A lightweight Windows desktop widget, written in Rust, to view and control Spotify playback without switching to the Spotify app.

## Features

- ▶️ Play / Pause
- ⏭️ Skip to next track
- ⏮️ Skip to previous track
- 👁️ Toggle visibility with a configurable global hotkey (default `Ctrl+Shift+.`) — auto-hides after a few seconds of inactivity
- 📍 Appears in the top-right corner of the screen
- 🎵 Live track info with a scrolling marquee for long titles, plus album art
- 🖥️ System tray icon (custom design) with Show/Hide and Quit
- 🔒 Persisted login — authorize once via browser, silently reused on future launches, with automatic token refresh
- 🪟 Clean release build — no console window, friendly error dialogs instead of crashes
- 🧍 Single-instance protection — launching twice shows a message instead of failing silently
- 📦 Windows installer with optional desktop shortcut and launch-at-startup

## Why?

I wanted a minimal, always-on-top widget to control Spotify from my desktop without alt-tabbing or opening the full app — and a good excuse to write something real in Rust.

## Requirements

- Windows 10/11
- A Spotify Premium account (required for playback control via the Spotify Web API)
- A registered Spotify Developer App ([create one here](https://developer.spotify.com/dashboard)) to get a Client ID/Secret

## Installation

### Option 1: Installer (recommended)

1. Download `SpotifyWidgetSetup.exe` from the [Releases](https://github.com/PietroCappelletti/rust-windows-spotify-widget/releases) page (once published).
2. Run it and follow the setup wizard — you can optionally create a desktop shortcut and enable launch-at-Windows-startup.
3. Continue to [Setup](#setup) below to configure your Spotify credentials.

### Option 2: From source

```bash
git clone https://github.com/PietroCappelletti/rust-windows-spotify-widget.git
cd rust-windows-spotify-widget
cargo build --release
```

The compiled binary will be in `target/release/`.

## Setup

1. Create a Spotify Developer App at the [Spotify Dashboard](https://developer.spotify.com/dashboard).
2. Copy your `Client ID` and `Client Secret`.
3. In the app's settings on the Spotify Dashboard, add this exact Redirect URI: `http://127.0.0.1:8888/callback`
4. In the app's install folder (or project root if running from source), copy `.env.example` to `.env` and fill in your values:

```
SPOTIFY_CLIENT_ID=your_client_id
SPOTIFY_CLIENT_SECRET=your_client_secret
SPOTIFY_REDIRECT_URI=http://127.0.0.1:8888/callback
HOTKEY_COMBO=ctrl+shift+period
```

5. Launch the app. On first launch, it opens your browser to log in and authorize access. After that, your session is stored locally and reused automatically — no repeated logins, and tokens refresh automatically in the background.

If `.env` is missing or incomplete, you'll see a clear error dialog explaining what to fix instead of a silent crash.

## Usage

- The widget starts hidden, with only a tray icon visible.
- Press your configured hotkey (default **`Ctrl+Shift+.`**) to show/hide it — it auto-hides after a few seconds of inactivity.
- Right-click the tray icon for **Show/Hide Widget** and **Quit**.
- Use the ⏮ ▶/⏸ ⏭ buttons to control playback; track name, artist, and album art update live.

**Note:** playback control requires an active Spotify session on some device (phone, desktop app, web player) — start playing something in Spotify first if you see a "no active device" message.

### Configuring the hotkey

Set `HOTKEY_COMBO` in `.env` using modifiers (`ctrl`, `shift`, `alt`, `meta`) joined with `+`, followed by a key code — letters are `KeyA`–`KeyZ`, digits are `Digit0`–`Digit9`, and punctuation has names like `Period`, `Comma`, `Slash`. Example: `HOTKEY_COMBO=ctrl+alt+KeyS` for Ctrl+Alt+S. An invalid combo falls back to the default automatically.

## Built with

- [Rust](https://www.rust-lang.org/)
- [egui](https://github.com/emilk/egui) / [eframe](https://github.com/emilk/egui) — immediate-mode GUI
- [tray-icon](https://crates.io/crates/tray-icon) — system tray icon and menu
- [global-hotkey](https://crates.io/crates/global-hotkey) — OS-level hotkey registration
- [oauth2](https://crates.io/crates/oauth2) — Spotify OAuth (Authorization Code + PKCE)
- [reqwest](https://crates.io/crates/reqwest) — HTTP client for the Spotify Web API
- [tokio](https://crates.io/crates/tokio) — async runtime
- [tiny_http](https://crates.io/crates/tiny_http) — local server to catch the OAuth redirect
- [dotenvy](https://crates.io/crates/dotenvy) — `.env` config loading
- [directories](https://crates.io/crates/directories) — cross-platform app-data paths for token storage
- [image](https://crates.io/crates/image) — album art decoding
- [ico](https://crates.io/crates/ico) — icon file generation
- [winresource](https://crates.io/crates/winresource) — embeds the app icon into the `.exe`
- [rfd](https://crates.io/crates/rfd) — native error dialogs
- [single-instance](https://crates.io/crates/single-instance) — prevents duplicate launches
- [Inno Setup](https://jrsoftware.org/isinfo.php) — Windows installer

## Roadmap

- [x] Basic playback controls (play/pause/skip/previous)
- [x] Show/hide toggle via global hotkey
- [x] System tray integration
- [x] Spotify OAuth login with persisted sessions
- [x] Automatic access token refresh
- [x] Album art display
- [x] Reposition widget to top-right corner
- [x] Custom tray + app icon art (procedurally generated, embedded in `.exe`)
- [x] Configurable hotkey
- [x] Windows installer with optional desktop shortcut and startup launch
- [x] Console-free release build with friendly error dialogs
- [x] Single-instance protection

Possible future ideas: settings UI instead of editing `.env` directly, volume control, prebuilt binary releases on GitHub.

## Contributing

This is currently a personal project, but suggestions and issues are welcome.

## License

This project is licensed under the [MIT License](LICENSE).
