# Iteration 7 Development — In-process operations + progress (EP-7)

## Files changed

| File | Change |
|---|---|
| `src/progress.rs` | New: `ProgressReporter` trait + `NullProgress` no-op; placed in top-level module so both sync and ui can import without circular deps |
| `src/main.rs` | `mod progress;`; pass `&NullProgress` to modified CLI callers of sync/engine functions |
| `src/ui/state.rs` | Added `OperationsState`, `ActiveOperation`, `OperationKind`, `OperationResult`, `OperationCommand`; `#[allow(dead_code)]` on `ActiveOperation.message` |
| `src/ui/ops.rs` | Full implementation: `StateReporter`, `token_exists`, `run_operation`, `ops_dispatcher_loop`; 3 unit tests |
| `src/ui/mod.rs` | Create ops channel + state; run `ops_dispatcher_loop` via `tokio::join!` alongside `polling_loop`; pass fields to `GurdoApp` |
| `src/ui/player.rs` | Added `ops_state` + `ops_cmd_tx` fields to `GurdoApp`; compute `spotify_connected` and capture in settings closure |
| `src/ui/settings.rs` | Filled Data section (4 buttons, progress panel, last-result); Spotify section (Login, status); `request_repaint_after(100ms)` while active |
| `src/sync/mod.rs` | Added `progress: &dyn ProgressReporter` to `sync_lastfm`; emits stage/tick per track and tag, stage for artist-history and scoring |
| `src/sync/artists.rs` | Added `progress` to `sync_artist_history`; ticks per year |
| `src/sync/expand.rs` | Added `progress` to `expand_artists` (stage + tick per artist) and `fetch_artist_tracks` (stage + tick per artist) |
| `src/engine/artist_scores.rs` | Added `progress` to `score_artists`; emits stage + finish |

## Key decisions

- `ProgressReporter` trait lives in `src/progress.rs` (not `src/ui/ops.rs`) to avoid a circular dependency: `ui::ops` imports `sync`, and `sync` needs the trait.
- `run_operation` returns `Result<String>` with a fixed summary string; sync functions keep returning `Result<()>` (no API break for CLI callers).
- `ActiveOperation.message` and `ProgressReporter::message` suppressed with `#[allow(dead_code)]` — wired for future use, no callers yet.

## In-process tests (S-01 to S-03)

| Scenario | Result | AC |
|---|---|---|
| S-01: stage resets current/total | PASS | AC-1 |
| S-02: tick updates progress | PASS | AC-1 |
| S-03: reporter noop when active=None | PASS | AC-1 |

## External interfaces wired

- `ops_dispatcher_loop` runs as second task in the tokio runtime alongside `polling_loop` (AC-6)
- Settings viewport sends `OperationCommand::Run(kind)` over `ops_cmd_tx` UnboundedSender

## Self-review

- No new warnings introduced (53 baseline maintained)
- No secrets in code; no test mocks for DB (sync functions create their own connection)
- AC-4: errors from ops appear in Settings `last_result` line, not player modals — ops errors never write to `PlayerState.error`
