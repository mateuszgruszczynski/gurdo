# Iteration 8 Development — Full config-knob exposure (EP-8)

## Files changed

| File | Change |
|---|---|
| `src/config.rs` | `impl Default for SyncConfig`; `impl Default for EngineConfig`; removed `#[allow(dead_code)]` from `Config::save` |
| `src/ui/knobs.rs` | `KnobSpec` struct + 4 static slices (`SYNC_KNOBS`, `ENGINE_KNOBS`, `ARTIST_SCORING_KNOBS`, `RECOMMEND_KNOBS`) — metadata for future epics; suppressed with `#[allow(dead_code)]` |
| `src/ui/mod.rs` | Added `settings_draft: Arc<Mutex<Option<Config>>>` to `GurdoApp` init |
| `src/ui/player.rs` | Removed `#[allow(dead_code)]` from `config_path`; added `settings_draft` field; updated settings viewport closure to capture and pass `shared_config`, `settings_draft`, `config_path`; removed unused `super::ops` import |
| `src/ui/settings.rs` | Revised `render` signature; filled Recommendations, Engine, Artist Scoring, Sync sections with `DragValue` knobs + per-field Reset; Appearance section (read-only); Save/Discard block; draft lifecycle |
| `src/sync/expand.rs` | Deleted `const TRACKS_PER_ARTIST`; added `config: &Config` param to `fetch_artist_tracks`; reads `config.engine.artist_top_tracks_limit` |
| `src/ui/ops.rs` | Pass `&config` to `fetch_artist_tracks` call |
| `src/main.rs` | Pass `&config` to `fetch_artist_tracks` call |

## Key decisions

- `knobs.rs` statics are metadata infrastructure for EP-10 (recommendation preview); they're
  suppressed with `#[allow(dead_code)]` rather than deleted since they document the knob set.
- Draft lifecycle: `None` = clean (display shared_config); `Some(draft)` = dirty edits pending.
  Any knob change → clone shared_config into draft → apply change. Save → write + update
  shared_config + clear draft. Discard → clear draft.
- `token_exists` call moved from player.rs into settings.rs to avoid the unused `super::ops`
  import that appeared when settings closure no longer needed `ops::token_exists` inline.

## In-process tests

3/3 EP-7 unit tests still pass (no regressions). No new unit tests added (draft/save logic
is a thin closure wrapper; covered by Integration smoke).

## Self-review

- No new warnings (53 baseline maintained)
- `TRACKS_PER_ARTIST` constant fully removed; confirmed with grep
- `Config::save` and `config_path` no longer dead code
- Save failure logs via `tracing::error!` rather than panicking
