use std::collections::HashMap;

use anyhow::Result;
use chrono::{Datelike, TimeZone, Utc};
use rusqlite::Connection;
use tracing::info;

use crate::config::Config;
use crate::db::queries;
use crate::lastfm::LastfmClient;
use crate::progress::ProgressReporter;

/// Hard cap Last.fm imposes on getWeeklyArtistChart responses.
const LASTFM_CHART_CAP: usize = 1000;

/// Fetch the artist chart for [from_ts, to_ts), auto-splitting the range in half
/// whenever the 1 000-entry cap is hit.  Returns a map of artist_name → (mbid, playcount).
/// `depth` guards against infinite recursion on extremely dense ranges (max 4 splits = 16 slices).
async fn fetch_range(
    client: &LastfmClient,
    username: &str,
    from_ts: i64,
    to_ts: i64,
    depth: u32,
) -> Result<HashMap<String, (String, u64)>> {
    let entries = client.weekly_artist_chart(username, from_ts, to_ts).await?;

    if entries.len() < LASTFM_CHART_CAP || depth >= 4 {
        return Ok(entries
            .into_iter()
            .map(|e| {
                let playcount = e.playcount_u64();
                (e.name, (e.mbid, playcount))
            })
            .collect());
    }

    // Hit cap — split the time range in half and recurse.
    let mid = (from_ts + to_ts) / 2;
    let h1 = Box::pin(fetch_range(client, username, from_ts, mid, depth + 1)).await?;
    let h2 = Box::pin(fetch_range(client, username, mid,     to_ts, depth + 1)).await?;

    // Merge: sum playcounts for artists that appear in both halves.
    let mut merged = h1;
    for (name, (mbid, count)) in h2 {
        let entry = merged.entry(name).or_insert((mbid, 0));
        entry.1 += count;
    }
    Ok(merged)
}

/// Sync artist history using year-by-year weekly charts plus recent period data.
///
/// Past years (where year < current year) are only fetched once and cached; the
/// current year and recent periods are always refreshed.
pub async fn sync_artist_history(
    conn: &Connection,
    client: &LastfmClient,
    config: &Config,
    progress: &dyn ProgressReporter,
) -> Result<()> {
    let username = &config.lastfm.username;
    let now      = Utc::now().timestamp();
    let current_year = Utc::now().year();

    // ── Year-by-year charts ───────────────────────────────────────────────────
    // Ask Last.fm which week is the earliest in the user's history.
    let chart_list = client.weekly_chart_list(username).await?;
    let first_year = chart_list
        .first()
        .and_then(|c| {
            let ts = c.from_ts();
            chrono::DateTime::from_timestamp(ts, 0).map(|dt| dt.year())
        })
        .unwrap_or(2005);

    info!("Syncing year-by-year artist charts {} → {}", first_year, current_year);

    let total_years = (current_year - first_year + 1) as u64;
    for (year_idx, year) in (first_year..=current_year).enumerate() {
        progress.tick(year_idx as u64 + 1, Some(total_years));
        let period_label = year.to_string();
        let is_current   = year == current_year;

        // Skip past years that are already cached.
        if !is_current && queries::is_period_synced(conn, &period_label)? {
            info!("  {} — already cached, skipping", year);
            continue;
        }

        let from_ts = Utc
            .with_ymd_and_hms(year, 1, 1, 0, 0, 0)
            .single()
            .map(|dt| dt.timestamp())
            .unwrap_or(0);
        let to_ts = if is_current {
            now
        } else {
            Utc.with_ymd_and_hms(year + 1, 1, 1, 0, 0, 0)
                .single()
                .map(|dt| dt.timestamp())
                .unwrap_or(now)
        };

        info!("  {} — fetching...", year);
        let artists = fetch_range(client, username, from_ts, to_ts, 0).await?;

        if artists.is_empty() {
            info!("  {} — no scrobbles", year);
            continue;
        }

        for (name, (mbid, playcount)) in &artists {
            queries::upsert_artist_chart_entry(conn, name, mbid, *playcount, &period_label, now)?;
        }
        info!("  {} — {} artists stored", year, artists.len());
    }

    Ok(())
}
