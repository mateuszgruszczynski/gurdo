# Iteration 10 Development — Combined "Update everything" action (EP-9)

## Files changed

| File | Change |
|------|--------|
| `src/ui/state.rs` | Added `step: Option<(u8, u8)>` field to `ActiveOperation`; added `UpdateAll` variant to `OperationCommand` |
| `src/ui/ops.rs` | Restructured dispatcher `while let` into `while let Some(cmd) = … { match cmd { Run … UpdateAll … } }`; `Run` branch sets `step: None`; `UpdateAll` branch implements four-step chain; `make_ops` test helper updated with `step: None` |
| `src/ui/settings.rs` | Added "Update everything" button above four-button row; updated progress label to prefix with `Step n/t: ` when `active.step` is `Some` |

## In-process tests

| Scenario | Level | Result | AC |
|----------|-------|--------|----|
| SC-5 `stage_resets_current_and_total` | Unit | PASS | AC-6 |
| SC-5 `tick_updates_progress` | Unit | PASS | AC-6 |
| SC-5 `reporter_is_noop_when_active_is_none` | Unit | PASS | AC-6 |

SC-1 through SC-4 (dispatcher state tests) deferred to Integration smoke — `run_operation`
hits real I/O; EP-12 test scaffolding is the prerequisite for mocking.

## Key decisions

- Dispatcher restructured from `while let Some(OperationCommand::Run(kind))` to
  `while let Some(cmd) { match cmd { … } }` — minimal delta, preserves the existing
  `Run` path identically.
- `UpdateAll` calls `return` on first failure to exit the task immediately (same as
  `ops_dispatcher_loop` returning from `async fn`); `active` is cleared before return.
- Step prefix is computed inline in `settings.rs` with `.unwrap_or_default()` so
  single-op display is unchanged.

## Warnings

`cargo build`: 53 (baseline maintained).  
`cargo test` binary: 54 — extra warning is pre-existing `unused import: super::*` in
`src/main.rs` test module (not introduced by this iteration).

## Self-review

- [x] No scope beyond spec
- [x] No new error handling for impossible paths
- [x] No new comments (WHY already in spec)
- [x] Warning budget maintained
