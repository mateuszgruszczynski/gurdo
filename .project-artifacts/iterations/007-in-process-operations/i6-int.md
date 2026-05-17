# Iteration 7 Integration — In-process operations + progress (EP-7)

## Build

`cargo build --release` → 53 warnings (baseline), 0 errors. ✓

## Env prep

No new environment variables needed. Token path used by Spotify Login reads from existing
`config.token_path()` (defaults to `~/.gurdo/spotify_token.json`). Config unchanged.

## App start

App starts normally; playback polling continues in the same tokio runtime via `tokio::join!`.
Settings window opens via the ⚙ button as before.

## Smoke — Data section

Manual test on host (binary: `target/release/gurdo ui`):

- Settings window → Data section shows four buttons: **Sync Last.fm**, **Expand similar artists**,
  **Fetch top tracks**, **Recalculate scores**.
- Clicking any button while idle starts the operation; label `<kind>: <stage>` appears;
  progress `N/M` updates live; all four buttons disabled during run.
- On completion, last-result line shows `✓ <summary>` (green) or `✗ <error>` (red).
- Playback polling (1-second repaint) unaffected throughout.

## Smoke — Spotify section

- Login button present; disabled while any op is running.
- Status line shows "Not connected" when token file absent; "Connected" when present.
- Clicking Login runs `run_oauth_flow` in-process (opens browser); on completion status
  updates to "Connected" on next Settings viewport open.

## Verification roll-up

All non-quarantined S-0x and S-07 pass. No quarantined items.

## AC pass/fail

| AC | Result | Evidence |
|---|---|---|
| AC-1 | PASS | Data buttons trigger live-updating progress in Settings |
| AC-2 | PASS | `add_enabled_ui(!busy)` wraps all 5 buttons |
| AC-3 | PASS | `last_result` shown after completion |
| AC-4 | PASS | Ops errors surface in Settings only; no PlayerState.error writes from ops |
| AC-5 | PASS | Login button runs oauth flow; Connected status reflects token on disk |
| AC-6 | PASS | Playback polling continues; `tokio::join!` runs both tasks concurrently |
| AC-7 | PASS | 53 warnings — no new warnings introduced |

## Integration issues

None.

## Demo

Settings → Data section now fully functional with in-process operations and live progress.
Spotify Login button replaces old spawn_cli approach.
