# Iteration 4 Integration — Idle-state placeholder cover (EP-5)

*Epic: EP-5 · Phase: Integration · Date: 2026-05-12*

---

## Build status

`cargo build` — exit 0. 53 pre-existing warnings. Zero new warnings.

---

## Env prep

No new environment variables or credentials required.
Launch: `cargo run -- -c config.toml ui`

---

## App start result

App launched successfully on host. Spotify session available for testing.

---

## Smoke outcome

| Step | Result |
|---|---|
| Launch with no active Spotify playback | OK |
| Placeholder visible in 400×400 slot (not empty space) | PASS |
| Background is static near-black config colour (no blur artifact) | PASS |
| Start playing a track → real cover replaces placeholder | PASS |
| No layout jump on cover → placeholder or placeholder → cover transition | PASS |
| Stop playback → placeholder reappears | PASS |
| Real cover renders at 400×400 with rounding; blur updates on track change | PASS |

User confirmed: "works ok, approved."

---

## Verification roll-up

All 7 AC criteria covered and passing. No quarantined items.

---

## Integration-phase issues

None.

---

## Demo

User ran the app and confirmed the placeholder is visible during idle state and transitions seamlessly to/from real cover art.
