# Iteration 11 Development — Recommendation preview-while-tuning (EP-10)

## Files changed

| File | Change |
|------|--------|
| `src/engine/recommend.rs` | Return type changed to `Vec<(String, String, f64)>`; score pushed as third element from `artists[artist_idx].1` |
| `src/ui/poll.rs` | Both `for (artist, track)` loops → `for (artist, track, _score)` (two call sites; second had different indentation so `replace_all` only caught one — fixed individually) |
| `src/ui/state.rs` | `preview_results: Option<Vec<(String, String, f64)>>` added to `OperationsState`; `Preview` variant added to `OperationCommand` |
| `src/ui/ops.rs` | `settings_draft: Arc<Mutex<Option<Config>>>` param added to `ops_dispatcher_loop`; `Preview` branch reads draft-or-live config, opens DB, calls `generate_recommendations`, stores in `ops.preview_results` or sets `last_result = Failed`; test constructors updated with `preview_results: None` |
| `src/ui/mod.rs` | `settings_draft` Arc created before thread spawn; passed to both `ops_dispatcher_loop` and `GurdoApp` |
| `src/ui/settings.rs` | Preview button (disabled when busy) after Recommendations knobs; bounded 300px scroll area renders `(artist — track | score)`; Discard clears `preview_results` |

## In-process tests

| Scenario | Level | Result | AC |
|----------|-------|--------|----|
| SC-3 `OperationsState::default().preview_results == None` | Unit (implicit via Default) | PASS | AC-6 |
| SC-4 `stage_resets_current_and_total` | Unit | PASS | AC-8 |
| SC-4 `tick_updates_progress` | Unit | PASS | AC-8 |
| SC-4 `reporter_is_noop_when_active_is_none` | Unit | PASS | AC-8 |

SC-1 (`weighted_sample` determinism) and SC-2 (score in result with SQLite fixture) deferred
to EP-12 test scaffolding.

## Key decisions / issues

- `replace_all: true` only caught the first of two `for (artist, track)` loops in `poll.rs`
  because the two loops have different leading indentation (16 vs 8 spaces). Second loop fixed
  individually.
- `mod.rs` restructured: `settings_draft` Arc created before the background thread spawn so it
  can be cloned for the dispatcher and still passed into `GurdoApp`.
- Preview does not set `active` — instant operation; no progress indicator needed.
- `preview_results` persists until re-run or Discard. No "(draft)" stale label per spec.

## Warnings

`cargo build --release`: 53 (baseline maintained).
