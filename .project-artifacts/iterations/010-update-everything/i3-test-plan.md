# Iteration 10 Test Plan — Combined "Update everything" action (EP-9)

## Scenarios

### Unit — Component — In-process

#### SC-1 · UpdateAll sets step field on each activation (AC-2)
**Level:** Component  
**Given** the dispatcher receives `UpdateAll`  
**When** it activates each step  
**Then** `active.step` is `Some((1,4))`, `Some((2,4))`, `Some((3,4))`, `Some((4,4))`  
**And** `active.kind` matches the corresponding `OperationKind` in order  

#### SC-2 · UpdateAll stops chain on first failure (AC-3)
**Level:** Component  
**Given** the dispatcher receives `UpdateAll`  
**When** step 2 (Expand) returns `Err`  
**Then** `last_result` is `Failed("Step 2/4 (Expand similar artists) failed: …")`  
**And** steps 3 and 4 are never started (`active` is cleared)  

#### SC-3 · UpdateAll sets Ok result after full success (AC-4)
**Level:** Component  
**Given** all four `run_operation` calls return `Ok`  
**When** the chain completes  
**Then** `last_result` is `Ok("Update complete (4 steps)")`  
**And** `active` is `None`  

#### SC-4 · Run (single-op) still works after step field added (AC-5)
**Level:** Component  
**Given** dispatcher receives `Run(Score)`  
**When** the operation activates  
**Then** `active.step` is `None`  

#### SC-5 · Existing StateReporter tests still pass (AC-6 / regression)
**Level:** Unit  
`stage_resets_current_and_total`, `tick_updates_progress`,
`reporter_is_noop_when_active_is_none` — all must pass unchanged.

### System-integration / E2E / out-of-process

No out-of-process scenarios for this epic. The feature is pure in-process state
machinery; AC-1 (button + live progress) is verified by Integration smoke only.

## AC coverage

| AC | Scenario(s) |
|----|-------------|
| AC-1 | Integration smoke |
| AC-2 | SC-1 |
| AC-3 | SC-2 |
| AC-4 | SC-3 |
| AC-5 | SC-4 + Integration smoke |
| AC-6 | `cargo build` warning count |
