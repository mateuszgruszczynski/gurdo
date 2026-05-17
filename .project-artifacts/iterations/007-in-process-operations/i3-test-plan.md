# Iteration 7 Test Plan — In-process operations + progress (EP-7)

## BDD scenarios

### S-01 — StateReporter stage resets current and total (Unit / AC-1)
Given an OperationsState with active = Some(ActiveOperation{current:5, total:Some(10)})  
When StateReporter::stage("New stage") is called  
Then active.stage == "New stage", active.current == 0, active.total == None

### S-02 — StateReporter tick updates progress (Unit / AC-1)
Given an active OperationsState  
When StateReporter::tick(42, Some(100)) is called  
Then active.current == 42, active.total == Some(100)

### S-03 — StateReporter is a no-op when active is None (Unit / AC-1)
Given OperationsState with active = None  
When stage/tick/message are called  
Then no panic, state unchanged

### S-04 — ops_dispatcher_loop: active set on start, cleared on finish (Component / AC-2, AC-3)
Given a cmd channel and a stub OperationsState  
When OperationCommand::Run(Score) is sent  
Then ops.active is Some during execution and None after; ops.last_result is Some(Ok(...))

### S-05 — ops_dispatcher_loop: failed operation stores Failed result (Component / AC-3, AC-4)
Given run_operation returns Err  
When the command completes  
Then ops.last_result == Some(Failed(error_string))

### S-06 — Data buttons disabled while active (UI regression / AC-2)
Given ops.active.is_some()  
When settings::render is called  
Then all 4 Data buttons and Login button are wrapped in add_enabled_ui(false, …)

### S-07 — No new warnings (cross-cutting / AC-7)
Given the pre-existing 53-warning baseline  
When `cargo build 2>&1 | grep "^warning"` is run  
Then the count does not exceed 53

## Regression scenarios (touches)

### R-01 — polling_loop unaffected by ops_dispatcher_loop running concurrently (AC-6)
Verified by: `cargo build` clean + manual smoke that playback polling emits no new errors.

## Level assignments

| Scenario | Level | Runs in |
|---|---|---|
| S-01 – S-03 | Unit | Development (in-process) |
| S-04 – S-05 | Component | Development (in-process, tokio test) |
| S-06 | System-integration | Integration (manual) |
| S-07 | System-integration | Integration (cargo build) |
| R-01 | System-integration | Integration (manual) |
