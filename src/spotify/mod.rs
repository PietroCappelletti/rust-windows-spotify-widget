mod auth;
mod api;
mod models;

pub use models::Track;

/// High-level client for controlling Spotify playback.
pub struct SpotifyClient {
  // TODO: hold access token, refresh token, http client
}

impl SpotifyClient {
  pub fn new(client_id: String, client_secret: String) -> Self {
    todo!()
  }

  pub async fn play(&self) -> Result<(), String> {
    todo!()
  }

  pub async fn pause(&self) -> Result<(), String> {
    todo!()
  }

  pub async fn next_track(&self) -> Result<(), String> {
    todo!()
  }

  pub async fn previous_track(&self) -> Result<(), String> {
    todo!()
  }

  pub async fn get_current_track(&self) -> Result<Option<Track>, String> {
    todo!()
  }
}