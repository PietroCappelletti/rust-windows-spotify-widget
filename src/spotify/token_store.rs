use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, UNIX_EPOCH};

use super::auth::AuthTokens;

#[derive(Debug, Serialize, Deserialize)]
struct StoredTokens {
    access_token: String,
    refresh_token: Option<String>,
    expires_at_unix: u64,
}

fn token_file_path() -> Result<PathBuf, String> {
    let proj_dirs = directories::ProjectDirs::from("dev", "PietroCappelletti", "rust-windows-spotify-widget")
        .ok_or_else(|| "could not determine app data directory".to_string())?;
    let dir = proj_dirs.data_dir();
    fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    Ok(dir.join("tokens.json"))
}

pub fn load() -> Option<AuthTokens> {
    let path = token_file_path().ok()?;
    let contents = fs::read_to_string(path).ok()?;
    let stored: StoredTokens = serde_json::from_str(&contents).ok()?;
    Some(AuthTokens {
        access_token: stored.access_token,
        refresh_token: stored.refresh_token,
        expires_at: UNIX_EPOCH + Duration::from_secs(stored.expires_at_unix),
    })
}

pub fn save(tokens: &AuthTokens) -> Result<(), String> {
    let path = token_file_path()?;
    let expires_at_unix = tokens
        .expires_at
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let stored = StoredTokens {
        access_token: tokens.access_token.clone(),
        refresh_token: tokens.refresh_token.clone(),
        expires_at_unix,
    };
    let json = serde_json::to_string_pretty(&stored).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}