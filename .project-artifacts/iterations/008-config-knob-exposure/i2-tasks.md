# Iteration 8 Tasks — Full config-knob exposure (EP-8)

## Config infrastructure

- [ ] T-01 `impl Default for EngineConfig` + `impl Default for SyncConfig` in `src/config.rs` (AC-4)
- [ ] T-02 Remove `#[allow(dead_code)]` from `Config::save` and `config_path` field (AC-8)

## Knob metadata

- [ ] T-03 `KnobSpec` struct + `SYNC_KNOBS`, `ENGINE_KNOBS`, `ARTIST_SCORING_KNOBS`, `RECOMMEND_KNOBS` slices in `src/ui/knobs.rs` (AC-1)

## Draft wiring

- [ ] T-04 Add `settings_draft: Arc<Mutex<Option<Config>>>` field to `GurdoApp`; init in `mod.rs` (AC-2, AC-3)
- [ ] T-05 Capture `shared_config`, `settings_draft`, `config_path` in settings viewport closure in `player.rs` (AC-2, AC-3)

## Settings render — sections

- [ ] T-06 Update `settings::render` signature (`+shared_config, +settings_draft, +config_path`; remove `+spotify_connected`) (AC-1)
- [ ] T-07 Implement `knob_row_f64` + `knob_row_u32` + `knob_row_usize` helpers (AC-1, AC-4)
- [ ] T-08 Fill Sync section with 3 knob rows (AC-1)
- [ ] T-09 Fill Engine section with 8 knob rows (AC-1)
- [ ] T-10 Fill Artist Scoring section with 3 knob rows (AC-1)
- [ ] T-11 Fill Recommendations section with 3 knob rows (AC-1)
- [ ] T-12 Save / Discard button block; dirty detection (AC-2, AC-3, AC-5)
- [ ] T-13 Appearance section (read-only paths + metadata) (AC-6)

## TRACKS_PER_ARTIST fix

- [ ] T-14 Delete `const TRACKS_PER_ARTIST`; add `config: &Config` param to `fetch_artist_tracks`; use `config.engine.artist_top_tracks_limit` (AC-7)
- [ ] T-15 Update callsites in `ops.rs` and `main.rs` (AC-7)

## Cross-cutting

- [ ] T-16 Zero new warnings beyond 53 baseline (AC-8)
