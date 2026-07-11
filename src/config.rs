use std::env;

/// Application configuration loaded from `.env` / environment variables.
pub struct Config {
  pub spotify_client_id: String,
  pub spotify_client_secret: String,
  pub spotify_redirect_uri: String,
  pub auto_hide_seconds: u64,
}

impl Config {
  /// Loads configuration from `.env` (if present) and environment variables.
  /// Panics with a clear message if required Spotify credentials are missing.
  pub fn load() -> Self {
    // Loads variables from a `.env` file in the working directory, if present.
    // Ignored (via `.ok()`) so this doesn't panic in case `.env` is missing —
    // e.g. if you later set real environment variables instead.
    dotenvy::dotenv().ok();

    let spotify_client_id = env::var("SPOTIFY_CLIENT_ID")
      .expect("Missing SPOTIFY_CLIENT_ID — check your .env file");

    let spotify_client_secret = env::var("SPOTIFY_CLIENT_SECRET")
      .expect("Missing SPOTIFY_CLIENT_SECRET — check your .env file");

    let spotify_redirect_uri = env::var("SPOTIFY_REDIRECT_URI")
      .unwrap_or_else(|_| "http://127.0.0.1:8888/callback".to_string());

    let auto_hide_seconds = env::var("AUTO_HIDE_SECONDS")
      .ok()
      .and_then(|v| v.parse().ok())
      .unwrap_or(6);

    Self {
      spotify_client_id,
      spotify_client_secret,
      spotify_redirect_uri,
      auto_hide_seconds,
    }
  }
}