use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Track {
    pub name: String,
    pub artist: String,
    pub album_art_url: Option<String>,
    pub is_playing: bool,
}

/// Track info plus device volume — everything the widget needs, from a
/// single call to Spotify's playback-state endpoint.
#[derive(Debug, Clone, Default)]
pub struct PlaybackState {
    pub track: Option<Track>,
    pub volume_percent: Option<u8>,
}

// --- Raw Spotify API response shapes ---

#[derive(Debug, Deserialize)]
pub(super) struct PlaybackStateResponse {
    pub is_playing: bool,
    pub item: Option<TrackItem>,
    pub device: Option<DeviceItem>,
}

#[derive(Debug, Deserialize)]
pub(super) struct DeviceItem {
    pub volume_percent: Option<u8>,
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

impl From<PlaybackStateResponse> for PlaybackState {
    fn from(resp: PlaybackStateResponse) -> Self {
        let track = resp.item.map(|item| Track {
            name: item.name,
            artist: item
                .artists
                .first()
                .map(|a| a.name.clone())
                .unwrap_or_else(|| "Unknown artist".to_string()),
            album_art_url: item.album.images.first().map(|i| i.url.clone()),
            is_playing: resp.is_playing,
        });

        PlaybackState {
            track,
            volume_percent: resp.device.and_then(|d| d.volume_percent),
        }
    }
}