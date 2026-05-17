use serde::{Deserialize, Serialize};

// ── OAuth ────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
}

/// Stored on disk at ~/.gurdo/spotify_token.json
#[derive(Debug, Deserialize, Serialize)]
pub struct StoredToken {
    pub access_token: String,
    pub refresh_token: String,
    /// Unix timestamp after which the access token should be refreshed
    pub expires_at: i64,
}

// ── Search ───────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub tracks: SearchTracks,
}

#[derive(Debug, Deserialize)]
pub struct SearchTracks {
    pub items: Vec<TrackItem>,
}

#[derive(Debug, Deserialize)]
pub struct TrackItem {
    pub id: Option<String>,
    pub uri: String,
    pub name: String,
    pub duration_ms: Option<u64>,
    pub artists: Vec<ArtistItem>,
    pub album: AlbumItem,
}

#[derive(Debug, Deserialize)]
pub struct ArtistItem {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct AlbumItem {
    pub name: String,
    pub images: Vec<AlbumImage>,
}

#[derive(Debug, Deserialize)]
pub struct AlbumImage {
    pub url: String,
    pub width: Option<u32>,
}

impl TrackItem {
    pub fn best_image_url(&self) -> Option<&str> {
        // Prefer 300px image; fall back to largest available
        self.album
            .images
            .iter()
            .find(|i| i.width.map(|w| w <= 300).unwrap_or(false))
            .or_else(|| self.album.images.first())
            .map(|i| i.url.as_str())
    }
}

// ── Currently playing ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CurrentlyPlayingResponse {
    pub is_playing: bool,
    pub progress_ms: Option<u64>,
    pub item: Option<TrackItem>,
}

// ── Player queue ─────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct PlayerQueueResponse {
    pub queue: Vec<QueueItem>,
}

#[derive(Debug, Deserialize)]
pub struct QueueItem {
    pub uri: String,
}

// ── Devices ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct DevicesResponse {
    pub devices: Vec<Device>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Device {
    pub id: Option<String>,
    pub is_active: bool,
}
