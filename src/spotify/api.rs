use super::models::{PlaybackState, PlaybackStateResponse};

const BASE_URL: &str = "https://api.spotify.com/v1/me/player";

fn player_url(path: &str) -> String {
    format!("{BASE_URL}{path}")
}

async fn send_player_command(
    client: &reqwest::Client,
    access_token: &str,
    method: reqwest::Method,
    path: &str,
) -> Result<(), String> {
    let response = client
        .request(method, player_url(path))
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| format!("request failed: {e}"))?;

    match response.status().as_u16() {
        200..=299 => Ok(()),
        404 => Err("no active Spotify device found — open Spotify and start playing something first".to_string()),
        403 => Err("Spotify Premium is required for playback control".to_string()),
        401 => Err("access token expired or invalid — needs refresh".to_string()),
        429 => Err("Spotify API rate limit hit — slow down a bit and try again".to_string()),
        code => Err(format!("Spotify API returned unexpected status {code}")),
    }
}

pub async fn play(client: &reqwest::Client, access_token: &str) -> Result<(), String> {
    send_player_command(client, access_token, reqwest::Method::PUT, "/play").await
}

pub async fn pause(client: &reqwest::Client, access_token: &str) -> Result<(), String> {
    send_player_command(client, access_token, reqwest::Method::PUT, "/pause").await
}

pub async fn next(client: &reqwest::Client, access_token: &str) -> Result<(), String> {
    send_player_command(client, access_token, reqwest::Method::POST, "/next").await
}

pub async fn previous(client: &reqwest::Client, access_token: &str) -> Result<(), String> {
    send_player_command(client, access_token, reqwest::Method::POST, "/previous").await
}

pub async fn set_volume(client: &reqwest::Client, access_token: &str, volume_percent: u8) -> Result<(), String> {
    let clamped = volume_percent.min(100);
    let path = format!("/volume?volume_percent={clamped}");
    send_player_command(client, access_token, reqwest::Method::PUT, &path).await
}

pub async fn playback_state(client: &reqwest::Client, access_token: &str) -> Result<PlaybackState, String> {
    let response = client
        .get(player_url(""))
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| format!("request failed: {e}"))?;

    match response.status().as_u16() {
        204 => Ok(PlaybackState::default()),
        401 => Err("access token expired or invalid — needs refresh".to_string()),
        429 => Err("Spotify API rate limit hit — slow down a bit and try again".to_string()),
        200..=299 => {
            let parsed: PlaybackStateResponse = response
                .json()
                .await
                .map_err(|e| format!("failed to parse response: {e}"))?;
            Ok(parsed.into())
        }
        code => Err(format!("Spotify API returned unexpected status {code}")),
    }
}