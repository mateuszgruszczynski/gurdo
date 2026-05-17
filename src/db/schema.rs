use anyhow::Result;
use rusqlite::Connection;

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch("
        -- User's top artists per period (overall, 12month, 6month, 3month, 1month, 7day)
        CREATE TABLE IF NOT EXISTS top_artists (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT NOT NULL,
            mbid        TEXT NOT NULL DEFAULT '',
            playcount   INTEGER NOT NULL DEFAULT 0,
            rank        INTEGER NOT NULL DEFAULT 0,
            period      TEXT NOT NULL,
            fetched_at  INTEGER NOT NULL,
            UNIQUE(name, period)
        );

        -- User's top tracks per period
        CREATE TABLE IF NOT EXISTS top_tracks (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT NOT NULL,
            artist_name TEXT NOT NULL,
            mbid        TEXT NOT NULL DEFAULT '',
            playcount   INTEGER NOT NULL DEFAULT 0,
            rank        INTEGER NOT NULL DEFAULT 0,
            period      TEXT NOT NULL,
            fetched_at  INTEGER NOT NULL,
            UNIQUE(name, artist_name, period)
        );

        -- Tracks the user has explicitly loved
        CREATE TABLE IF NOT EXISTS loved_tracks (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT NOT NULL,
            artist_name TEXT NOT NULL,
            mbid        TEXT NOT NULL DEFAULT '',
            loved_at    INTEGER,
            fetched_at  INTEGER NOT NULL,
            UNIQUE(name, artist_name)
        );

        -- User's top tags (genre fingerprint)
        CREATE TABLE IF NOT EXISTS top_tags (
            id         INTEGER PRIMARY KEY AUTOINCREMENT,
            name       TEXT NOT NULL UNIQUE,
            count      INTEGER NOT NULL DEFAULT 0,
            fetched_at INTEGER NOT NULL
        );

        -- Similar artists cache (keyed by seed artist)
        CREATE TABLE IF NOT EXISTS similar_artists (
            id             INTEGER PRIMARY KEY AUTOINCREMENT,
            seed_artist    TEXT NOT NULL,
            similar_artist TEXT NOT NULL,
            match_score    REAL NOT NULL,
            fetched_at     INTEGER NOT NULL,
            UNIQUE(seed_artist, similar_artist)
        );

        -- Artist top tracks cache
        CREATE TABLE IF NOT EXISTS artist_top_tracks (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            artist_name TEXT NOT NULL,
            track_name  TEXT NOT NULL,
            mbid        TEXT NOT NULL DEFAULT '',
            playcount   INTEGER NOT NULL DEFAULT 0,
            listeners   INTEGER NOT NULL DEFAULT 0,
            rank        INTEGER NOT NULL DEFAULT 0,
            fetched_at  INTEGER NOT NULL,
            UNIQUE(artist_name, track_name)
        );

        -- Tag top tracks cache
        CREATE TABLE IF NOT EXISTS tag_top_tracks (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            tag_name    TEXT NOT NULL,
            track_name  TEXT NOT NULL,
            artist_name TEXT NOT NULL,
            mbid        TEXT NOT NULL DEFAULT '',
            rank        INTEGER NOT NULL DEFAULT 0,
            fetched_at  INTEGER NOT NULL,
            UNIQUE(tag_name, track_name, artist_name)
        );

        -- Spotify URI cache: maps Last.fm (artist, track) → Spotify URI + album art
        -- spotify_uri is NULL when the track was searched but not found on Spotify
        CREATE TABLE IF NOT EXISTS spotify_uri_cache (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            artist_name TEXT NOT NULL,
            track_name  TEXT NOT NULL,
            spotify_uri TEXT,
            album_art_url TEXT,
            fetched_at  INTEGER NOT NULL,
            UNIQUE(artist_name, track_name)
        );

        -- Raw artist playcount data per period label: year labels like 2007, 2008, ...
        -- or recent periods like 1month, 3month.
        -- Year data comes from user.getWeeklyArtistChart; recent from user.getTopArtists.
        CREATE TABLE IF NOT EXISTS artist_chart_entries (
            artist_name  TEXT NOT NULL,
            mbid         TEXT NOT NULL DEFAULT '',
            playcount    INTEGER NOT NULL,
            period_label TEXT NOT NULL,
            fetched_at   INTEGER NOT NULL,
            PRIMARY KEY (artist_name, period_label)
        );

        -- Unified artist pool: both known (from play history) and similar artists.
        -- Score phase writes known artists, expand phase adds similar ones.
        -- final_score is computed at query time from the stored components.
        CREATE TABLE IF NOT EXISTS artists (
            artist_name      TEXT NOT NULL PRIMARY KEY,
            source           TEXT NOT NULL DEFAULT 'external',  -- 'external' | 'similar'
            total_playcount  INTEGER NOT NULL DEFAULT 0,
            years_active     INTEGER NOT NULL DEFAULT 0,
            playcount_score  REAL NOT NULL DEFAULT 0.0,
            year_bonus       REAL NOT NULL DEFAULT 1.0,
            similarity_score REAL NOT NULL DEFAULT 0.0,   -- derived from connections to known artists
            similarity_appearances INTEGER NOT NULL DEFAULT 0,
            best_similar_source TEXT,                      -- highest-scoring parent artist
            likes            INTEGER NOT NULL DEFAULT 0,
            dislikes         INTEGER NOT NULL DEFAULT 0,
            final_score      REAL NOT NULL DEFAULT 0.0,
            generated_at     INTEGER NOT NULL DEFAULT 0
        );

        CREATE INDEX IF NOT EXISTS idx_top_artists_period  ON top_artists(period);
        CREATE INDEX IF NOT EXISTS idx_top_tracks_period   ON top_tracks(period);
        CREATE INDEX IF NOT EXISTS idx_similar_artists_seed ON similar_artists(seed_artist);
        CREATE INDEX IF NOT EXISTS idx_artist_top_tracks    ON artist_top_tracks(artist_name);
        CREATE INDEX IF NOT EXISTS idx_tag_top_tracks       ON tag_top_tracks(tag_name);
        CREATE INDEX IF NOT EXISTS idx_artist_chart_period  ON artist_chart_entries(period_label);
        CREATE INDEX IF NOT EXISTS idx_artists_score        ON artists(final_score DESC);

        -- Per-track like/dislike feedback. Both fields are lowercase.
        -- liked: 1 = liked, 0 = disliked.
        CREATE TABLE IF NOT EXISTS track_feedback (
            artist_name  TEXT NOT NULL,
            track_name   TEXT NOT NULL,
            liked        INTEGER NOT NULL,
            created_at   INTEGER NOT NULL,
            PRIMARY KEY (artist_name, track_name)
        );

        -- Per-artist aggregate feedback counts.
        CREATE TABLE IF NOT EXISTS artist_feedback (
            artist_name  TEXT NOT NULL PRIMARY KEY,
            likes        INTEGER NOT NULL DEFAULT 0,
            dislikes     INTEGER NOT NULL DEFAULT 0
        );
    ")?;

    // Migrations: drop tables from previous versions.
    conn.execute_batch("
        DROP TABLE IF EXISTS spotify_saved_tracks;
        DROP TABLE IF EXISTS recommendations;
        DROP TABLE IF EXISTS artist_scores;
        DROP TABLE IF EXISTS expanded_artists;
        DROP TABLE IF EXISTS similar_tracks;
    ")?;

    // Migration: remove loved_tracks column from artists (re-scored on next run).
    let has_loved_col: bool = conn
        .prepare("SELECT loved_tracks FROM artists LIMIT 0")
        .is_ok();
    if has_loved_col {
        conn.execute("DROP TABLE artists", [])?;
        // Re-run init to recreate without loved_tracks column.
        conn.execute_batch("
            CREATE TABLE IF NOT EXISTS artists (
                artist_name      TEXT NOT NULL PRIMARY KEY,
                source           TEXT NOT NULL DEFAULT 'external',
                total_playcount  INTEGER NOT NULL DEFAULT 0,
                years_active     INTEGER NOT NULL DEFAULT 0,
                playcount_score  REAL NOT NULL DEFAULT 0.0,
                year_bonus       REAL NOT NULL DEFAULT 1.0,
                similarity_score REAL NOT NULL DEFAULT 0.0,
                similarity_appearances INTEGER NOT NULL DEFAULT 0,
                best_similar_source TEXT,
                likes            INTEGER NOT NULL DEFAULT 0,
                dislikes         INTEGER NOT NULL DEFAULT 0,
                final_score      REAL NOT NULL DEFAULT 0.0,
                generated_at     INTEGER NOT NULL DEFAULT 0
            );
            CREATE INDEX IF NOT EXISTS idx_artists_score ON artists(final_score DESC);
        ")?;
    }

    Ok(())
}
