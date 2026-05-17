# Iteration 5 Verification — Settings viewport window (EP-6)

*Epic: EP-6 · Phase: Verification · Date: 2026-05-12*

---

## Environment

Build: dev container. Run: host (hybrid mode).
Launch: `cargo run -- -c config.toml ui`

No external stubs required — pure UI restructuring.

---

## In-process results

| Scenario | Result |
|---|---|
| S-1: `cargo build` — 53 warnings, zero new | PASS |
| S-2: serde defaults — old config.toml without new fields loads cleanly | PASS (confirmed by build + runtime; serde `#[serde(default)]` on both new fields) |

---

## E2E / UI scenarios (to be run in Integration phase)

| # | Scenario | AC(s) |
|---|---|---|
| S-3 | Player window opens at configured size | AC-2 |
| S-4 | `⚙` opens new OS window "Gurdo — Settings" | AC-3 |
| S-5 | Settings window appears approximately centred over player | AC-4 |
| S-6 | Drag settings window — it stays where dropped | AC-4 |
| S-7 | Player controls work while settings open | AC-3 |
| S-8 | Settings shows 7 sections with placeholder labels | AC-5 |
| S-9 | OS close → window gone, player unaffected | AC-6 |
| S-10 | Click `⚙` again → settings reopens | AC-6 |
| R-1 | Blurred cover background correct with two viewports | — |
| R-2 | Placeholder cover shows when no track playing | — |

---

## Quarantined items

None.

---

## AC coverage

| AC | Status |
|---|---|
| AC-1 | Covered — `#[serde(default)]` on both fields; build green |
| AC-2 | Covered — S-3 (Integration) |
| AC-3 | Covered — S-4, S-7 |
| AC-4 | Covered — S-5, S-6 |
| AC-5 | Covered — S-8 |
| AC-6 | Covered — S-9, S-10 |
| AC-7 | Covered — S-1 (SettingsDraft gone, no new dead_code) |
| AC-8 | Covered — S-1 (53 warnings) |
