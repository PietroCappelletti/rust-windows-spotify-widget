use serde::Deserialize;

/// A currently playing (or paused) Spotify track, simplified for our UI.
#[derive(Debug, Clone)]
pub struct Track {
  pub name: String,
  pub artist: String,
  pub album_art_url: Option<String>,
  pub is_playing: bool,
}

// --- Raw Spotify API response shapes (used only for deserialization) ---

#[derive(Debug, Deserialize)]
pub(super) struct CurrentlyPlayingResponse {
  pub is_playing: bool,
  pub item: Option<TrackItem>,
}

#[derive(Debug, Deserialize)]
pub(super) struct TrackItem {
  pub name: String,
  pub artists: Vec<ArtistItem>,
  pub album: AlbumItem,
}

#[derive(Debug, Deserialize)]
pub(super) struct ArtistItem {
  pub name: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct AlbumItem {
  pub images: Vec<ImageItem>,
}

#[derive(Debug, Deserialize)]
pub(super) struct ImageItem {
  pub url: String,
}

impl From<CurrentlyPlayingResponse> for Option<Track> {
  fn from(resp: CurrentlyPlayingResponse) -> Self {
    let item = resp.item?;
    Some(Track {
      name: item.name,
      artist: item
        .artists
        .first()
        .map(|a| a.name.clone())
        .unwrap_or_else(|| "Unknown artist".to_string()),
      album_art_url: item.album.images.first().map(|i| i.url.clone()),
      is_playing: resp.is_playing,
    })
  }
}