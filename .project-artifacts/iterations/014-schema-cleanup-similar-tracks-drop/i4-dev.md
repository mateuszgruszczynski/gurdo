# Iteration 14 Development — Schema cleanup: similar_tracks drop (EP-13)

## Files changed

| File | Change |
|------|--------|
| `src/db/schema.rs` | Removed `CREATE TABLE IF NOT EXISTS similar_tracks` block and `idx_similar_tracks_seed` index; added `DROP TABLE IF EXISTS similar_tracks` to migrations batch |
| `src/db/queries.rs` | Removed `upsert_similar_track`, `get_similar_tracks_for_seed`, `is_track_synced_for_similar` |
| `src/lastfm/client.rs` | Removed `track_similar` method |
| `src/lastfm/models.rs` | Removed `SimilarTracksResponse`, `SimilarTracks`, `SimilarTrack` structs |
| `src/config.rs` | Removed `similar_tracks_limit` field, `default_similar_tracks_limit()` fn, and `Default` entry; updated test fixture TOML strings (both occurrences) |
| `config.toml` | Removed `similar_tracks_limit = 20` line |
| `config.toml.example` | Removed `similar_tracks_limit = 20` line and its comment |

No test code changes; no production behaviour changes.

## Key decisions

- Migration placed in the existing `DROP TABLE IF EXISTS` batch — consistent with how `spotify_saved_tracks`, `recommendations`, etc. were removed.
- `similar_artists` table and all its callers untouched — confirmed live in `expand.rs`.

## Warning budget

`cargo build`: 47 warnings (down from 53 — 6 fewer dead-code warnings from removed items). `cargo test`: 16/16 pass.

## Self-review checklist

- AC-1 ✓ — `CREATE TABLE similar_tracks` absent from schema.rs
- AC-2 ✓ — `DROP TABLE IF EXISTS similar_tracks` in migrations batch
- AC-3 ✓ — three query functions removed
- AC-4 ✓ — `track_similar` + 3 model structs removed
- AC-5 ✓ — `similar_tracks_limit` removed from config.rs, config.toml, config.toml.example
- AC-6 ✓ — 0 new warnings; warning count improved
