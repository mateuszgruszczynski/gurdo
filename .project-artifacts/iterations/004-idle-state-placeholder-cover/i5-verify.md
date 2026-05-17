# Iteration 4 Verification — Idle-state placeholder cover (EP-5)

*Epic: EP-5 · Phase: Verification · Date: 2026-05-12*

---

## Environment

Build: dev container. Run: host (hybrid mode, same as prior iterations).
Launch: `cargo run -- -c config.toml ui`

No external stubs required — this epic is pure UI rendering.

---

## In-process results

| Scenario | Result |
|---|---|
| S-1: `decode_image(PLACEHOLDER_COVER)` — build succeeds, 53 warnings (no new) | PASS |
| S-2: `cargo build` zero new warnings | PASS |

---

## E2E / UI scenarios (to be run in Integration phase)

| # | Scenario | AC(s) |
|---|---|---|
| S-3 | Launch with no active Spotify playback → placeholder visible in slot; static bg colour | AC-1,2,3,5 |
| S-4 | Play a track → real cover replaces placeholder; no layout jump | AC-4 |
| S-5 | Stop playback → placeholder reappears; no layout jump | AC-3,4 |
| R-1 | Real album art at 400×400 + rounding(10.0) unchanged | AC-4 |
| R-2 | Blurred background still updates on track change | — |

---

## Quarantined items

None.

---

## AC coverage

| AC | Status |
|---|---|
| AC-1 | Covered — field exists, init in update() |
| AC-2 | Covered — S-1 (build) + S-3 (runtime) |
| AC-3 | Covered — S-3, S-5 |
| AC-4 | Covered — S-4, R-1 |
| AC-5 | Covered — S-3 |
| AC-6 | Covered — `#[allow(dead_code)]` removed, build clean |
| AC-7 | Covered — S-2 |

All ACs have at least one passing or Integration-scheduled scenario.
