# Iteration 1 Spec — Remove similar_tracks dead code (EP-13)

## Problem
The `similar_tracks` table and its associated Rust code were populated by a retired sync pipeline and are never read by any live code path. They add maintenance surface and schema noise with no benefit.

## Goal
Delete all `similar_tracks`-related schema, queries, Last.fm client code, and config fields so the codebase compiles cleanly and the table is dropped from existing databases.

## Acceptance criteria

| ID | Criterion |
|----|-----------|
| AC-1 | `src/db/schema.rs` contains neither `CREATE TABLE IF NOT EXISTS similar_tracks` nor `CREATE INDEX IF NOT EXISTS idx_similar_tracks_seed`. |
| AC-2 | The migrations block in `init_db()` includes `DROP TABLE IF EXISTS similar_tracks;`, so running the app against an existing database drops the table without error. |
| AC-3 | `src/db/queries.rs` contains none of `upsert_similar_track`, `get_similar_tracks_for_seed`, or `is_track_synced_for_similar`. |
| AC-4 | `src/lastfm/client.rs` contains no `track_similar` method, and `src/lastfm/models.rs` contains none of `SimilarTracksResponse`, `SimilarTracks`, or `SimilarTrack`. |
| AC-5 | `src/config.rs` contains no `similar_tracks_limit` field and no `default_similar_tracks_limit` function; both `config.toml` and `config.toml.example` contain no `similar_tracks_limit` key. |
| AC-6 | `cargo build` completes with zero new compiler warnings (the deleted items must not be replaced by unused-import or dead-code warnings elsewhere). |

## Out of scope
- Removing or modifying the `similar_artists` table, its queries, or any code in `expand.rs` or the recommender that uses it.
- Changes to any other sync pipeline code not listed above.
- Data migration beyond the single `DROP TABLE IF EXISTS similar_tracks;` statement.

## Key decisions
- The drop is placed in the existing migrations batch in `init_db()`, consistent with how previous stale tables (`spotify_saved_tracks`, `recommendations`, etc.) were removed.
- No schema version column is introduced; the idempotent `DROP TABLE IF EXISTS` is sufficient.
- `similar_artists` is explicitly out of scope to avoid scope creep — confirmed live via `expand.rs`.
