use serde::Deserialize;

/// A currently playing (or paused) Spotify track.
#[derive(Debug, Clone, Deserialize)]
pub struct Track {
  pub name: String,
  pub artist: String,
  pub album_art_url: Option<String>,
  pub is_playing: bool,
}