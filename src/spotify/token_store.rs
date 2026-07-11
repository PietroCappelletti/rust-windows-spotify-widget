use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use super::auth::AuthTokens;

#[derive(Debug, Serialize, Deserialize)]
struct StoredTokens {
  access_token: String,
  refresh_token: Option<String>,
}

fn token_file_path() -> Result<PathBuf, String> {
  let proj_dirs = directories::ProjectDirs::from("dev", "PietroCappelletti", "rust-windows-spotify-widget")
    .ok_or_else(|| "could not determine app data directory".to_string())?;
  let dir = proj_dirs.data_dir();
  fs::create_dir_all(dir).map_err(|e| e.to_string())?;
  Ok(dir.join("tokens.json"))
}

/// Loads previously saved tokens, if any exist on disk.
pub fn load() -> Option<AuthTokens> {
  let path = token_file_path().ok()?;
  let contents = fs::read_to_string(path).ok()?;
  let stored: StoredTokens = serde_json::from_str(&contents).ok()?;
  Some(AuthTokens {
    access_token: stored.access_token,
    refresh_token: stored.refresh_token,
  })
}

/// Persists tokens to disk so future launches can skip the browser login.
pub fn save(tokens: &AuthTokens) -> Result<(), String> {
  let path = token_file_path()?;
  let stored = StoredTokens {
    access_token: tokens.access_token.clone(),
    refresh_token: tokens.refresh_token.clone(),
  };
  let json = serde_json::to_string_pretty(&stored).map_err(|e| e.to_string())?;
  fs::write(path, json).map_err(|e| e.to_string())
}