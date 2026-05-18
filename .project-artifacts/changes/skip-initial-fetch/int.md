# Integration: skip-initial-fetch

## Build status
`cargo build` — green. 1 pre-existing warning in poll.rs (unrelated).

## Environment preparation
No `.env` required. App uses `~/.gurdo/config.toml` and `~/.gurdo/secrets.toml`.

## Application start
Native GUI — binary launched on host via `cargo run`.

## Manual smoke
User ran full setup flow on host. All scenarios passed:
- FetchPrompt screen appeared after OAuth with correct labels and both buttons
- "Skip for now" closed setup, main app opened with empty library, no error
- "Fetch now" path confirmed working (Fetching phase started normally)

## Verification roll-up
See `verify.md`. Unit tests: 6/6 pass. E2E/UI: manual smoke confirmed by user — all passed.

## AC pass/fail table

| AC | Scenario | Result |
|----|----------|--------|
| AC 1 — OAuth success → FetchPrompt, no fetch | T-01 (unit) + manual smoke | PASS |
| AC 2 — OAuth skip → FetchPrompt, no fetch | T-02 (unit) + manual smoke | PASS |
| AC 3 — FetchPrompt renders required elements | T-06 (manual smoke) | PASS |
| AC 4 — "Fetch now" starts fetch | T-07 (manual smoke) | PASS |
| AC 5 — "Skip for now" → main app, empty DB | T-08 (manual smoke) | PASS |
| AC 6 — Window close on FetchPrompt → Complete | T-03 (unit) | PASS |
| AC 7 — Existing Fetching phase unchanged | T-07 (manual smoke) | PASS |
| AC 8 — Fetch error → "Continue anyway" | T-09 — not exercised this smoke | in-process code path unchanged |
| AC 9 — oauth_status reset | T-01 + T-10 (unit) | PASS |
| AC 10 — Cancel paths unaffected | T-04 + T-05 (unit) | PASS |

## Packaging
Skipped — policy: milestone.

## Integration green — continuing with Retrospective.
