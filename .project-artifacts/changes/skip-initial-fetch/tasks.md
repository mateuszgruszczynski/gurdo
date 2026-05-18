# Tasks: skip-initial-fetch

## DEV-1 — Extend Phase enum + close-handler
**Role:** DEV
**Description:** Add `FetchPrompt` variant to the `Phase` enum in `setup.rs`. Update the `close_requested` handler in `update()` so that closing on `Phase::FetchPrompt` sets `SetupOutcome::Complete` (not a cancellation).
**Dependencies:** none
**Done when:** `Phase::FetchPrompt` compiles; close-on-FetchPrompt test passes (AC 6, AC 10).

## DEV-2 — Replace auto-fetch trigger with FetchPrompt transition
**Role:** DEV
**Description:** In `update()`, replace the `OAuthStatus::Success` auto-call to `start_fetch()` (line ~115) with a transition to `Phase::FetchPrompt` + reset `self.oauth_status = OAuthStatus::Idle`. Do the same for the "Skip for now" OAuth button (line ~196): transition to `Phase::FetchPrompt` instead of calling `start_fetch()`.
**Dependencies:** DEV-1
**Done when:** Neither OAuth success nor OAuth skip triggers a fetch thread; FetchPrompt phase is entered in both cases (AC 1, AC 2, AC 9).

## DEV-3 — Implement show_fetch_prompt() UI
**Role:** DEV
**Description:** Add a `show_fetch_prompt()` method to `SetupApp`. Layout (per spec): heading, body copy, weak "do it later from Settings" note, then vertical-centered "Fetch now" and "Skip for now" buttons. "Fetch now" calls `self.start_fetch(ctx)` and sets `self.phase = Phase::Fetching`. "Skip for now" sets `SetupOutcome::Complete` and sends `ViewportCommand::Close`.
**Dependencies:** DEV-2
**Done when:** Both buttons render and behave correctly; no progress indicators visible on this screen (AC 3, AC 4, AC 5).

## DEV-4 — In-process unit tests for phase transitions
**Role:** DEV
**Description:** Add unit tests (in `setup.rs` or a sibling test module) covering: (a) OAuth success → phase becomes FetchPrompt + oauth_status reset to Idle; (b) OAuth skip → phase becomes FetchPrompt; (c) FetchPrompt close → outcome is Complete. Mock or stub the egui context as needed.
**Dependencies:** DEV-3
**Done when:** `cargo test` passes with the new tests green.

## QA-1 — E2E UI: skip-fetch path
**Role:** QA
**Description:** Automated or manual E2E scenario: complete setup through OAuth, arrive at FetchPrompt, click "Skip for now" → main app window opens, track list is empty, no error dialog shown.
**Dependencies:** DEV-3
**Done when:** Scenario passes; AC 5 verified.

## QA-2 — E2E UI: fetch-now path (regression)
**Role:** QA
**Description:** Automated or manual E2E scenario: complete setup through OAuth, arrive at FetchPrompt, click "Fetch now" → Fetching phase starts, all 4 steps complete, main app opens with data. Verifies no regression on the happy path (AC 7).
**Dependencies:** DEV-3
**Done when:** Scenario passes; AC 7 verified.

## DEV-5 — CHANGELOG entry
**Role:** DEV
**Description:** Add an entry to `CHANGELOG.md` under the Unreleased section: "After setup, users are now prompted whether to fetch data immediately or defer it to later from Settings."
**Dependencies:** DEV-3
**Done when:** Entry present in CHANGELOG.md.
