# rust-windows-spotify-widget

A lightweight Windows desktop widget, written in Rust, to view and control Spotify playback without switching to the Spotify app.

## Features

- ▶️ Play / Pause
- ⏭️ Skip to next track
- ⏮️ Skip to previous track
- 👁️ Show / Hide the widget
- 🎵 View current track info (song title, artist, album art)

> More features may be added over time — see [Roadmap](#roadmap).

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

1.  Create a Spotify Developer App at the [Spotify Dashboard](https://developer.spotify.com/dashboard).
2.  Copy your `Client ID` and `Client Secret`.
3.  Create a `.env` file in the project root:
    ```
    SPOTIFY_CLIENT_ID=your_client_id
    SPOTIFY_CLIENT_SECRET=your_client_secret
    ```
4.  Run the app — on first launch it will prompt you to authorize via your browser.

## Usage

```bash
cargo run --release
```

The widget will appear on your desktop. [Add keybindings / tray icon behavior here once implemented.]

## Built with

- [Rust](https://www.rust-lang.org/)
- _(GUI crate — e.g. egui/eframe, TBD)_
- [reqwest](https://crates.io/crates/reqwest) — HTTP client for the Spotify Web API
- [tokio](https://crates.io/crates/tokio) — async runtime

## Roadmap

- [ ] Basic playback controls (play/pause/skip/previous)
- [ ] Show/hide toggle
- [ ] System tray integration
- [ ] Album art & track info display
- [ ] Configurable hotkeys
- [ ] Prebuilt Windows installer

## Contributing

This is currently a personal project, but suggestions and issues are welcome once it's further along.

## License

This project is licensed under the [MIT License](LICENSE).
