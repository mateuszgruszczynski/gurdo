# Iteration 13 Test Plan — Test scaffolding (EP-12)

This iteration IS the test plan — the deliverable is the tests themselves.

## Scenarios (= the tests being added)

| ID | Location | Description | AC |
|----|----------|-------------|----|
| T-1 | `recommend.rs` | `weighted_sample` seeded → deterministic index | AC-1 |
| T-2 | `recommend.rs` | `weighted_sample` single weight → always index 0 | AC-1 |
| T-3 | `recommend.rs` | `weighted_sample` equal weights → valid index each call | AC-1 |
| T-4 | `recommend.rs` | `generate_recommendations` on seeded in-memory fixture returns non-empty with score > 0 | AC-6 |
| T-5 | `queries.rs` | `upsert_artist_external` → `get_all_artists_ranked` round-trip | AC-3 |
| T-6 | `queries.rs` | `upsert_artist_top_track` → `get_all_artist_top_tracks` round-trip | AC-4 |
| T-7 | `queries.rs` | `get_scoreable_artists_with_tracks` filters correctly | AC-5 |
| T-8 | `queries.rs` | `recalculate_all_scores` formula matches documented formula | AC-2 |
| T-9 | `progress.rs` | `RecordingReporter` records stage/tick/finish in order | AC-7 |

All scenarios run in-process with `cargo test`. No network, no display, no real files.

## Existing tests (must keep passing)

- `config::tests::secrets_path_is_sibling` (AC from EP-11)
- `config::tests::load_overlays_secrets_when_present` (AC from EP-11)
- `config::tests::load_uses_config_values_when_secrets_absent` (AC from EP-11)
- `ui::ops::tests::stage_resets_current_and_total` (AC from EP-7)
- `ui::ops::tests::tick_updates_progress` (AC from EP-7)
- `ui::ops::tests::reporter_is_noop_when_active_is_none` (AC from EP-7)
- `tests::parse_config_arg_default` (AC from EP-9)
