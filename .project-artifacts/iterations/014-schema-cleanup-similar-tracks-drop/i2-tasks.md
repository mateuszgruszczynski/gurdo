# Iteration 14 Decomposition — Schema cleanup: similar_tracks drop (EP-13)

## Tasks

### DEV-1 — `src/db/schema.rs`: remove table + index, add migration
- Delete `CREATE TABLE IF NOT EXISTS similar_tracks (...)` block.
- Delete `CREATE INDEX IF NOT EXISTS idx_similar_tracks_seed ON similar_tracks(...)` line.
- Add `DROP TABLE IF EXISTS similar_tracks;` to the existing migrations batch.
- **AC:** AC-1, AC-2

### DEV-2 — `src/db/queries.rs`: remove three dead functions
- Remove `upsert_similar_track`.
- Remove `get_similar_tracks_for_seed`.
- Remove `is_track_synced_for_similar`.
- **AC:** AC-3

### DEV-3 — `src/lastfm/client.rs` + `src/lastfm/models.rs`: remove API dead code
- Remove `track_similar` method from `LastfmClient`.
- Remove `SimilarTracksResponse`, `SimilarTracks`, `SimilarTrack` structs.
- **AC:** AC-4

### DEV-4 — `src/config.rs` + `config.toml` + `config.toml.example`: remove config field
- Remove `similar_tracks_limit` field from `EngineConfig`.
- Remove `default_similar_tracks_limit()` fn.
- Remove `similar_tracks_limit` from `EngineConfig::default()`.
- Remove `similar_tracks_limit = 20` line from both TOML files.
- **AC:** AC-5

### Cross-cutting — Warning budget
- `cargo build` ≤ 53 warnings (expect improvement); `cargo test` green.
- **AC:** AC-6
