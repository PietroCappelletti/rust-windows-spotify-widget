use super::models::Track;

/// Raw HTTP calls to the Spotify Web API. Called by `SpotifyClient`.
pub async fn play(access_token: &str) -> Result<(), String> {
  todo!()
}

pub async fn pause(access_token: &str) -> Result<(), String> {
  todo!()
}

pub async fn next(access_token: &str) -> Result<(), String> {
  todo!()
}

pub async fn previous(access_token: &str) -> Result<(), String> {
  todo!()
}

pub async fn current_playback(access_token: &str) -> Result<Option<Track>, String> {
  todo!()
}