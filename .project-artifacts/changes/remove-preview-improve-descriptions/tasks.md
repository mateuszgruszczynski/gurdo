# Tasks — Remove recommendation preview + improve settings descriptions

## DEV tasks

- [x] T-1 `src/ui/state.rs` — remove `preview_results` field from `OperationsState`; remove `Preview` variant from `OperationCommand` (AC-2, AC-3)
- [x] T-2 `src/ui/ops.rs` — remove `OperationCommand::Preview` dispatch arm; remove `preview_results: None` from the production `OperationsState` init and from the `make_ops` test helper (AC-3, AC-5, AC-6)
- [x] T-3 `src/ui/settings.rs` — remove Preview button + results scroll panel; replace all 17 knob `desc` strings with new plain-English versions (AC-1, AC-4)

## Cross-cutting

- [x] T-4 `cargo build` ≤ 2 warnings; `cargo test` 16/16 (AC-5, AC-6)
