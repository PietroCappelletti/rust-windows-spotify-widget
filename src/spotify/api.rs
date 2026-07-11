use super::models::{CurrentlyPlayingResponse, Track};

const BASE_URL: &str = "https://api.spotify.com/v1/me/player";

/// Shared helper: builds an authorized request to a player endpoint.
fn player_url(path: &str) -> String {
  format!("{BASE_URL}{path}")
}

async fn send_player_command(access_token: &str, method: reqwest::Method, path: &str) -> Result<(), String> {
  let client = reqwest::Client::new();

  let response = client
    .request(method, player_url(path))
    .bearer_auth(access_token)
    .send()
    .await
    .map_err(|e| format!("request failed: {e}"))?;

  // Spotify returns 204 No Content on success for these endpoints.
  // 403/404 usually mean "no active device" — surface that clearly.
  match response.status().as_u16() {
    200..=299 => Ok(()),
    404 => Err("no active Spotify device found — open Spotify and start playing something first".to_string()),
    403 => Err("Spotify Premium is required for playback control".to_string()),
    401 => Err("access token expired or invalid — needs refresh".to_string()),
    code => Err(format!("Spotify API returned unexpected status {code}")),
  }
}

pub async fn play(access_token: &str) -> Result<(), String> {
  send_player_command(access_token, reqwest::Method::PUT, "/play").await
}

pub async fn pause(access_token: &str) -> Result<(), String> {
  send_player_command(access_token, reqwest::Method::PUT, "/pause").await
}

pub async fn next(access_token: &str) -> Result<(), String> {
  send_player_command(access_token, reqwest::Method::POST, "/next").await
}

pub async fn previous(access_token: &str) -> Result<(), String> {
  send_player_command(access_token, reqwest::Method::POST, "/previous").await
}

pub async fn current_playback(access_token: &str) -> Result<Option<Track>, String> {
  let client = reqwest::Client::new();

  let response = client
    .get(player_url("/currently-playing"))
    .bearer_auth(access_token)
    .send()
    .await
    .map_err(|e| format!("request failed: {e}"))?;

  match response.status().as_u16() {
    // 204 = nothing currently playing (e.g. paused with no context, or fully stopped)
    204 => Ok(None),
    401 => Err("access token expired or invalid — needs refresh".to_string()),
    200..=299 => {
      let parsed: CurrentlyPlayingResponse = response
        .json()
        .await
        .map_err(|e| format!("failed to parse response: {e}"))?;
      Ok(parsed.into())
    }
    code => Err(format!("Spotify API returned unexpected status {code}")),
  }
}