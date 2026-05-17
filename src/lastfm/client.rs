use anyhow::{bail, Result};
use reqwest::Client;
use serde::de::DeserializeOwned;
use tokio::time::{sleep, Duration};
use tracing::debug;

use super::models::*;

const BASE_URL: &str = "https://ws.audioscrobbler.com/2.0/";
// Last.fm allows ~5 req/s; we stay conservative at 4 req/s
const REQUEST_DELAY_MS: u64 = 250;

pub struct LastfmClient {
    client: Client,
    api_key: String,
}

impl LastfmClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    async fn get<T: DeserializeOwned>(&self, params: &[(&str, &str)]) -> Result<T> {
        sleep(Duration::from_millis(REQUEST_DELAY_MS)).await;

        let mut query: Vec<(&str, &str)> = vec![
            ("api_key", &self.api_key),
            ("format", "json"),
        ];
        query.extend_from_slice(params);

        debug!("Last.fm request: {:?}", params.iter().find(|(k, _)| *k == "method"));

        let response = self.client.get(BASE_URL).query(&query).send().await?;

        if !response.status().is_success() {
            bail!("Last.fm API error: HTTP {}", response.status());
        }

        let text = response.text().await?;

        // Last.fm returns error objects with {"error": N, "message": "..."}
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(code) = val.get("error") {
                let msg = val.get("message").and_then(|m| m.as_str()).unwrap_or("");
                bail!("Last.fm API error {}: {}", code, msg);
            }
        }

        let result = serde_json::from_str::<T>(&text)
            .map_err(|e| anyhow::anyhow!("JSON parse error: {}\nBody: {}", e, &text[..text.len().min(300)]))?;

        Ok(result)
    }

    // ── user endpoints ───────────────────────────────────────────────────────

    pub async fn user_loved_tracks(&self, username: &str, limit: u32) -> Result<Vec<LovedTrack>> {
        let per_page = 200u32;
        let mut all: Vec<LovedTrack> = Vec::new();
        let mut page = 1u32;

        loop {
            let page_s = page.to_string();
            let per_page_s = per_page.to_string();
            let resp: LovedTracksResponse = self.get(&[
                ("method", "user.getLovedTracks"),
                ("user", username),
                ("limit", &per_page_s),
                ("page", &page_s),
            ]).await?;

            let total_pages = resp.lovedtracks.attr.total_pages_u32();
            all.extend(resp.lovedtracks.tracks);

            if page >= total_pages || all.len() >= limit as usize {
                break;
            }
            page += 1;
        }

        all.truncate(limit as usize);
        Ok(all)
    }

    pub async fn user_top_tags(&self, username: &str, limit: u32) -> Result<Vec<TopTag>> {
        let limit_s = limit.to_string();
        let resp: TopTagsResponse = self.get(&[
            ("method", "user.getTopTags"),
            ("user", username),
            ("limit", &limit_s),
        ]).await?;
        Ok(resp.toptags.tags)
    }

    // ── artist endpoints ─────────────────────────────────────────────────────

    pub async fn artist_similar(&self, artist: &str, limit: u32) -> Result<Vec<SimilarArtist>> {
        let limit_s = limit.to_string();
        let resp: SimilarArtistsResponse = self.get(&[
            ("method", "artist.getSimilar"),
            ("artist", artist),
            ("limit", &limit_s),
            ("autocorrect", "1"),
        ]).await?;
        Ok(resp.similarartists.artists)
    }

    pub async fn artist_top_tracks(&self, artist: &str, limit: u32) -> Result<Vec<ArtistTopTrack>> {
        let limit_s = limit.to_string();
        let resp: ArtistTopTracksResponse = self.get(&[
            ("method", "artist.getTopTracks"),
            ("artist", artist),
            ("limit", &limit_s),
            ("autocorrect", "1"),
        ]).await?;
        Ok(resp.toptracks.tracks)
    }

    // ── weekly chart endpoints ───────────────────────────────────────────────

    /// Returns the list of all available weekly chart boundaries for a user.
    pub async fn weekly_chart_list(&self, username: &str) -> Result<Vec<WeeklyChartEntry>> {
        let resp: WeeklyChartListResponse = self.get(&[
            ("method", "user.getWeeklyChartList"),
            ("user", username),
        ]).await?;
        Ok(resp.weeklychartlist.charts)
    }

    /// Returns the artist chart for an arbitrary date range.
    /// Last.fm caps the response at 1000 entries regardless of the range size.
    pub async fn weekly_artist_chart(
        &self,
        username: &str,
        from_ts: i64,
        to_ts: i64,
    ) -> Result<Vec<WeeklyArtistEntry>> {
        let from_s = from_ts.to_string();
        let to_s   = to_ts.to_string();
        let resp: WeeklyArtistChartResponse = self.get(&[
            ("method", "user.getWeeklyArtistChart"),
            ("user", username),
            ("from", &from_s),
            ("to", &to_s),
        ]).await?;
        Ok(resp.weeklyartistchart.artists)
    }

}
