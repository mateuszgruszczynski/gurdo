# Iteration 10 Integration — Combined "Update everything" action (EP-9)

## Build

`cargo build --release` → 53 warnings, 0 errors. ✓

## Smoke

UI runs on host (hybrid build model). No display in dev container — manual AC-1/AC-2
smoke is done by code review of the rendered path:

- `settings.rs:47–48`: "Update everything" button sends `OperationCommand::UpdateAll`.
- `settings.rs:46,65`: all five buttons (incl. "Update everything") wrapped in `add_enabled_ui(!busy, …)`.
- `settings.rs:69–72`: progress label prefixes `Step n/t: ` when `active.step` is `Some`.
- `ops.rs:119–155`: `UpdateAll` branch iterates `[SyncLastfm, Expand, FetchTracks, Score]`,
  sets `step: Some((i+1, 4))` per step; on `Err` clears `active`, sets `Failed` message, returns early;
  on full success sets `Ok("Update complete (4 steps)")`.
- `ops.rs:96–115`: `Run` branch unchanged; sets `step: None`.

## AC pass/fail

| AC | Result | Notes |
|----|--------|-------|
| AC-1 | PASS | Button present; chain implemented in dispatcher |
| AC-2 | PASS | Step prefix `Step n/t: ` in progress label (code review) |
| AC-3 | PASS | `return` after `Err` prevents subsequent steps; `Failed` message includes step + error |
| AC-4 | PASS | `Ok("Update complete (4 steps)")` set after loop exit |
| AC-5 | PASS | `Run` branch and all 4 individual buttons unchanged; Login button unchanged |
| AC-6 | PASS | 53 warnings (baseline) |

## Integration issues

None.

Integration green — continuing with Retrospective.
