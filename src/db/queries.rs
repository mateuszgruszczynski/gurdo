use anyhow::Result;
use rusqlite::{Connection, params};

#[cfg(test)]
fn open_mem() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    crate::db::schema::init_db(&conn).unwrap();
    conn
}

// ── Upsert helpers ────────────────────────────────────────────────────────────

pub fn upsert_loved_track(
    conn: &Connection,
    name: &str,
    artist_name: &str,
    mbid: &str,
    loved_at: Option<i64>,
    fetched_at: i64,
) -> Result<()> {
    conn.execute(
        "INSERT INTO loved_tracks (name, artist_name, mbid, loved_at, fetched_at)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(name, artist_name) DO UPDATE SET
           mbid=excluded.mbid, loved_at=excluded.loved_at, fetched_at=excluded.fetched_at",
        params![name, artist_name, mbid, loved_at, fetched_at],
    )?;
    Ok(())
}

pub fn upsert_top_tag(conn: &Connection, name: &str, count: u32, fetched_at: i64) -> Result<()> {
    conn.execute(
        "INSERT INTO top_tags (name, count, fetched_at)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(name) DO UPDATE SET count=excluded.count, fetched_at=excluded.fetched_at",
        params![name, count, fetched_at],
    )?;
    Ok(())
}

pub fn upsert_similar_artist(
    conn: &Connection,
    seed: &str,
    similar: &str,
    score: f64,
    fetched_at: i64,
) -> Result<()> {
    let seed = seed.to_lowercase();
    let similar = similar.to_lowercase();
    conn.execute(
        "INSERT INTO similar_artists (seed_artist, similar_artist, match_score, fetched_at)
         VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(seed_artist, similar_artist) DO UPDATE SET
           match_score=excluded.match_score, fetched_at=excluded.fetched_at",
        params![seed, similar, score, fetched_at],
    )?;
    Ok(())
}

pub fn upsert_artist_top_track(
    conn: &Connection,
    artist: &str,
    track: &str,
    mbid: &str,
    playcount: u64,
    listeners: u64,
    rank: u32,
    fetched_at: i64,
) -> Result<()> {
    let artist = artist.to_lowercase();
    conn.execute(
        "INSERT INTO artist_top_tracks (artist_name, track_name, mbid, playcount, listeners, rank, fetched_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
         ON CONFLICT(artist_name, track_name) DO UPDATE SET
           mbid=excluded.mbid, playcount=excluded.playcount,
           listeners=excluded.listeners, rank=excluded.rank, fetched_at=excluded.fetched_at",
        params![artist, track, mbid, playcount as i64, listeners as i64, rank, fetched_at],
    )?;
    Ok(())
}

// ── Read helpers ──────────────────────────────────────────────────────────────

pub fn is_artist_synced_for_similar(conn: &Connection, artist: &str) -> Result<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM similar_artists WHERE seed_artist=?1",
        params![artist.to_lowercase()],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

pub fn is_artist_tracks_synced(conn: &Connection, artist: &str) -> Result<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM artist_top_tracks WHERE artist_name=?1",
        params![artist.to_lowercase()],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

pub fn get_similar_artists_for_seed(conn: &Connection, seed: &str) -> Result<Vec<(String, f64)>> {
    let mut stmt = conn.prepare(
        "SELECT similar_artist, match_score FROM similar_artists WHERE seed_artist=?1 ORDER BY match_score DESC"
    )?;
    let rows = stmt.query_map(params![seed], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
    })?;
    Ok(rows.filter_map(|r| r.ok()).collect())
}

/// Load all tracks for all artists in one query. Returns a map of artist_name → [(track_name, rank)].
pub fn get_all_artist_top_tracks(conn: &Connection) -> Result<std::collections::HashMap<String, Vec<(String, u32)>>> {
    let mut stmt = conn.prepare(
        "SELECT artist_name, track_name, rank FROM artist_top_tracks ORDER BY artist_name, rank ASC"
    )?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, u32>(2)?))
    })?;
    let mut map: std::collections::HashMap<String, Vec<(String, u32)>> = std::collections::HashMap::new();
    for row in rows.flatten() {
        map.entry(row.0).or_default().push((row.1, row.2));
    }
    Ok(map)
}

// ── Spotify URI cache ─────────────────────────────────────────────────────────

pub fn upsert_spotify_uri(
    conn: &Connection,
    artist: &str,
    track: &str,
    spotify_uri: Option<&str>,
    album_art_url: Option<&str>,
    fetched_at: i64,
) -> Result<()> {
    conn.execute(
        "INSERT INTO spotify_uri_cache (artist_name, track_name, spotify_uri, album_art_url, fetched_at)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(artist_name, track_name) DO UPDATE SET
           spotify_uri=excluded.spotify_uri, album_art_url=excluded.album_art_url, fetched_at=excluded.fetched_at",
        rusqlite::params![artist, track, spotify_uri, album_art_url, fetched_at],
    )?;
    Ok(())
}


// ── Unified artists table ─────────────────────────────────────────────────────

/// Returns (artist_name, final_score) for all artists that have tracks in
/// artist_top_tracks. All names are stored lowercase.
pub fn get_scoreable_artists_with_tracks(conn: &Connection) -> Result<Vec<(String, f64)>> {
    let mut stmt = conn.prepare(
        "SELECT a.artist_name, a.final_score
         FROM artists a
         WHERE a.final_score > 0
           AND EXISTS (SELECT 1 FROM artist_top_tracks t WHERE t.artist_name = a.artist_name)",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
    })?;
    Ok(rows.filter_map(|r| r.ok()).collect())
}

/// Returns all artists with source='similar' sorted by score, for fetch-tracks.
pub fn get_similar_artists_scored(conn: &Connection, limit: usize) -> Result<Vec<(String, f64)>> {
    let limit_sql = if limit == usize::MAX { -1i64 } else { limit as i64 };
    let mut stmt = conn.prepare(
        "SELECT artist_name, final_score FROM artists WHERE source='similar' ORDER BY final_score DESC LIMIT ?1",
    )?;
    let rows = stmt.query_map(params![limit_sql], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
    })?;
    Ok(rows.filter_map(|r| r.ok()).collect())
}

/// Clear only similar artists (before a re-expand).
pub fn clear_similar_artists_scored(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM artists WHERE source='similar'", [])?;
    Ok(())
}

/// Upsert an external (known) artist from the score phase.
/// Only writes playcount components — does not touch similarity or feedback fields.
pub fn upsert_artist_external(
    conn: &Connection,
    artist_name: &str,
    total_playcount: u64,
    years_active: u32,
    playcount_score: f64,
    year_bonus: f64,
    generated_at: i64,
) -> Result<()> {
    conn.execute(
        "INSERT INTO artists
         (artist_name, source, total_playcount, years_active, playcount_score, year_bonus,
          generated_at)
         VALUES (?1, 'external', ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(artist_name) DO UPDATE SET
           source='external',
           total_playcount=excluded.total_playcount, years_active=excluded.years_active,
           playcount_score=excluded.playcount_score, year_bonus=excluded.year_bonus,
           generated_at=excluded.generated_at",
        params![artist_name, total_playcount as i64, years_active, playcount_score, year_bonus,
                generated_at],
    )?;
    Ok(())
}

/// Update similarity fields for an artist. Works for both external and similar-only artists.
/// Does not change source or playcount fields.
pub fn upsert_artist_similarity(
    conn: &Connection,
    artist_name: &str,
    similarity_score: f64,
    appearances: u32,
    best_source: &str,
    generated_at: i64,
) -> Result<()> {
    conn.execute(
        "INSERT INTO artists
         (artist_name, source, similarity_score, similarity_appearances,
          best_similar_source, generated_at)
         VALUES (?1, 'similar', ?2, ?3, ?4, ?5)
         ON CONFLICT(artist_name) DO UPDATE SET
           similarity_score=excluded.similarity_score,
           similarity_appearances=excluded.similarity_appearances,
           best_similar_source=excluded.best_similar_source,
           generated_at=excluded.generated_at",
        params![artist_name, similarity_score, appearances, best_source, generated_at],
    )?;
    Ok(())
}

/// Sync like/dislike counts from artist_feedback into the artists table.
pub fn sync_artist_feedback_to_artists(conn: &Connection) -> Result<usize> {
    let updated = conn.execute(
        "UPDATE artists SET
           likes = COALESCE((SELECT likes FROM artist_feedback WHERE artist_feedback.artist_name = artists.artist_name), 0),
           dislikes = COALESCE((SELECT dislikes FROM artist_feedback WHERE artist_feedback.artist_name = artists.artist_name), 0)",
        [],
    )?;
    Ok(updated)
}

/// Recalculate final_score for ALL artists using the unified formula:
///   playcount_base  = playcount_score × year_bonus
///   similarity_base = similarity_score × (1 + multi_source_bonus × (appearances - 1))
///   base_score      = playcount_base + similarity_base
///   feedback_bonus  = likes × like_bonus_flat
///   feedback_penalty = max(0, 1.0 - dislikes × dislike_pct)
///   final_score     = (base_score + feedback_bonus) × feedback_penalty
pub fn recalculate_all_scores(
    conn: &Connection,
    like_bonus_flat: f64,
    dislike_pct: f64,
    multi_source_bonus_pct: f64,
) -> Result<usize> {
    let updated = conn.execute(
        "UPDATE artists SET final_score = ROUND(
           (playcount_score * year_bonus
            + similarity_score * (1.0 + ?1 * MAX(0, similarity_appearances - 1))
            + likes * ?2)
           * MAX(0.0, 1.0 - dislikes * ?3),
         2)",
        params![multi_source_bonus_pct, like_bonus_flat, dislike_pct],
    )?;
    Ok(updated)
}

/// Returns all artists sorted by score for reporting.
pub fn get_all_artists_ranked(conn: &Connection) -> Result<Vec<(String, String, f64, u64, u32, f64, f64, i32, i32)>> {
    let mut stmt = conn.prepare(
        "SELECT artist_name, source, final_score, total_playcount, years_active,
                playcount_score, similarity_score, likes, dislikes
         FROM artists ORDER BY final_score DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, f64>(2)?,
            row.get::<_, i64>(3)? as u64,
            row.get::<_, u32>(4)?,
            row.get::<_, f64>(5)?,
            row.get::<_, f64>(6)?,
            row.get::<_, i32>(7)?,
            row.get::<_, i32>(8)?,
        ))
    })?;
    Ok(rows.filter_map(|r| r.ok()).collect())
}

/// Returns all external artist names (for use as seed pool in expand).
pub fn get_external_artist_names(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT artist_name FROM artists WHERE source='external' ORDER BY final_score DESC",
    )?;
    let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
    Ok(rows.filter_map(|r| r.ok()).collect())
}

/// Returns (artist_name, final_score) for external artists, used by expand to build score map.
/// Returns external artists with their playcount base score (playcount_score × year_bonus).
/// Uses the pure playcount base — NOT final_score — so similarity derivation is stable
/// and doesn't inflate on repeated expand runs.
pub fn get_external_artists_with_scores(conn: &Connection) -> Result<Vec<(String, f64)>> {
    let mut stmt = conn.prepare(
        "SELECT artist_name, playcount_score * year_bonus FROM artists
         WHERE source='external' ORDER BY playcount_score * year_bonus DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
    })?;
    Ok(rows.filter_map(|r| r.ok()).collect())
}

// ── Track & artist feedback ───────────────────────────────────────────────────

/// Record a like or dislike for a track and update the artist aggregate.
/// `liked`: true = liked, false = disliked.
pub fn record_feedback(conn: &Connection, artist: &str, track: &str, liked: bool) -> Result<()> {
    let artist = artist.to_lowercase();
    let track  = track.to_lowercase();
    let now    = chrono::Utc::now().timestamp();

    // Was there a previous opposing vote we need to reverse?
    let previous: Option<bool> = conn.query_row(
        "SELECT liked FROM track_feedback WHERE artist_name=?1 AND track_name=?2",
        params![artist, track],
        |r| r.get::<_, i32>(0),
    ).ok().map(|v| v != 0);

    conn.execute(
        "INSERT INTO track_feedback (artist_name, track_name, liked, created_at)
         VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(artist_name, track_name) DO UPDATE SET liked=excluded.liked, created_at=excluded.created_at",
        params![artist, track, liked as i32, now],
    )?;

    // Ensure artist_feedback row exists.
    conn.execute(
        "INSERT INTO artist_feedback (artist_name, likes, dislikes) VALUES (?1, 0, 0)
         ON CONFLICT(artist_name) DO NOTHING",
        params![artist],
    )?;

    // Adjust counts: reverse previous vote if needed, then apply new vote.
    match (previous, liked) {
        (Some(true), false) => {
            // was liked, now disliked: -1 like, +1 dislike
            conn.execute(
                "UPDATE artist_feedback SET likes=MAX(0,likes-1), dislikes=dislikes+1 WHERE artist_name=?1",
                params![artist],
            )?;
        }
        (Some(false), true) => {
            // was disliked, now liked: +1 like, -1 dislike
            conn.execute(
                "UPDATE artist_feedback SET likes=likes+1, dislikes=MAX(0,dislikes-1) WHERE artist_name=?1",
                params![artist],
            )?;
        }
        (None, true) => {
            conn.execute("UPDATE artist_feedback SET likes=likes+1 WHERE artist_name=?1", params![artist])?;
        }
        (None, false) => {
            conn.execute("UPDATE artist_feedback SET dislikes=dislikes+1 WHERE artist_name=?1", params![artist])?;
        }
        _ => {} // same vote again, no change
    }

    Ok(())
}

/// Remove feedback for a track entirely (unlike, not dislike).
/// Reverses the previous vote in artist_feedback counts.
pub fn remove_feedback(conn: &Connection, artist: &str, track: &str) -> Result<()> {
    let artist = artist.to_lowercase();
    let track  = track.to_lowercase();

    let previous: Option<bool> = conn.query_row(
        "SELECT liked FROM track_feedback WHERE artist_name=?1 AND track_name=?2",
        params![artist, track],
        |r| r.get::<_, i32>(0),
    ).ok().map(|v| v != 0);

    // Delete the feedback row.
    conn.execute(
        "DELETE FROM track_feedback WHERE artist_name=?1 AND track_name=?2",
        params![artist, track],
    )?;

    // Reverse the count.
    match previous {
        Some(true) => {
            conn.execute(
                "UPDATE artist_feedback SET likes=MAX(0,likes-1) WHERE artist_name=?1",
                params![artist],
            )?;
        }
        Some(false) => {
            conn.execute(
                "UPDATE artist_feedback SET dislikes=MAX(0,dislikes-1) WHERE artist_name=?1",
                params![artist],
            )?;
        }
        None => {} // nothing to reverse
    }

    Ok(())
}

/// Recalculate the final_score for a single artist after a feedback change.
/// Uses the same unified formula as `recalculate_all_scores`.
pub fn recalculate_artist_score(
    conn: &Connection,
    artist: &str,
    like_bonus_flat: f64,
    dislike_pct: f64,
    multi_source_bonus_pct: f64,
) -> Result<()> {
    let artist = artist.to_lowercase();

    // Sync feedback counts from artist_feedback into artists table.
    conn.execute(
        "UPDATE artists SET
           likes = COALESCE((SELECT likes FROM artist_feedback WHERE artist_feedback.artist_name = ?1), 0),
           dislikes = COALESCE((SELECT dislikes FROM artist_feedback WHERE artist_feedback.artist_name = ?1), 0)
         WHERE artist_name = ?1",
        params![artist],
    )?;

    // Recalculate final_score with same formula as recalculate_all_scores.
    conn.execute(
        "UPDATE artists SET final_score = ROUND(
           (playcount_score * year_bonus
            + similarity_score * (1.0 + ?2 * MAX(0, similarity_appearances - 1))
            + likes * ?3)
           * MAX(0.0, 1.0 - dislikes * ?4),
         2)
         WHERE artist_name = ?1",
        params![artist, multi_source_bonus_pct, like_bonus_flat, dislike_pct],
    )?;

    Ok(())
}

/// Returns the feedback for a specific track: Some(true)=liked, Some(false)=disliked, None=no feedback.
pub fn get_track_feedback(conn: &Connection, artist: &str, track: &str) -> Result<Option<bool>> {
    let result = conn.query_row(
        "SELECT liked FROM track_feedback WHERE artist_name=?1 AND track_name=?2",
        params![artist.to_lowercase(), track.to_lowercase()],
        |r| r.get::<_, i32>(0),
    );
    match result {
        Ok(v) => Ok(Some(v != 0)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

/// Returns a set of lowercase artist names that have at least one liked track.
pub fn get_liked_artist_names(conn: &Connection) -> Result<std::collections::HashSet<String>> {
    let mut stmt = conn.prepare(
        "SELECT LOWER(artist_name) FROM artist_feedback WHERE likes > 0",
    )?;
    let rows = stmt.query_map([], |r| r.get::<_, String>(0))?;
    Ok(rows.flatten().collect())
}

/// Returns a HashSet of (artist_name, track_name) pairs that were disliked.
pub fn get_disliked_tracks(conn: &Connection) -> Result<std::collections::HashSet<(String, String)>> {
    let mut stmt = conn.prepare(
        "SELECT artist_name, track_name FROM track_feedback WHERE liked=0",
    )?;
    let rows = stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?;
    Ok(rows.flatten().collect())
}


// ── Artist chart entries ──────────────────────────────────────────────────────

pub fn upsert_artist_chart_entry(
    conn: &Connection,
    artist_name: &str,
    mbid: &str,
    playcount: u64,
    period_label: &str,
    fetched_at: i64,
) -> Result<()> {
    let artist_name = artist_name.to_lowercase();
    conn.execute(
        "INSERT INTO artist_chart_entries (artist_name, mbid, playcount, period_label, fetched_at)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(artist_name, period_label) DO UPDATE SET
           mbid=excluded.mbid, playcount=excluded.playcount, fetched_at=excluded.fetched_at",
        params![artist_name, mbid, playcount as i64, period_label, fetched_at],
    )?;
    Ok(())
}

/// Returns true if any entries exist for this period label (used to skip re-fetching past years).
pub fn is_period_synced(conn: &Connection, period_label: &str) -> Result<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM artist_chart_entries WHERE period_label=?1",
        params![period_label],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

/// Returns all (artist_name, mbid, playcount, period_label) rows for year-numeric periods only.
/// Year periods are those whose label is a 4-digit number (e.g. "2007").
pub fn get_all_year_chart_entries(
    conn: &Connection,
) -> Result<Vec<(String, String, u64, String)>> {
    let mut stmt = conn.prepare(
        "SELECT artist_name, mbid, playcount, period_label
         FROM artist_chart_entries
         WHERE period_label GLOB '[0-9][0-9][0-9][0-9]'
         ORDER BY artist_name, period_label",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i64>(2)? as u64,
            row.get::<_, String>(3)?,
        ))
    })?;
    Ok(rows.filter_map(|r| r.ok()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── upsert_artist_external round-trip ────────────────────────────────────

    #[test]
    fn upsert_and_rank_artist() {
        let conn = open_mem();
        upsert_artist_external(&conn, "radiohead", 5000, 10, 42.5, 1.1, 0).unwrap();
        let ranked = get_all_artists_ranked(&conn).unwrap();
        assert_eq!(ranked.len(), 1);
        let (name, source, _score, playcount, years, _pc, _sim, _likes, _dislikes) = &ranked[0];
        assert_eq!(name, "radiohead");
        assert_eq!(source, "external");
        assert_eq!(*playcount, 5000);
        assert_eq!(*years, 10);
    }

    // ── upsert_artist_top_track round-trip ───────────────────────────────────

    #[test]
    fn upsert_and_read_top_track() {
        let conn = open_mem();
        upsert_artist_top_track(&conn, "Portishead", "Glory Box", "", 200, 100, 1, 0).unwrap();
        let tracks = get_all_artist_top_tracks(&conn).unwrap();
        let artist_tracks = tracks.get("portishead").expect("artist should be present");
        assert_eq!(artist_tracks.len(), 1);
        assert_eq!(artist_tracks[0].0, "Glory Box");
        assert_eq!(artist_tracks[0].1, 1u32);
    }

    // ── get_scoreable_artists_with_tracks filtering ──────────────────────────

    #[test]
    fn scoreable_requires_score_and_track() {
        let conn = open_mem();

        // Artist with score but no track → should NOT appear
        upsert_artist_external(&conn, "notracks", 100, 3, 5.0, 1.0, 0).unwrap();
        conn.execute(
            "UPDATE artists SET final_score = 5.0 WHERE artist_name = 'notracks'", [],
        ).unwrap();

        // Artist with score AND track → SHOULD appear
        upsert_artist_external(&conn, "hastracks", 200, 4, 8.0, 1.0, 0).unwrap();
        conn.execute(
            "UPDATE artists SET final_score = 8.0 WHERE artist_name = 'hastracks'", [],
        ).unwrap();
        upsert_artist_top_track(&conn, "hastracks", "A Song", "", 50, 30, 1, 0).unwrap();

        let scoreable = get_scoreable_artists_with_tracks(&conn).unwrap();
        assert_eq!(scoreable.len(), 1);
        assert_eq!(scoreable[0].0, "hastracks");
    }

    // ── recalculate_all_scores formula ───────────────────────────────────────

    #[test]
    fn recalculate_scores_matches_formula() {
        let conn = open_mem();

        // Insert artist with known components.
        upsert_artist_external(&conn, "testartist", 0, 0, 10.0, 1.05, 0).unwrap();
        conn.execute(
            "UPDATE artists SET
               similarity_score = 5.0,
               similarity_appearances = 2,
               likes = 1,
               dislikes = 0
             WHERE artist_name = 'testartist'",
            [],
        ).unwrap();

        recalculate_all_scores(&conn, 3.0, 0.10, 0.05).unwrap();

        let ranked = get_all_artists_ranked(&conn).unwrap();
        let score = ranked[0].2;

        // formula:
        //   base = 10.0 * 1.05 + 5.0 * (1 + 0.05 * max(0, 2-1))
        //        = 10.5 + 5.0 * 1.05 = 10.5 + 5.25 = 15.75
        //   bonus   = 1 * 3.0 = 3.0
        //   penalty = max(0, 1.0 - 0 * 0.10) = 1.0
        //   final   = ROUND((15.75 + 3.0) * 1.0, 2) = 18.75
        assert!((score - 18.75).abs() < 0.01, "expected 18.75, got {}", score);
    }
}
