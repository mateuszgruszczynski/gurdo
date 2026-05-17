# Iteration 2 Integration — Embedded assets + CJK font fix (EP-3)

*Epic: EP-3 · Phase: Integration · Date: 2026-05-12*

---

## Build status

`cargo build --release` — exit 0. Binary: `target/release/gurdo`, 41 MB (+17 MB over pre-EP-3 baseline of 24 MB). 53 pre-existing warnings (unrelated modules). Zero new warnings.

---

## Env prep

No new environment variables or credentials required for EP-3. Config file path passed as `--config config.toml` (before the subcommand).

Invocation: `cargo run -- -c config.toml ui`

---

## App start result

App launched successfully on host. Spotify session active. Player window opened at 440×660.

---

## Smoke outcome

| Step | Result |
|---|---|
| Launch with `cargo run -- -c config.toml ui` | OK |
| Japanese track title (e.g. 夜に駆ける) — kanji/kana render | PASS |
| Korean track title (e.g. 강남스타일) — Hangul renders | PASS |
| Cyrillic track title — no regression | PASS |
| Latin/ASCII track title — no regression | PASS |

---

## Verification roll-up

All 12 scenarios passed. No quarantined items.

| AC | Result |
|---|---|
| AC-1 | PASS |
| AC-2 | PASS |
| AC-3 | PASS |
| AC-4 | PASS |
| AC-5 | PASS |
| AC-6 | PASS |
| AC-7 | PASS |
| AC-8 | PASS |
| AC-9 | PASS |
| AC-10 | PASS |

---

## Integration-phase issues

None.

---

## Demo

The user ran the app on the host and confirmed CJK glyph rendering directly. No demo script required — the fix is directly observable in the player window.
