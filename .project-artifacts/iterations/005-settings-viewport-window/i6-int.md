# Iteration 5 Integration — Settings viewport window (EP-6)

*Epic: EP-6 · Phase: Integration · Date: 2026-05-12*

---

## Build status

`cargo build` — exit 0. 53 pre-existing warnings. Zero new warnings.

---

## Env prep

No new environment variables or credentials required.
Launch: `cargo run -- -c config.toml ui`

---

## App start result

App launched successfully on host.

---

## Smoke outcome

| Step | Result |
|---|---|
| Player window opens at configured size | PASS |
| Click `⚙` → new OS window "Gurdo — Settings" opens (not a popup overlay) | PASS |
| Settings window approximately centred over player | PASS |
| Drag settings window → stays where dropped, does not snap back | PASS |
| Player controls (play/pause, like/dislike, seek) respond while settings open | PASS |
| Settings shows 7 sections with italic gray placeholder labels | PASS |
| OS close button → window closes immediately | PASS |
| In-window Close button → window closes immediately (after `request_repaint_of(ROOT)` fix) | PASS |
| Click `⚙` again → settings reopens | PASS |
| Blurred cover background correct with two viewports active | PASS |
| Placeholder cover shows when no track playing | PASS |

---

## Integration-phase issues fixed

1. **1s delay on in-window Close button** — after clicking Close, the settings window
   persisted for ~1 second. Root cause: `settings_open.store(false)` was set but the
   player viewport only repaints every 1s, so `show_viewport_deferred` continued to be
   called for that window. Fix: added `ctx.request_repaint_of(egui::ViewportId::ROOT)`
   immediately after the store call in `settings::render`. Committed: 524e1bd.

2. **EP-17 added to backlog** — during smoke test, Spotify API was experiencing downtime.
   Every 5s poll failure raised a blocking error modal. A new P1 epic (EP-17: Spotify API
   error suppression + status indicator) was added to the backlog to address modal flood
   during outages.

---

## Verification roll-up

All 8 ACs covered and passing. No quarantined items.

---

## Demo

User ran the app, confirmed settings opens as a proper OS window, Close button works
instantly, all 7 placeholder sections visible.
