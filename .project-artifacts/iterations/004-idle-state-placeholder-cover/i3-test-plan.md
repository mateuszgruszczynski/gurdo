# Iteration 4 Test Plan — Idle-state placeholder cover (EP-5)

*Epic: EP-5 · Phase: Test Plan · Date: 2026-05-12*

---

## Scenarios

### In-process (Unit / Component)

| # | Scenario | Level | AC(s) | Owner |
|---|---|---|---|---|
| S-1 | `decode_image` succeeds on `PLACEHOLDER_COVER` bytes — returns a 400×400 `ColorImage` without panic | Unit | AC-2 | Development |
| S-2 | `cargo build` produces zero new warnings beyond the 53-warning baseline | Component | AC-7 | Development |

### E2E / UI (Integration phase smoke)

| # | Scenario | Level | AC(s) | Owner |
|---|---|---|---|---|
| S-3 | Launch app with Spotify idle (no active playback) → placeholder image visible in 400×400 slot; background is static config colour, not blurred | E2E/UI | AC-1, AC-2, AC-3, AC-5 | Integration |
| S-4 | Start playing a track → cover art replaces placeholder seamlessly; no layout jump | E2E/UI | AC-4 | Integration |
| S-5 | Stop playback (or lose active device) → placeholder reappears; no layout jump | E2E/UI | AC-3, AC-4 | Integration |

---

## Regression scenarios (areas this epic touches)

| # | Scenario | AC(s) | Owner |
|---|---|---|---|
| R-1 | Real album art still renders at 400×400 with rounding(10.0) — no change from EP-4 behaviour | AC-4 | Integration |
| R-2 | Blurred background still updates on track change — BackgroundPainter unaffected | — | Integration |

---

## AC coverage

| AC | Covered by |
|---|---|
| AC-1 | S-3 (field present) |
| AC-2 | S-1, S-3 |
| AC-3 | S-3, S-5 |
| AC-4 | S-4, R-1 |
| AC-5 | S-3 |
| AC-6 | S-2 (no dead_code warning) |
| AC-7 | S-2 |

---

## Notes

- S-1 is an in-process unit assertion inside `Development`; no test file added (size-S epic, no test scaffolding until EP-12).
- S-3–S-5 and R-1–R-2 are manual smoke steps in Integration (headless Spotify session on host).
- No out-of-process stubs needed — this epic touches only local UI rendering code.
