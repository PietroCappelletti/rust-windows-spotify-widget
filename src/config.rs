use std::env;

pub struct Config {
    pub spotify_client_id: String,
    pub spotify_client_secret: String,
    pub spotify_redirect_uri: String,
    pub auto_hide_seconds: u64,
    pub hotkey_combo: String,
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

        let hotkey_combo = env::var("HOTKEY_COMBO")
            .unwrap_or_else(|_| "ctrl+shift+period".to_string());

        Ok(Self {
            spotify_client_id,
            spotify_client_secret,
            spotify_redirect_uri,
            auto_hide_seconds,
            hotkey_combo,
        })
    }
}