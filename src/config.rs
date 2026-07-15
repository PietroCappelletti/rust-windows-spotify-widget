use std::env;
use std::fs;
use crate::hotkey::HotkeyAction;
use std::collections::HashMap;

pub struct Config {
    pub spotify_client_id: String,
    pub spotify_client_secret: String,
    pub spotify_redirect_uri: String,
    pub auto_hide_seconds: u64,
    pub hotkey_combos: HashMap<HotkeyAction, Option<String>>,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        dotenvy::dotenv().ok();

        let spotify_client_id = env::var("SPOTIFY_CLIENT_ID").map_err(|_| {
            "Missing SPOTIFY_CLIENT_ID.\n\nCopy .env.example to .env in the app's install folder and fill in your Spotify Client ID and Secret. See README.md for full setup steps.".to_string()
        })?;

        let spotify_client_secret = env::var("SPOTIFY_CLIENT_SECRET").map_err(|_| {
            "Missing SPOTIFY_CLIENT_SECRET.\n\nCopy .env.example to .env in the app's install folder and fill in your Spotify Client ID and Secret. See README.md for full setup steps.".to_string()
        })?;

        let spotify_redirect_uri = env::var("SPOTIFY_REDIRECT_URI")
            .unwrap_or_else(|_| "http://127.0.0.1:8888/callback".to_string());

        let auto_hide_seconds = env::var("AUTO_HIDE_SECONDS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(6);

        let mut hotkey_combos = HashMap::new();
        hotkey_combos.insert(
            HotkeyAction::ToggleVisibility,
            Some(env::var("HOTKEY_COMBO").unwrap_or_else(|_| "ctrl+shift+period".to_string())),
        );
        hotkey_combos.insert(HotkeyAction::PlayPause, env::var("HOTKEY_PLAY_PAUSE").ok());
        hotkey_combos.insert(HotkeyAction::Next, env::var("HOTKEY_NEXT").ok());
        hotkey_combos.insert(HotkeyAction::Previous, env::var("HOTKEY_PREVIOUS").ok());

        Ok(Self {
            spotify_client_id,
            spotify_client_secret,
            spotify_redirect_uri,
            auto_hide_seconds,
            hotkey_combos,
        })
    }
}

/// Updates (or adds) a single KEY=value line in the local `.env` file.
fn save_env_var(key: &str, value: &str) -> Result<(), String> {
    let path = ".env";
    let existing = fs::read_to_string(path).unwrap_or_default();

    let mut found = false;
    let mut new_lines: Vec<String> = existing
        .lines()
        .map(|line| {
            if line.trim_start().starts_with(&format!("{key}=")) {
                found = true;
                format!("{key}={value}")
            } else {
                line.to_string()
            }
        })
        .collect();

    if !found {
        new_lines.push(format!("{key}={value}"));
    }

    fs::write(path, new_lines.join("\n") + "\n").map_err(|e| e.to_string())
}

pub fn save_hotkey_combo(action: HotkeyAction, combo: &str) -> Result<(), String> {
    save_env_var(action.env_key(), combo)
}