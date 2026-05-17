# Iteration 6 Integration — Spotify API error suppression + status indicator (EP-17)

*Epic: EP-17 · Phase: Integration · Date: 2026-05-12*

---

## Build status

`cargo build` — exit 0. 53 warnings. Zero new.

---

## Smoke outcome

| Step | Result |
|---|---|
| Normal playback — no warning label, no behaviour change | PASS |
| Spotify downtime — modal shows "OK" + "Snooze 10 min" | PASS |
| Snooze → no new modals; `⚠ Spotify API unavailable` replaces time label | PASS |
| No layout shift — controls stay in place | PASS |
| API recovery → warning disappears | PASS |

User confirmed: "works fine, approved."

---

## Integration-phase issues fixed

**Layout shift** — original implementation added the warning as an extra label between artist name and controls, pushing all controls down. Fixed by moving the warning into the existing time-display label slot below the progress bar, replacing `0:00 / 0:00` while snoozed.

---

## Verification roll-up

All 6 ACs covered and passing. No quarantined items.
