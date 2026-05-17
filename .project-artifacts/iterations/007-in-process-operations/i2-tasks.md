# Iteration 7 Tasks — In-process operations + progress (EP-7)

*Auto-continued from Decomposition — all tasks map to ACs or standard cross-cutting categories.*

## Infrastructure — state types

- [x] T-01 Add `OperationsState`, `ActiveOperation`, `OperationKind`, `OperationResult`, `OperationCommand` to `src/ui/state.rs` (AC-1, AC-2, AC-3)

## New module — ops.rs

- [x] T-02 Define `ProgressReporter` trait (`stage`, `tick`, `message`, `finish`) in `src/ui/ops.rs` (AC-1)
- [x] T-03 Implement `StateReporter` (writes to `Arc<Mutex<OperationsState>>`) in `src/ui/ops.rs` (AC-1)
- [x] T-04 Implement `run_operation` dispatcher fn (routes `OperationKind` → sync/engine fn or OAuth) in `src/ui/ops.rs` (AC-1, AC-5)
- [x] T-05 Implement `ops_dispatcher_loop` tokio task in `src/ui/ops.rs` (AC-1, AC-2, AC-3, AC-6)

## Sync/engine functions — add progress reporting

- [x] T-06 Add `progress: &dyn ProgressReporter` to `sync_lastfm`; emit stage/tick/finish in `src/sync/mod.rs` (AC-1)
- [x] T-07 Add `progress` to `expand_artists`; emit stage/tick/finish in `src/sync/expand.rs` (AC-1)
- [x] T-08 Add `progress` to `fetch_artist_tracks`; emit stage/tick/finish in `src/sync/expand.rs` (AC-1)
- [x] T-09 Add `progress` to `score_artists`; emit stage/finish in `src/engine/artist_scores.rs` (AC-1)

## UI wiring

- [x] T-10 Add `ops_state: Arc<Mutex<OperationsState>>` + `ops_cmd_tx` fields to `GurdoApp`; forward to `settings::render` in `src/ui/player.rs` (AC-1, AC-2)
- [x] T-11 Create ops channel + state; run `ops_dispatcher_loop` via `tokio::join!` in `src/ui/mod.rs` (AC-6)
- [x] T-12 Fill Settings Data section (4 buttons + progress panel + last-result line) in `src/ui/settings.rs` (AC-1, AC-2, AC-3, AC-4)
- [x] T-13 Fill Settings Spotify section (Login button + status line) in `src/ui/settings.rs` (AC-5)
- [x] T-14 Add `ctx.request_repaint_after(100ms)` while op active in `src/ui/settings.rs` (AC-1)

## Cross-cutting

- [x] T-15 Zero new warnings beyond pre-existing 53 baseline (AC-7)
