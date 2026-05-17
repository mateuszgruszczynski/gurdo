# Iteration 13 Decomposition — Test scaffolding (EP-12)

## Tasks

### DEV-1 — `src/progress.rs`: RecordingReporter + ordering test
- Add `RecordingReporter` struct under `#[cfg(test)]`.
- Add one test: stage/tick/finish events recorded in order.
- **AC:** AC-7

### DEV-2 — `src/engine/recommend.rs`: weighted_sample tests
- Add `#[cfg(test)]` module with tests for determinism, single-weight, equal-weights.
- **AC:** AC-1

### DEV-3 — `src/engine/recommend.rs`: generate_recommendations fixture test
- In-memory SQLite, seed 2 artists + 3 tracks + scores via direct SQL UPDATE.
- Assert non-empty result with score > 0.
- **AC:** AC-6

### DEV-4 — `src/db/queries.rs`: round-trip tests
- `open_mem()` helper: `Connection::open_in_memory()` + `init_db`.
- `upsert_artist_external` → `get_all_artists_ranked` round-trip.
- `upsert_artist_top_track` → `get_all_artist_top_tracks` round-trip.
- `get_scoreable_artists_with_tracks` filtering test.
- `recalculate_all_scores` formula test.
- **AC:** AC-2, AC-3, AC-4, AC-5

### Cross-cutting — Warning budget
- `cargo test` green; `cargo build` ≤ 53 warnings.
- **AC:** AC-8

## Decision notes

All tasks map to ACs. No production code changes. Auto-continue to Test Plan.
