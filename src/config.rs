/// Application configuration loaded from `.env` / environment variables.
pub struct Config {
  pub spotify_client_id: String,
  pub spotify_client_secret: String,
  pub auto_hide_seconds: u64,
}

impl Config {
  /// Loads configuration from environment variables (via `.env`).
  pub fn load() -> Self {
    todo!("read SPOTIFY_CLIENT_ID / SPOTIFY_CLIENT_SECRET from .env")
  }
}