# rust-windows-spotify-widget

A lightweight Windows desktop widget, written in Rust, to view and control Spotify playback without switching to the Spotify app.

## Features

- ▶️ Play / Pause
- ⏭️ Skip to next track
- ⏮️ Skip to previous track
- 👁️ Toggle visibility with a global hotkey (`Ctrl+Shift+.`) — auto-hides after a few seconds of inactivity
- 🎵 View current track info (song title, artist)
- 🖥️ System tray icon with Show/Hide and Quit
- 🔒 Persisted login — authorize once via browser, silently reused on future launches

> Album art display, custom tray icon art, and repositioning are in progress — see [Roadmap](#roadmap).

## Why?

I wanted a minimal, always-on-top widget to control Spotify from my desktop without alt-tabbing or opening the full app — and a good excuse to write something real in Rust.

## Requirements

- Windows 10/11
- A Spotify Premium account (required for playback control via the Spotify Web API)
- A registered Spotify Developer App ([create one here](https://developer.spotify.com/dashboard)) to get a Client ID/Secret

## Installation

### From source

```bash
git clone https://github.com/PietroCappelletti/rust-windows-spotify-widget.git
cd rust-windows-spotify-widget
cargo build --release
```

The compiled binary will be in `target/release/`.

### Prebuilt binary

_(Coming soon — once the project reaches a stable release.)_

## Setup

1. Create a Spotify Developer App at the [Spotify Dashboard](https://developer.spotify.com/dashboard).
2. Copy your `Client ID` and `Client Secret`.
3. In the app's settings on the Spotify Dashboard, add this exact Redirect URI: `http://127.0.0.1:8888/callback`
4. Create a `.env` file in the project root:

```
SPOTIFY_CLIENT_ID=your_client_id
SPOTIFY_CLIENT_SECRET=your_client_secret
SPOTIFY_REDIRECT_URI=http://127.0.0.1:8888/callback
```

5. Run the app. On first launch, it opens your browser to log in and authorize access. After that, your session is stored locally and reused automatically — no repeated logins.

## Usage

```bash
cargo run --release
```

- The widget starts hidden, with only a tray icon visible.
- Press **`Ctrl+Shift+.`** to show/hide it — it auto-hides after a few seconds of inactivity.
- Right-click the tray icon for **Show/Hide Widget** and **Quit**.
- Use the ⏮ ▶/⏸ ⏭ buttons in the widget to control playback.

**Note:** playback control requires an active Spotify session on some device (phone, desktop app, web player) — start playing something in Spotify first if you see a "no active device" message.

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

## Roadmap

- [x] Basic playback controls (play/pause/skip/previous)
- [x] Show/hide toggle via global hotkey
- [x] System tray integration
- [x] Spotify OAuth login with persisted sessions
- [x] Album art display
- [x] Reposition widget to top-right corner of the screen
- [ ] Custom tray + app icon art
- [ ] Configurable hotkey (currently hardcoded to `Ctrl+Shift+.`)
- [ ] Automatic access token refresh on expiry (currently only refreshes on relaunch)
- [ ] Prebuilt Windows installer

## Contributing

This is currently a personal project, but suggestions and issues are welcome once it's further along.

## License

This project is licensed under the [MIT License](LICENSE).
