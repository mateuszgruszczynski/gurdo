# Verification: skip-initial-fetch

## Test environment
- Binary: `cargo build` → `target/debug/gurdo`
- Platform: Linux (no devcontainer)
- No external stubs needed — this change is pure UI state machine within the setup window

## External-service stubs
None required. The FetchPrompt phase contains no network calls. The Fetching phase (unchanged) would need Last.fm/Spotify, but that is covered by existing behaviour.

## E2E tests — Type: UI

egui is an immediate-mode Rust GUI framework with no standard automated UI testing toolchain (no accessibility tree, no Playwright/Cypress equivalent for native desktop). Automated E2E for this app is not feasible without significant custom instrumentation not in scope.

**All E2E scenarios are manual.** Steps for a human tester:

### T-06 — FetchPrompt screen renders required elements
```
Given  fresh install (delete ~/.gurdo/secrets.toml), run gurdo
When   enter Last.fm username → Continue → complete OAuth phase
Then   screen shows heading containing "fetch"
And    note mentioning "Settings" is visible
And    "Fetch now" button is visible
And    "Skip for now" button is visible
And    no step counter or progress bar is visible
```

### T-07 — "Fetch now" starts the fetch and completes setup
```
Given  the FetchPrompt screen is shown (as above)
When   click "Fetch now"
Then   screen transitions to "Fetching your music data" heading
And    "Step 1/4" label appears
And    all four steps complete
And    setup window closes
And    main app window opens
```

### T-08 — "Skip for now" opens main app with empty library
```
Given  the FetchPrompt screen is shown
When   click "Skip for now"
Then   setup window closes immediately (no fetch activity)
And    main app window opens
And    library is empty — no tracks listed
And    no error dialog shown
```

### T-09 — Fetch error still shows "Continue anyway" (regression)
```
Given  FetchPrompt shown; network unavailable or invalid Last.fm credentials
When   click "Fetch now" and the first step fails
Then   "✗ Step 1/4 ... failed: ..." error label appears
And    "Continue anyway" button is visible
And    clicking it closes setup and opens main app
```

## Run results

| Scenario | Type | Status | Notes |
|----------|------|--------|-------|
| T-01 (unit) | Unit | PASS (automated) | cargo test |
| T-02 (unit) | Unit | PASS (automated) | cargo test |
| T-03 (unit) | Unit | PASS (automated) | cargo test |
| T-04 (unit) | Unit | PASS (automated) | cargo test |
| T-05 (unit) | Unit | PASS (automated) | cargo test |
| T-10 (unit) | Unit | PASS (automated) | cargo test |
| T-06 (E2E/UI) | Manual | PENDING human run | egui has no automated UI test toolchain |
| T-07 (E2E/UI) | Manual | PENDING human run | requires Last.fm/Spotify credentials |
| T-08 (E2E/UI) | Manual | PENDING human run | |
| T-09 (E2E/UI) | Manual | PENDING human run | requires network failure simulation |

Build: `cargo build` → **green** (1 pre-existing warning in poll.rs, unrelated).

## Quarantined tests
None quarantined. T-06 through T-09 are documented manual scenarios rather than automated tests — not quarantined, just not automatable with current tooling.

## AC coverage table

| AC | Verification scenario | Coverage |
|----|----------------------|----------|
| AC 1 — OAuth success → FetchPrompt, no fetch | T-01 (unit) + T-06/T-07 (manual E2E) | in-process + manual |
| AC 2 — OAuth skip → FetchPrompt, no fetch | T-02 (unit) | in-process only — no out-of-process observable beyond phase state |
| AC 3 — FetchPrompt renders required elements | T-06 (manual E2E) | manual E2E |
| AC 4 — "Fetch now" starts fetch | T-07 (manual E2E) | manual E2E |
| AC 5 — "Skip for now" → main app, empty DB | T-08 (manual E2E) | manual E2E |
| AC 6 — Window close on FetchPrompt → Complete | T-03 (unit) | in-process only — close_requested handler is pure logic |
| AC 7 — Existing Fetching phase unchanged | T-07 (manual E2E) | manual E2E (regression) |
| AC 8 — Fetch error → "Continue anyway" | T-09 (manual E2E) | manual E2E (regression) |
| AC 9 — oauth_status reset | T-01 + T-10 (unit) | in-process only — internal state, no out-of-process observable |
| AC 10 — Cancel paths unaffected | T-04 + T-05 (unit) | in-process only — handle_close() is pure logic |
