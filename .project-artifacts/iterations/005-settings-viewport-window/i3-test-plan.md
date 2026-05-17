# Iteration 5 Test Plan — Settings viewport window (EP-6)

*Epic: EP-6 · Phase: Test Plan · Date: 2026-05-12*

---

## Scenarios

### In-process (Unit / Component)

| # | Scenario | Level | AC(s) | Owner |
|---|---|---|---|---|
| S-1 | `cargo build` with new config fields and removed SettingsDraft — zero new warnings | Component | AC-8 | Development |
| S-2 | Old `config.toml` without `player_window_size`/`settings_window_size` deserialises without error (serde defaults kick in) | Unit | AC-1 | Development |

### E2E / UI (Integration phase smoke)

| # | Scenario | Level | AC(s) | Owner |
|---|---|---|---|---|
| S-3 | Launch app → player window opens at the configured size (not hardcoded 440×660) | E2E/UI | AC-2 | Integration |
| S-4 | Click `⚙` → a new OS window opens (separate from player) with title "Gurdo — Settings" | E2E/UI | AC-3 | Integration |
| S-5 | Settings window appears approximately centred over the player | E2E/UI | AC-4 | Integration |
| S-6 | Drag settings window to a corner; it stays — does not snap back | E2E/UI | AC-4 | Integration |
| S-7 | Player controls (play/pause, seek, like/dislike) respond while settings window is open | E2E/UI | AC-3 | Integration |
| S-8 | Settings window shows 7 distinct sections with placeholder labels | E2E/UI | AC-5 | Integration |
| S-9 | Click OS close button on settings window → window closes; player unaffected | E2E/UI | AC-6 | Integration |
| S-10 | Click `⚙` again after closing → settings window reopens | E2E/UI | AC-6 | Integration |

### Regression scenarios

| # | Scenario | Owner |
|---|---|---|
| R-1 | Blurred cover background still paints correctly with two viewports active | Integration |
| R-2 | Placeholder cover still shows when no track is playing | Integration |
| R-3 | Error modal still appears on top of player content when triggered | Integration |

---

## AC coverage

| AC | Covered by |
|---|---|
| AC-1 | S-2 |
| AC-2 | S-3 |
| AC-3 | S-4, S-7 |
| AC-4 | S-5, S-6 |
| AC-5 | S-8 |
| AC-6 | S-9, S-10 |
| AC-7 | S-1 (compile-time: SettingsDraft gone, no dead_code warning) |
| AC-8 | S-1 |

---

## Notes

- S-2 is verified in Development by running `toml::from_str` on a minimal config string missing the new fields; confirmed by build success + absence of serde errors in smoke run.
- S-3–S-10 and R-1–R-3 are manual smoke steps in Integration.
- No external stubs required — this epic is pure UI.
