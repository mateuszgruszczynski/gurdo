pub mod artists;
pub mod expand;
pub use artists::sync_artist_history;
pub use expand::expand_artists;
pub use expand::fetch_artist_tracks;

use std::collections::HashMap;

use anyhow::Result;
use chrono::Utc;
use rusqlite::Connection;
use tracing::info;

use crate::config::Config;
use crate::db::queries;
use crate::lastfm::LastfmClient;
use crate::progress::ProgressReporter;

/// Full Last.fm sync: loved tracks + top tags + year-by-year artist charts
/// + compute playcount_score/year_bonus and persist external artists.
pub async fn sync_lastfm(
    conn: &Connection,
    client: &LastfmClient,
    config: &Config,
    progress: &dyn ProgressReporter,
) -> Result<()> {
    let username = &config.lastfm.username;
    let now = Utc::now().timestamp();

    // ── Loved tracks ──────────────────────────────────────────────────────────
    progress.stage("Syncing loved tracks");
    info!("Syncing loved tracks (up to {})", config.sync.loved_tracks_limit);
    let loved = client.user_loved_tracks(username, config.sync.loved_tracks_limit).await?;
    let loved_total = loved.len() as u64;
    for (i, t) in loved.iter().enumerate() {
        queries::upsert_loved_track(
            conn,
            &t.name,
            &t.artist.name,
            &t.mbid,
            t.date.as_ref().map(|d| d.timestamp()),
            now,
        )?;
        // Record each loved track as a like in feedback tables.
        queries::record_feedback(conn, &t.artist.name, &t.name, true)?;
        progress.tick(i as u64 + 1, Some(loved_total));
    }
    info!("  → {} loved tracks stored (and recorded as likes)", loved.len());

    // ── Top tags (genre fingerprint, up to 50 — Last.fm API max) ─────────────
    progress.stage("Syncing top tags");
    info!("Syncing top tags");
    let tags = client.user_top_tags(username, 50).await?;
    let tags_total = tags.len() as u64;
    for (i, tag) in tags.iter().enumerate() {
        queries::upsert_top_tag(conn, &tag.name, tag.count, now)?;
        progress.tick(i as u64 + 1, Some(tags_total));
    }
    info!("  → {} tags stored", tags.len());

    // ── Year-by-year artist charts ────────────────────────────────────────────
    progress.stage("Syncing artist history");
    sync_artist_history(conn, client, config, progress).await?;

    // ── Compute playcount_score / year_bonus and persist external artists ─────
    progress.stage("Scoring external artists");
    score_external_artists(conn, config)?;

    progress.finish(true, &format!("{} loved tracks, {} tags synced", loved.len(), tags.len()));
    Ok(())
}

/// Aggregate playcount data from `artist_chart_entries`, compute
/// `playcount_score` (0–100) and `year_bonus`, and upsert into the
/// `artists` table as source='external'. Also syncs feedback counts.
fn score_external_artists(conn: &Connection, config: &Config) -> Result<()> {
    let cfg = &config.artist_scoring;

    // ── 1. Aggregate playcount and year count per artist ─────────────────────
    let year_entries = queries::get_all_year_chart_entries(conn)?;

    let mut aggregated: HashMap<String, (u64, std::collections::HashSet<String>)> = HashMap::new();
    for (name, _mbid, playcount, period_label) in &year_entries {
        let key = name.to_lowercase();
        let entry = aggregated.entry(key).or_default();
        entry.0 += playcount;
        entry.1.insert(period_label.clone());
    }

    info!("Total unique artists in year charts: {}", aggregated.len());

    // ── 2. Load artists with liked tracks (includes loved tracks from Last.fm) ─
    let liked_artists = queries::get_liked_artist_names(conn)?;

    // ── 3. Select the scoring pool ───────────────────────────────────────────
    let threshold = cfg.min_playcount_threshold;

    let mut pool: std::collections::HashSet<String> = aggregated
        .iter()
        .filter(|(name, (total_pc, _))| *total_pc >= threshold || liked_artists.contains(*name))
        .map(|(name, _)| name.clone())
        .collect();

    for name in &liked_artists {
        pool.insert(name.clone());
    }

    let liked_only = pool.iter()
        .filter(|name| {
            aggregated.get(*name).map(|(pc, _)| *pc < threshold).unwrap_or(true)
                && liked_artists.contains(*name)
        })
        .count();

    info!(
        "Scoring pool: {} artists (≥{} plays: {}, liked-only: {})",
        pool.len(), threshold, pool.len() - liked_only, liked_only,
    );

    // ── 4. Compute playcount scores and normalize to 0-100 ──────────────────
    let max_raw_score = pool.iter()
        .map(|name| {
            let total_playcount = aggregated.get(name).map(|(pc, _)| *pc).unwrap_or(0);
            (total_playcount as f64).max(1.0).powf(cfg.score_exponent)
        })
        .fold(1.0_f64, f64::max);

    let mut scored: Vec<(String, u64, u32, f64, f64)> = pool
        .iter()
        .map(|name| {
            let (total_playcount, years_active) = aggregated
                .get(name)
                .map(|(pc, years_set)| (*pc, years_set.len() as u32))
                .unwrap_or((0, 0));

            let raw_score = (total_playcount as f64).max(1.0).powf(cfg.score_exponent);
            let playcount_score = ((raw_score / max_raw_score) * 10000.0).round() / 100.0;
            let year_bonus = ((1.0 + (years_active.saturating_sub(1) as f64) * (cfg.year_bonus_pct / 100.0)) * 100.0).round() / 100.0;

            (name.clone(), total_playcount, years_active, playcount_score, year_bonus)
        })
        .collect();

    scored.sort_by(|a, b| {
        let a_base = a.3 * a.4;
        let b_base = b.3 * b.4;
        b_base.partial_cmp(&a_base).unwrap()
    });

    // ── 5. Persist to unified artists table ─────────────────────────────────
    let now = Utc::now().timestamp();
    // Reset playcount fields for all external artists (preserves similarity data).
    conn.execute(
        "UPDATE artists SET total_playcount=0, years_active=0, playcount_score=0.0, year_bonus=1.0
         WHERE source='external'",
        [],
    )?;

    for (name, total_playcount, years_active, playcount_score, year_bonus) in &scored {
        queries::upsert_artist_external(
            conn,
            name,
            *total_playcount,
            *years_active,
            *playcount_score,
            *year_bonus,
            now,
        )?;
    }

    // ── 6. Sync feedback counts into artists table ──────────────────────────
    queries::sync_artist_feedback_to_artists(conn)?;

    info!("Synced {} external artists to DB", scored.len());
    Ok(())
}
