use std::collections::HashMap;

use anyhow::Result;
use chrono::Utc;
use rusqlite::Connection;
use tracing::info;

use crate::config::Config;
use crate::db::queries;
use crate::lastfm::LastfmClient;
use crate::progress::ProgressReporter;

/// For each external artist in the unified `artists` table, fetch their top
/// similar artists from Last.fm (cached in `similar_artists`), then store
/// similarity components (similarity_score, appearances, best_source).
/// Does NOT compute final_score — that is done by `score_artists`.
pub async fn expand_artists(
    conn: &Connection,
    client: &LastfmClient,
    config: &Config,
    progress: &dyn ProgressReporter,
) -> Result<()> {
    let now = Utc::now().timestamp();

    // Load external artists with scores.
    let scored = queries::get_external_artists_with_scores(conn)?;
    if scored.is_empty() {
        info!("No external artists found — run `gurdo sync-lastfm` first");
        return Ok(());
    }
    info!("Expanding from {} external artists", scored.len());

    let score_map: HashMap<String, f64> = scored
        .iter()
        .map(|(name, score)| (name.to_lowercase(), *score))
        .collect();

    // ── Fetch similar artists (cached) ────────────────────────────────────────
    progress.stage("Fetching similar artists");
    let mut fetched = 0usize;
    let mut cached  = 0usize;
    let total = scored.len() as u64;

    for (i, (source_name, _)) in scored.iter().enumerate() {
        progress.tick(i as u64 + 1, Some(total));
        if queries::is_artist_synced_for_similar(conn, source_name)? {
            cached += 1;
            continue;
        }

        let similar = client.artist_similar(source_name, config.engine.similar_artists_limit).await?;
        for s in &similar {
            queries::upsert_similar_artist(conn, source_name, &s.name, s.match_score(), now)?;
        }
        fetched += 1;
    }

    info!("Similar artists: {} fetched from API, {} served from cache", fetched, cached);

    // ── Aggregate similarity data for ALL artists (external + new) ─────────
    struct Entry {
        best_source: String,
        best_score:  f64,
        appearances: u32,
    }

    let mut aggregated: HashMap<String, Entry> = HashMap::new();

    for (source_name, source_score) in &scored {
        let similar = queries::get_similar_artists_for_seed(conn, source_name)?;
        for (sim_name, _match_score) in &similar {
            let key = sim_name.to_lowercase();

            let entry = aggregated.entry(key).or_insert_with(|| Entry {
                best_source: source_name.clone(),
                best_score:  *source_score,
                appearances: 0,
            });
            entry.appearances += 1;
            if *source_score > entry.best_score {
                entry.best_score  = *source_score;
                entry.best_source = source_name.clone();
            }
        }
    }

    let external_with_sim = aggregated.keys().filter(|k| score_map.contains_key(*k)).count();
    let similar_only = aggregated.len() - external_with_sim;
    info!("Similarity data: {} external artists boosted, {} new similar-only artists", external_with_sim, similar_only);

    // ── Persist similarity components ────────────────────────────────────────
    // Clear old similar-only artists; external ones keep their playcount data.
    queries::clear_similar_artists_scored(conn)?;

    for (artist_name, e) in &aggregated {
        let similarity_score = (e.best_score * config.engine.similarity_multiplier * 100.0).round() / 100.0;
        // Updates existing external artists or inserts new similar-only artists.
        queries::upsert_artist_similarity(
            conn,
            artist_name,
            similarity_score,
            e.appearances,
            &e.best_source,
            now,
        )?;
    }

    // ── Sync feedback counts into artists table ────────────────────────────
    queries::sync_artist_feedback_to_artists(conn)?;

    info!("Expand complete — {} similar artists stored (run `score` to recalculate)", similar_only);
    progress.finish(true, &format!("{} similar artists fetched, {} cached", fetched, cached));
    Ok(())
}

/// Fetch top tracks for external and similar artists.
/// If `sample` is Some(n), fetch only the top n from each pool; otherwise fetch all.
/// Skips artists already cached in artist_top_tracks.
pub async fn fetch_artist_tracks(
    conn: &Connection,
    client: &LastfmClient,
    sample: Option<usize>,
    config: &Config,
    progress: &dyn ProgressReporter,
) -> Result<()> {
    let now = Utc::now().timestamp();

    let external_artists: Vec<String> = {
        let all = queries::get_external_artist_names(conn)?;
        match sample {
            Some(n) => all.into_iter().take(n).collect(),
            None    => all,
        }
    };

    let similar_artists: Vec<String> = {
        let all = queries::get_similar_artists_scored(conn, usize::MAX)?;
        match sample {
            Some(n) => all.into_iter().take(n).map(|(name, _)| name).collect(),
            None    => all.into_iter().map(|(name, _)| name).collect(),
        }
    };

    let all: Vec<(String, &str)> = external_artists.iter().map(|n| (n.clone(), "external"))
        .chain(similar_artists.iter().map(|n| (n.clone(), "similar")))
        .collect();

    info!("Fetching tracks for {} external + {} similar artists", external_artists.len(), similar_artists.len());

    progress.stage("Fetching top tracks");
    let mut fetched = 0usize;
    let mut cached  = 0usize;
    let total = all.len() as u64;

    for (i, (artist, source)) in all.iter().enumerate() {
        progress.tick(i as u64 + 1, Some(total));
        if queries::is_artist_tracks_synced(conn, artist)? {
            info!("  [{}] {} — cached", source, artist);
            cached += 1;
            continue;
        }

        info!("  [{}] {} — fetching", source, artist);
        let tracks = client.artist_top_tracks(artist, config.engine.artist_top_tracks_limit).await?;
        for t in &tracks {
            queries::upsert_artist_top_track(
                conn,
                &t.artist.name,
                &t.name,
                &t.mbid,
                t.playcount.parse().unwrap_or(0),
                t.listeners.parse().unwrap_or(0),
                t.rank(),
                now,
            )?;
        }
        fetched += 1;
    }

    info!("Tracks fetched: {} from API, {} served from cache", fetched, cached);
    progress.finish(true, &format!("{} artists fetched, {} cached", fetched, cached));
    Ok(())
}
