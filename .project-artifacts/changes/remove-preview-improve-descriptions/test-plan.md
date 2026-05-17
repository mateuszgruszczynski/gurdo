# Test Plan — Remove recommendation preview + improve settings descriptions

## Scenarios

### S-1 Preview button absent (Component / UI)
**Given** the compiled binary
**When** `OperationCommand` variants are inspected
**Then** no `Preview` variant exists — verified by `grep -r "Preview" src/ui/state.rs` returning no match

Covers: AC-3

---

### S-2 `preview_results` field absent (Component / UI)
**Given** the compiled binary
**When** `OperationsState` struct definition is inspected
**Then** no `preview_results` field exists

Covers: AC-2

---

### S-3 `cargo build` green (Component / CLI)
**Given** all changes applied
**When** `cargo build` is run
**Then** exit 0, ≤ 2 warnings (both pre-existing)

Covers: AC-5

---

### S-4 `cargo test` green (Component / CLI)
**Given** all changes applied
**When** `cargo test` is run
**Then** 16/16 pass

Covers: AC-6

---

### S-5 All 17 knob descriptions updated (Component / UI — code inspection)
**Given** `src/ui/settings.rs`
**When** each `knob_*` call is inspected
**Then** no description contains the words "exponent", "multiplier", "fraction", "weight", "sampling", "modifier", or "pool"

Covers: AC-4

---

## Regression scenarios

### S-6 Discard handler compiles without preview_results reset (Component / CLI)
**Given** `preview_results` removed from `OperationsState`
**When** `cargo build` is run
**Then** no reference to `preview_results` in `settings.rs` Discard block — no compile error

Covers: AC-5 (regression: Discard handler used to clear `preview_results`)

---

## Level / type assignments

| Scenario | Level | Type | Phase |
|----------|-------|------|-------|
| S-1 | Component | UI (code inspection) | Development |
| S-2 | Component | UI (code inspection) | Development |
| S-3 | Component | CLI | Development |
| S-4 | Component | CLI | Development |
| S-5 | Component | UI (code inspection) | Development |
| S-6 | Component | CLI | Development |

All scenarios are in-process (code inspection + build/test). No out-of-process or E2E scenarios — the change is UI surface removal and text replacement with no external interface changes.

## AC coverage

| AC | Scenario(s) |
|----|-------------|
| AC-1 | S-1 (button removed — implicit from Preview command removal), S-3 |
| AC-2 | S-2, S-3 |
| AC-3 | S-1, S-3 |
| AC-4 | S-5 |
| AC-5 | S-3, S-6 |
| AC-6 | S-4 |
