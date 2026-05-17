# Iteration 6 Test Plan — Spotify API error suppression + status indicator (EP-17)

*Epic: EP-17 · Phase: Test Plan · Date: 2026-05-12*

---

## Scenarios

### In-process

| # | Scenario | Level | AC(s) | Owner |
|---|---|---|---|---|
| S-1 | `cargo build` — 53 warnings, zero new | Component | AC-6 | Development |

### E2E / UI (Integration phase smoke)

| # | Scenario | Level | AC(s) | Owner |
|---|---|---|---|---|
| S-2 | During API downtime: first poll failure → modal appears with "OK" and "Snooze 10 min" buttons | E2E/UI | AC-4 | Integration |
| S-3 | Click "Snooze 10 min" → modal dismisses; no new modal on subsequent poll failures | E2E/UI | AC-1 | Integration |
| S-4 | While snoozed: amber `⚠ Spotify API unavailable` label visible below artist name | E2E/UI | AC-2 | Integration |
| S-5 | API recovers (successful poll) → label disappears without user action | E2E/UI | AC-3 | Integration |
| S-6 | Click "OK" → modal dismissed; next poll failure raises modal again | E2E/UI | AC-4 | Integration |
| S-7 | User clicks Play/Pause during snooze and it fails → modal appears (explicit action not suppressed) | E2E/UI | AC-5 | Integration |

### Regression

| # | Scenario | Owner |
|---|---|---|
| R-1 | Normal playback polling (API up) — no label shown, no modal | Integration |
| R-2 | Like/dislike/queue still work and errors surface normally | Integration |

---

## AC coverage

| AC | Covered by |
|---|---|
| AC-1 | S-3 |
| AC-2 | S-4 |
| AC-3 | S-5 |
| AC-4 | S-2, S-6 |
| AC-5 | S-7 |
| AC-6 | S-1 |

---

## Notes

- S-5 and S-7 may be hard to trigger exactly during smoke if API is healthy; describe
  expected behaviour and note as "verified by code review" if API is up during Integration.
- S-3 can be partially validated by checking the snooze timestamp is set correctly even
  without a live downtime event.
