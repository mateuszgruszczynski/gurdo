# Iteration 11 Decomposition — Recommendation preview-while-tuning (EP-10)

## Tasks

### DEV-1 — `src/engine/recommend.rs`: Return score in results
- Change return type to `Result<Vec<(String, String, f64)>>`.
- Push `artists[artist_idx].1` as the third tuple element.
- **AC:** AC-2, AC-3, AC-7

### DEV-2 — `src/ui/poll.rs`: Update call sites
- Two `for (artist, track) in` loops → `for (artist, track, _score) in`.
- **AC:** AC-7

### DEV-3 — `src/ui/state.rs`: Add `preview_results` + `Preview` variant
- Add `pub preview_results: Option<Vec<(String, String, f64)>>` to `OperationsState`.
- Update `Default` derive (field defaults to `None`).
- Add `Preview` variant to `OperationCommand`.
- **AC:** AC-1, AC-3, AC-6

### DEV-4 — `src/ui/ops.rs`: Handle `Preview`; add `settings_draft` param
- Add `settings_draft: Arc<Mutex<Option<Config>>>` parameter to `ops_dispatcher_loop`.
- Handle `OperationCommand::Preview` in the match: reads draft-or-live config, opens DB,
  calls `generate_recommendations`, stores into `ops.preview_results` on success or
  `ops.last_result = Failed(…)` on error / empty.
- Import `crate::engine::recommend::generate_recommendations`.
- **AC:** AC-2, AC-4, AC-5

### DEV-5 — `src/ui/mod.rs`: Pass `settings_draft` to dispatcher
- Update the `ops_dispatcher_loop(…)` call to include `Arc::clone(&settings_draft)`.
- **AC:** AC-2

### DEV-6 — `src/ui/settings.rs`: Preview button + results panel; clear on Discard
- In Recommendations section after the three knobs: `add_enabled_ui(!busy)` wrapping
  a "Preview" button that sends `OperationCommand::Preview`.
- After the button: if `ops.preview_results` is `Some(results)`, render a
  `ScrollArea::vertical().max_height(300)` with one row per result.
- In the Discard handler: `ops_state.lock().unwrap().preview_results = None`.
- **AC:** AC-1, AC-3, AC-5, AC-6

### Cross-cutting — Warning budget
- `cargo build` must produce ≤ 53 warnings.
- **AC:** AC-8

## Decision notes

All tasks map directly to ACs. No scope added beyond the spec. Auto-continue to Test Plan.
