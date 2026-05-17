use anyhow::{bail, Result};
use reqwest::{Client, StatusCode};
use serde_json::json;
use tracing::debug;

use super::models::{
    CurrentlyPlayingResponse,
    DevicesResponse, Device,
    PlayerQueueResponse,
    SearchResponse, TrackItem,
};

const API_BASE: &str = "https://api.spotify.com/v1";

pub struct SpotifyClient {
    client: Client,
    access_token: String,
}

impl SpotifyClient {
    pub fn new(access_token: String) -> Self {
        Self {
            client: Client::new(),
            access_token,
        }
    }

    // ── Search ────────────────────────────────────────────────────────────────

    /// Search for a track by artist + name. Returns the best match or None if not found.
    pub async fn search_track(&self, artist: &str, track: &str) -> Result<Option<TrackItem>> {
        let q = format!("track:{} artist:{}", track, artist);
        debug!("Spotify search: {}", q);

        let resp = self.client
            .get(format!("{}/search", API_BASE))
            .bearer_auth(&self.access_token)
            .query(&[
                ("q", q.as_str()),
                ("type", "track"),
                ("limit", "1"),
                ("market", "from_token"),
            ])
            .send()
            .await?;

        if resp.status() == StatusCode::UNAUTHORIZED {
            bail!("Spotify token expired — run `gurdo login` to re-authenticate");
        }
        if !resp.status().is_success() {
            bail!("Spotify search error: HTTP {}", resp.status());
        }

        let body: SearchResponse = resp.json().await?;
        Ok(body.tracks.items.into_iter().next())
    }

    // ── Liked Songs ──────────────────────────────────────────────────────────

    // ── Currently playing ─────────────────────────────────────────────────────

    pub async fn get_currently_playing(&self) -> Result<Option<CurrentlyPlayingResponse>> {
        let resp = self.client
            .get(format!("{}/me/player/currently-playing", API_BASE))
            .bearer_auth(&self.access_token)
            .send()
            .await?;

        if resp.status() == StatusCode::NO_CONTENT {
            return Ok(None);
        }
        if resp.status() == StatusCode::UNAUTHORIZED {
            bail!("Spotify token expired — run `gurdo login` to re-authenticate");
        }
        if !resp.status().is_success() {
            bail!("Spotify currently-playing error: HTTP {}", resp.status());
        }

        let body: CurrentlyPlayingResponse = resp.json().await?;
        Ok(Some(body))
    }

    // ── Queue ─────────────────────────────────────────────────────────────────

    /// Returns the URIs of tracks currently in Spotify's playback queue
    /// (does not include the currently-playing track).
    pub async fn get_queue(&self) -> Result<Vec<String>> {
        let resp = self.client
            .get(format!("{}/me/player/queue", API_BASE))
            .bearer_auth(&self.access_token)
            .send()
            .await?;

        if resp.status() == StatusCode::NO_CONTENT {
            return Ok(vec![]);
        }
        if resp.status() == StatusCode::UNAUTHORIZED {
            bail!("Spotify token expired — run `gurdo login` to re-authenticate");
        }
        if !resp.status().is_success() {
            bail!("Spotify queue error: HTTP {}", resp.status());
        }

        let body: PlayerQueueResponse = resp.json().await?;
        Ok(body.queue.into_iter().map(|i| i.uri).collect())
    }

    // ── Devices ───────────────────────────────────────────────────────────────

    pub async fn get_devices(&self) -> Result<Vec<Device>> {
        let resp = self.client
            .get(format!("{}/me/player/devices", API_BASE))
            .bearer_auth(&self.access_token)
            .send()
            .await?;

        if resp.status() == StatusCode::UNAUTHORIZED {
            bail!("Spotify token expired — run `gurdo login` to re-authenticate");
        }
        if !resp.status().is_success() {
            bail!("Spotify devices error: HTTP {}", resp.status());
        }

        let body: DevicesResponse = resp.json().await?;
        Ok(body.devices)
    }

    /// Returns the active device, or the first available one if none is active.
    pub async fn active_device(&self) -> Result<Device> {
        let devices = self.get_devices().await?;
        if devices.is_empty() {
            bail!("No Spotify devices found. Open Spotify on your computer or phone first.");
        }
        let device = devices.iter().find(|d| d.is_active)
            .or_else(|| devices.first())
            .cloned()
            .unwrap();
        Ok(device)
    }

    // ── Playback ──────────────────────────────────────────────────────────────

    /// Start playing a list of track URIs on a specific device.
    pub async fn play(&self, device_id: &str, uris: &[String]) -> Result<()> {
        let resp = self.client
            .put(format!("{}/me/player/play", API_BASE))
            .bearer_auth(&self.access_token)
            .query(&[("device_id", device_id)])
            .json(&json!({ "uris": uris }))
            .send()
            .await?;

        match resp.status() {
            StatusCode::NO_CONTENT | StatusCode::OK => Ok(()),
            StatusCode::UNAUTHORIZED => bail!("Spotify token expired — run `gurdo login`"),
            StatusCode::FORBIDDEN => bail!("Spotify Premium required for playback control"),
            StatusCode::NOT_FOUND => bail!("Device not found — it may have gone offline"),
            s => bail!("Spotify play error: HTTP {}", s),
        }
    }

    pub async fn seek(&self, device_id: &str, position_ms: u64) -> Result<()> {
        let resp = self.client
            .put(format!("{}/me/player/seek", API_BASE))
            .bearer_auth(&self.access_token)
            .query(&[("device_id", device_id), ("position_ms", &position_ms.to_string())])
            .header("Content-Length", "0")
            .send()
            .await?;

        if resp.status() != StatusCode::NO_CONTENT && resp.status() != StatusCode::OK {
            bail!("Spotify seek error: HTTP {}", resp.status());
        }
        Ok(())
    }

    pub async fn pause(&self, device_id: &str) -> Result<()> {
        let resp = self.client
            .put(format!("{}/me/player/pause", API_BASE))
            .bearer_auth(&self.access_token)
            .query(&[("device_id", device_id)])
            .header("Content-Length", "0")
            .send()
            .await?;

        if resp.status() != StatusCode::NO_CONTENT && resp.status() != StatusCode::OK {
            bail!("Spotify pause error: HTTP {}", resp.status());
        }
        Ok(())
    }

    pub async fn next(&self, device_id: &str) -> Result<()> {
        let resp = self.client
            .post(format!("{}/me/player/next", API_BASE))
            .bearer_auth(&self.access_token)
            .query(&[("device_id", device_id)])
            .header("Content-Length", "0")
            .send()
            .await?;

        if resp.status() != StatusCode::NO_CONTENT && resp.status() != StatusCode::OK {
            bail!("Spotify next error: HTTP {}", resp.status());
        }
        Ok(())
    }

    pub async fn previous(&self, device_id: &str) -> Result<()> {
        let resp = self.client
            .post(format!("{}/me/player/previous", API_BASE))
            .bearer_auth(&self.access_token)
            .query(&[("device_id", device_id)])
            .header("Content-Length", "0")
            .send()
            .await?;

        if resp.status() != StatusCode::NO_CONTENT && resp.status() != StatusCode::OK {
            bail!("Spotify previous error: HTTP {}", resp.status());
        }
        Ok(())
    }

    pub async fn add_to_queue(&self, device_id: &str, uri: &str) -> Result<()> {
        let resp = self.client
            .post(format!("{}/me/player/queue", API_BASE))
            .bearer_auth(&self.access_token)
            .query(&[("uri", uri), ("device_id", device_id)])
            .header("Content-Length", "0")
            .send()
            .await?;

        match resp.status() {
            StatusCode::NO_CONTENT | StatusCode::OK => Ok(()),
            StatusCode::UNAUTHORIZED => bail!("Spotify token expired — run `gurdo login`"),
            StatusCode::FORBIDDEN => bail!("Spotify Premium required for playback control"),
            s => bail!("Spotify add to queue error: HTTP {}", s),
        }
    }

    pub async fn resume(&self, device_id: &str) -> Result<()> {
        let resp = self.client
            .put(format!("{}/me/player/play", API_BASE))
            .bearer_auth(&self.access_token)
            .query(&[("device_id", device_id)])
            .header("Content-Length", "0")
            .send()
            .await?;

        match resp.status() {
            StatusCode::NO_CONTENT | StatusCode::OK => Ok(()),
            StatusCode::UNAUTHORIZED => bail!("Spotify token expired — run `gurdo login`"),
            StatusCode::FORBIDDEN => bail!("Spotify Premium required for playback control"),
            StatusCode::NOT_FOUND => bail!("Device not found — it may have gone offline"),
            s => bail!("Spotify resume error: HTTP {}", s),
        }
    }
}
