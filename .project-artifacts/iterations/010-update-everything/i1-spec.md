# Iteration 10 Spec — Combined "Update everything" action (EP-9)

*Epic: EP-9 · Phase: Refinement · Date: 2026-05-12*

---

## Problem

Running a full data refresh requires clicking four separate buttons in order: Sync Last.fm →
Expand → Fetch Tracks → Recalculate Scores. EP-9 adds one button that chains all four
sequentially with a "Step N/4" progress label, stopping on the first failure.

---

## Scope

### In scope

**`src/ui/state.rs`**
- Add `UpdateAll` variant to `OperationCommand`.
- Add `step: Option<(u8, u8)>` field to `ActiveOperation` — `Some((current, total))` when
  running a multi-step sequence; `None` for single operations.

**`src/ui/ops.rs`**
- Handle `OperationCommand::UpdateAll` in `ops_dispatcher_loop`:
  - Iterate through `[SyncLastfm, Expand, FetchTracks, Score]`.
  - For each step `(i, kind)`, set `active` with `step: Some((i as u8 + 1, 4))` and run
    `run_operation`. On `Err`, set `last_result = Failed("Step N/4 (<name>) failed: <e>")`,
    clear `active`, and stop the chain.
  - On full success, set `last_result = Ok("Update complete (4 steps)")`.

**`src/ui/settings.rs`**
- "Update everything" button in the Data section (before the individual buttons);
  disabled when `busy`.
- Progress label: when `active.step` is `Some((n, t))`, prefix with `Step n/t: `.

### Out of scope

- Parallel execution of any steps.
- Cancellation mid-chain.
- Adding steps beyond the four data operations.

---

## Acceptance criteria

| # | Criterion |
|---|---|
| AC-1 | "Update everything" button runs Sync → Expand → Fetch → Score in order with live progress. |
| AC-2 | Progress shows "Step N/4: \<kind\>: \<stage\>" while running. |
| AC-3 | On failure of any step, subsequent steps are skipped; `last_result` shows which step failed and the error. |
| AC-4 | On full success, `last_result` shows "✓ Update complete (4 steps)". |
| AC-5 | All 5 individual buttons (4 Data + Login) remain functional and unchanged. |
| AC-6 | `cargo build` produces zero new warnings beyond the 53 baseline. |

---

## Implementation notes

### `ActiveOperation.step`

```rust
#[derive(Clone)]
pub struct ActiveOperation {
    pub kind:    OperationKind,
    pub step:    Option<(u8, u8)>,   // (current, total) for multi-step sequences
    pub stage:   String,
    pub current: u64,
    pub total:   Option<u64>,
    #[allow(dead_code)]
    pub message: String,
}
```

Existing single-op dispatch sets `step: None`.

### Dispatcher UpdateAll branch

```rust
OperationCommand::UpdateAll => {
    let steps = [
        OperationKind::SyncLastfm,
        OperationKind::Expand,
        OperationKind::FetchTracks,
        OperationKind::Score,
    ];
    let total = steps.len() as u8;
    for (i, kind) in steps.iter().enumerate() {
        {
            let mut o = ops.lock().unwrap();
            o.active = Some(ActiveOperation {
                kind:    kind.clone(),
                step:    Some((i as u8 + 1, total)),
                stage:   String::new(),
                current: 0,
                total:   None,
                message: String::new(),
            });
        }
        let reporter = StateReporter { ops: Arc::clone(&ops) };
        let config   = shared_config.lock().unwrap().clone();
        if let Err(e) = run_operation(kind.clone(), &config, &reporter).await {
            let mut o = ops.lock().unwrap();
            o.active      = None;
            o.last_result = Some(OperationResult::Failed(
                format!("Step {}/{} ({}) failed: {}", i + 1, total, kind.label(), e)
            ));
            return;
        }
    }
    let mut o = ops.lock().unwrap();
    o.active      = None;
    o.last_result = Some(OperationResult::Ok("Update complete (4 steps)".to_string()));
}
```

### Settings progress label

```rust
if let Some(active) = &ops.active {
    let step_prefix = active.step
        .map(|(n, t)| format!("Step {}/{}: ", n, t))
        .unwrap_or_default();
    ui.label(format!("{}{}: {}", step_prefix, active.kind.label(), active.stage));
    // ... existing progress count ...
}
```

### Button placement

```
[Update everything]   ← new, full-width or left-aligned, above the row of 4
[Sync Last.fm] [Expand similar artists] [Fetch top tracks] [Recalculate scores]
```

All five wrapped in `add_enabled_ui(!busy, …)`.

---

## Files changed (expected)

| File | Change |
|---|---|
| `src/ui/state.rs` | Add `UpdateAll` to `OperationCommand`; add `step` field to `ActiveOperation` |
| `src/ui/ops.rs` | Handle `OperationCommand::UpdateAll` in dispatcher loop; update `ActiveOperation` construction in `Run` branch to set `step: None` |
| `src/ui/settings.rs` | "Update everything" button; step prefix in progress label |
