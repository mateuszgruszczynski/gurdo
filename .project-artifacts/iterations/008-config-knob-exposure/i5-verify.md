# Iteration 8 Verification — Full config-knob exposure (EP-8)

## In-process tests

| Scenario | Level | Result | AC |
|---|---|---|---|
| S-04: TRACKS_PER_ARTIST removed (`grep` clean) | compile check | PASS | AC-7 |
| S-05: warning count | `cargo build` | 53 warnings | AC-8 |
| EP-7 unit tests (3/3) | Unit | PASS | regression |

## Out-of-process

S-01 – S-03 (draft None/Some lifecycle, Save updates shared_config) — no out-of-process test
infra for egui closures; verified in Integration smoke.

AC-1,4,5,6 — manual UI smoke in Integration.

## AC coverage

| AC | Status | Evidence |
|---|---|---|
| AC-1 | Pending Integration | 16 knob fields rendered in 4 sections |
| AC-2 | Pending Integration | Save path implemented; manual smoke |
| AC-3 | Pending Integration | Discard path clears draft |
| AC-4 | Pending Integration | ↺ button on each knob |
| AC-5 | Pending Integration | Save enabled only when `dirty || any_changed` |
| AC-6 | Pending Integration | Appearance section renders read-only paths |
| AC-7 | PASS | grep returns empty; `cargo build` clean |
| AC-8 | PASS | 53 warnings — baseline maintained |

## Quarantined items

None.
