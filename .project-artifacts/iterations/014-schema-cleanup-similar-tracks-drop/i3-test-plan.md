# Iteration 14 Test Plan — Schema cleanup: similar_tracks drop (EP-13)

All verification is structural (grep / compile) — no new test scenarios needed because:
- AC-1/AC-2/AC-3/AC-4/AC-5 are negative assertions (things that must be absent); confirmed by `cargo build` succeeding without dead-code warnings for the removed items.
- AC-6 (warning budget) is verified by `cargo build` output.
- The existing 16 tests act as regression guard: if any removed function was actually called, compilation will fail.

## Regression scenarios (must keep passing)

All 16 existing tests must continue to pass unchanged after this epic.

## Structural checks (in-CI grep, run manually)

| Check | Command | Expected |
|-------|---------|----------|
| No similar_tracks table | `grep -r "similar_tracks" src/db/schema.rs` | no output |
| Migration present | `grep "DROP TABLE IF EXISTS similar_tracks" src/db/schema.rs` | 1 match |
| No dead query fns | `grep -E "upsert_similar_track\|get_similar_tracks_for_seed\|is_track_synced_for_similar" src/db/queries.rs` | no output |
| No lastfm dead code | `grep -E "track_similar\|SimilarTrack" src/lastfm/` | no output |
| No config field | `grep "similar_tracks_limit" src/config.rs config.toml config.toml.example` | no output |
