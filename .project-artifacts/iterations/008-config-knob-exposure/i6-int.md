# Iteration 8 Integration — Full config-knob exposure (EP-8)

## Build

`cargo build --release` → 53 warnings (baseline), 0 errors. ✓

## Smoke — knob sections

Manual test on host (`target/release/gurdo ui` → ⚙ → Settings):

- **Recommendations section**: 3 DragValue knobs (count, artist score exp, track rank exp);
  each has ↺ reset button and tooltip.
- **Engine section**: 8 DragValue knobs; Reset restores to Default values.
- **Artist Scoring section**: 3 knobs including `min_playcount_threshold` (u64).
- **Sync section**: 3 knobs (loved_tracks_limit, seed_artists_limit, seed_tracks_limit).

## Smoke — draft / Save / Discard

- With no changes: Save button absent.
- Changing any DragValue: `• Save` + `Discard changes` buttons appear below.
- Discard: values snap back to saved state; buttons disappear.
- Save: writes to `config.toml`; `shared_config` updated in-process; buttons disappear.
  Next operation that reads the changed field uses the new value.

## Smoke — Appearance section

Shows: Data directory, Database path, Config file path, Token file path, Last.fm username,
Spotify client ID (first 8 chars + …). All read-only.

## Smoke — Data + Spotify (regression)

Operation buttons still functional; progress/result display unchanged from EP-7.
Login button still present and working.

## AC pass/fail

| AC | Result |
|---|---|
| AC-1 | PASS — all 16 knob fields editable |
| AC-2 | PASS — Save writes to config.toml and updates shared_config |
| AC-3 | PASS — Discard returns all values without disk write |
| AC-4 | PASS — ↺ resets individual field to compiled default |
| AC-5 | PASS — Save button absent when nothing changed |
| AC-6 | PASS — Appearance section shows paths and identifiers |
| AC-7 | PASS — TRACKS_PER_ARTIST deleted; fetch_artist_tracks reads config |
| AC-8 | PASS — 53 warnings, no regressions |

## Integration issues

None.
