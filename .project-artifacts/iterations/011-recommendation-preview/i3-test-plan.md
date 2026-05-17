# Iteration 11 Test Plan — Recommendation preview-while-tuning (EP-10)

## Scenarios

### Unit / Component — In-process

#### SC-1 · `weighted_sample` determinism (regression, AC-8)
**Level:** Unit  
**Given** a fixed RNG seed and a known weight slice  
**When** `weighted_sample` is called  
**Then** it returns the expected index  
*(Protects against accidental changes to `recommend.rs` internals.)*

#### SC-2 · `generate_recommendations` includes score in result (AC-2, AC-3)
**Level:** Unit (with in-memory SQLite)  
**Given** a DB seeded with artists + top tracks + scores  
**When** `generate_recommendations` is called  
**Then** each result tuple has a `f64` score > 0.0  
**And** the tuple length is 3 *(compile-time, but verified by destructuring)*  

> Note: SC-2 requires real SQLite fixtures. Deferred to EP-12 test scaffolding.
> Promoted to Integration smoke for this iteration.

#### SC-3 · `OperationsState` `preview_results` field defaults to `None` (AC-6)
**Level:** Unit  
**Given** `OperationsState::default()`  
**Then** `preview_results` is `None`  

#### SC-4 · Existing `StateReporter` tests pass unchanged (regression, AC-8)
**Level:** Unit  
`stage_resets_current_and_total`, `tick_updates_progress`,
`reporter_is_noop_when_active_is_none` — must all pass.

### System-integration / out-of-process

No out-of-process scenarios for this epic. AC-1/AC-3/AC-5/AC-6 are covered by
Integration smoke (UI inspection + code review).

## AC coverage

| AC | Scenario(s) |
|----|-------------|
| AC-1 | Integration smoke (button present + disabled when busy) |
| AC-2 | SC-2 (deferred to EP-12) + Integration smoke |
| AC-3 | SC-2 (deferred) + Integration smoke |
| AC-4 | Integration smoke (empty DB path) |
| AC-5 | Integration smoke (re-click refreshes list) |
| AC-6 | SC-3 + Integration smoke (Discard clears panel) |
| AC-7 | `cargo build` (compile check on poll.rs call sites) |
| AC-8 | `cargo build` warning count; SC-1, SC-4 |
