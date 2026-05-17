# Iteration 13 Development — Test scaffolding (EP-12)

## Files changed

| File | Change |
|------|--------|
| `src/progress.rs` | Added `RecordingReporter` + `recording_reporter_captures_events_in_order` test under `#[cfg(test)]` |
| `src/engine/recommend.rs` | Added `#[cfg(test)]` module: 3 `weighted_sample` tests + 1 `generate_recommendations` fixture test |
| `src/db/queries.rs` | Added `open_mem()` helper + 4 round-trip/formula tests under `#[cfg(test)]` |

No production code changes.

## In-process tests by level / AC

| Test | Level | AC |
|------|-------|----|
| `progress::tests::recording_reporter_captures_events_in_order` | Unit | AC-7 |
| `engine::recommend::tests::weighted_sample_deterministic` | Unit | AC-1 |
| `engine::recommend::tests::weighted_sample_single_weight_always_zero` | Unit | AC-1 |
| `engine::recommend::tests::weighted_sample_equal_weights_all_valid` | Unit | AC-1 |
| `engine::recommend::tests::generate_recommendations_returns_results_with_scores` | Component | AC-6 |
| `db::queries::tests::upsert_and_rank_artist` | Component | AC-3 |
| `db::queries::tests::upsert_and_read_top_track` | Component | AC-4 |
| `db::queries::tests::scoreable_requires_score_and_track` | Component | AC-5 |
| `db::queries::tests::recalculate_scores_matches_formula` | Component | AC-2 |

## Key decisions

- `upsert_artist_external` does not lowercase artist names (unlike `upsert_artist_top_track`). Test fixtures use lowercase names to match what the DB stores, keeping tests deterministic without changing production behaviour.
- `generate_recommendations` fixture seeds scores via raw `UPDATE` after `upsert_artist_external`, matching the real scoring pipeline's two-phase write pattern.
- `RecordingReporter` is `#[cfg(test)]` only and placed in `progress.rs` so any test module can import it from `crate::progress::RecordingReporter`.

## Self-review

- No production code modified — zero regression risk.
- Warning budget: `cargo build` = 53 warnings (unchanged). `cargo test` = 54 (one pre-existing `unused import: super::*` in `main.rs` test, present since EP-9).
- All 16 tests pass (7 pre-existing + 9 new).
