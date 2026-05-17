# Iteration 10 Decomposition — Combined "Update everything" action (EP-9)

## Tasks

### DEV-1 — `src/ui/state.rs`: Add `UpdateAll` variant to `OperationCommand`
- Add `UpdateAll` to the `OperationCommand` enum.
- **AC:** AC-1, AC-5

### DEV-2 — `src/ui/state.rs`: Add `step` field to `ActiveOperation`
- Add `pub step: Option<(u8, u8)>` between `kind` and `stage`.
- **AC:** AC-2

### DEV-3 — `src/ui/ops.rs`: Update `Run` branch — set `step: None`
- In the `while let Some(OperationCommand::Run(kind))` branch, add `step: None` to the
  `ActiveOperation` constructor so existing single-op dispatch compiles after the field is added.
- **AC:** AC-5

### DEV-4 — `src/ui/ops.rs`: Handle `UpdateAll` in dispatcher loop
- Change `while let Some(OperationCommand::Run(kind))` to a `loop { match cmd_rx.recv().await }`
  pattern (or a `match` inside the current while-let) so `UpdateAll` can be handled.
- Implement the four-step chain exactly as specified.
- **AC:** AC-1, AC-3, AC-4

### DEV-5 — `src/ui/settings.rs`: "Update everything" button
- Inside `add_enabled_ui(!busy, …)`, add the button above the existing four-button row.
- **AC:** AC-1, AC-5

### DEV-6 — `src/ui/settings.rs`: Step prefix in progress label
- Replace `format!("{}: {}", active.kind.label(), active.stage)` with the step-prefixed version.
- **AC:** AC-2

### Cross-cutting — Warning budget
- After all changes, `cargo build` must produce ≤ 53 warnings.
- **AC:** AC-6

## Decision notes

All seven tasks map 1-to-1 to ACs or standard cross-cutting. No scope added.  
Auto-continue to Test Plan.
