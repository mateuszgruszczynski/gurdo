# Test Plan: skip-initial-fetch

## T-01 — OAuth success transitions to FetchPrompt
**Covers AC:** 1, 9
**Level:** Unit | **Type:** — | **Owned by:** Development

```
Given  the SetupApp is in Phase::OAuth with OAuthStatus::Success
When   the update() logic processes the OAuth status check
Then   the phase becomes Phase::FetchPrompt
And    oauth_status is reset to OAuthStatus::Idle
And    start_fetch() is not called
```
**Notes:** Test by calling the transition function directly or by inspecting state after a simulated frame with OAuthStatus::Success set.

---

## T-02 — "Skip for now" on OAuth transitions to FetchPrompt
**Covers AC:** 2
**Level:** Unit | **Type:** — | **Owned by:** Development

```
Given  the SetupApp is in Phase::OAuth
When   the "Skip for now" button action is triggered
Then   the phase becomes Phase::FetchPrompt
And    start_fetch() is not called
```

---

## T-03 — Window close on FetchPrompt produces Complete outcome
**Covers AC:** 6
**Level:** Unit | **Type:** — | **Owned by:** Development

```
Given  the SetupApp is in Phase::FetchPrompt
When   the close_requested event is handled
Then   SetupOutcome::Complete is written to the shared outcome
And    no cancellation outcome is produced
```

---

## T-04 — Window close on Phase::Fields produces CancelledPhase1 (regression)
**Covers AC:** 10
**Level:** Unit | **Type:** — | **Owned by:** Development

```
Given  the SetupApp is in Phase::Fields
When   the close_requested event is handled
Then   SetupOutcome::CancelledPhase1 is written
```

---

## T-05 — Window close on Phase::OAuth produces CancelledOAuth (regression)
**Covers AC:** 10
**Level:** Unit | **Type:** — | **Owned by:** Development

```
Given  the SetupApp is in Phase::OAuth (any OAuthStatus)
When   the close_requested event is handled
Then   SetupOutcome::CancelledOAuth is written
```

---

## T-06 — FetchPrompt screen renders required elements
**Covers AC:** 3
**Level:** E2E | **Type:** UI | **Owned by:** Verification

```
Given  a fresh install with valid Last.fm credentials entered and OAuth phase completed
When   the setup window displays the FetchPrompt screen
Then   a label containing the word "fetch" is visible
And    a note mentioning "Settings" is visible
And    a "Fetch now" button is visible
And    a "Skip for now" button is visible
And    no progress indicator or step counter is visible
```

---

## T-07 — "Fetch now" button starts the fetch and completes setup
**Covers AC:** 4, 7
**Level:** E2E | **Type:** UI | **Owned by:** Verification

```
Given  the setup window is on the FetchPrompt screen
When   the user clicks "Fetch now"
Then   the screen transitions to the fetching view showing "Step 1/4"
And    all four fetch steps complete successfully
And    the setup window closes
And    the main application window opens
```
**Notes:** Requires a real or stubbed Last.fm/Spotify connection. If network unavailable, this scenario is environment-dependent — see T-09 for error path.

---

## T-08 — "Skip for now" on FetchPrompt opens main app with empty library
**Covers AC:** 5
**Level:** E2E | **Type:** UI | **Owned by:** Verification

```
Given  the setup window is on the FetchPrompt screen
When   the user clicks "Skip for now"
Then   the setup window closes immediately
And    the main application window opens
And    the track list or library view shows no entries
And    no error dialog is displayed
```

---

## T-09 — Fetch error path still shows "Continue anyway" (regression)
**Covers AC:** 8
**Level:** E2E | **Type:** UI | **Owned by:** Verification

```
Given  the user clicked "Fetch now" and the fetch operation encounters an error
When   the error is reported in the fetching view
Then   a "Continue anyway" button appears
And    clicking it closes the setup window
And    the main application window opens
```
**Notes:** Simulate by blocking network access or pointing to an invalid Last.fm username.

---

## T-10 — oauth_status guard prevents double-trigger on re-render
**Covers AC:** 9
**Level:** Unit | **Type:** — | **Owned by:** Development

```
Given  oauth_status was set to OAuthStatus::Success and the phase transitioned to FetchPrompt
When   the update() loop runs a second frame while still in FetchPrompt
Then   start_fetch() is not called again
And    the phase remains FetchPrompt
```
**Notes:** Verifies the oauth_status reset (set to Idle on transition) is effective. Test by running the relevant update branch twice with phase == FetchPrompt.

---

## AC Coverage Table

| AC | Scenario(s) | Gap |
|----|------------|-----|
| AC 1 — OAuth success → FetchPrompt, no fetch | T-01, T-07 | — |
| AC 2 — OAuth skip → FetchPrompt, no fetch | T-02 | — |
| AC 3 — FetchPrompt renders required elements | T-06 | — |
| AC 4 — "Fetch now" starts fetch | T-07 | — |
| AC 5 — "Skip for now" → main app, empty DB | T-08 | — |
| AC 6 — Window close on FetchPrompt → Complete | T-03 | — |
| AC 7 — Existing Fetching phase unchanged | T-07 | — |
| AC 8 — Fetch error → "Continue anyway" | T-09 | — |
| AC 9 — oauth_status reset on FetchPrompt | T-01, T-10 | — |
| AC 10 — Cancel paths unaffected | T-04, T-05 | — |
