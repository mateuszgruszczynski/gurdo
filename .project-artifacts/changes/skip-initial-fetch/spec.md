# Spec: Introduce FetchPrompt Phase in First-Run Setup Flow

## Title
Add explicit user consent step before initial data fetch in first-run setup

## Description
Currently, after the OAuth step in the first-run setup flow, the app immediately and automatically starts fetching the user's Last.fm and Spotify data. There is no way for a user to defer this operation.

This change inserts a new `Phase::FetchPrompt` between `Phase::OAuth` and `Phase::Fetching`. The screen presents a brief explanation and two explicit choices: fetch immediately, or skip and open the app with an empty database, with a note that the fetch can be triggered later from Settings. No data fetch is initiated until the user actively opts in.

## User Story
As a new Gurdo user completing first-run setup, I want to be asked whether to fetch my music data immediately or defer it, so that I can open the app right away without waiting for a potentially lengthy data import.

## Acceptance Criteria

1. **After OAuth success — no automatic fetch.**
   Given the user is on `Phase::OAuth` and the Spotify OAuth flow completes successfully (`OAuthStatus::Success`), when the UI re-renders, `start_fetch()` must NOT be called and `Phase::Fetching` must NOT be entered. The app must transition to `Phase::FetchPrompt` instead. Verified by confirming no fetch thread is spawned and the FetchPrompt screen is visible.

2. **After "Skip for now" on OAuth — no automatic fetch.**
   Given the user is on `Phase::OAuth` and clicks the "Skip for now" button, when the click is processed, `start_fetch()` must NOT be called. The app must transition to `Phase::FetchPrompt`. Verified by confirming the FetchPrompt screen is shown and no fetch activity indicator appears.

3. **FetchPrompt screen contains required UI elements.**
   Given the app is in `Phase::FetchPrompt`, when the screen renders, it must display: (a) a label including the text "fetch" (case-insensitive), (b) a note that the fetch can be done later from Settings, (c) a "Fetch now" button, and (d) a "Skip for now" button.

4. **"Fetch now" button starts the fetch.**
   Given the app is in `Phase::FetchPrompt`, when the user clicks "Fetch now", the app must transition to `Phase::Fetching`, `start_fetch()` must be called exactly once, and the step counter ("Step 1/4") must appear in the UI.

5. **"Skip for now" on FetchPrompt closes the window with Complete outcome.**
   Given the app is in `Phase::FetchPrompt`, when the user clicks "Skip for now", `SetupOutcome::Complete` must be written and `ViewportCommand::Close` sent. The setup window must close and the main app must open with an empty track list and no error dialog.

6. **Closing the window on FetchPrompt is treated as Complete.**
   Given the app is in `Phase::FetchPrompt`, when the user closes the window via the OS close button, the outcome must be `SetupOutcome::Complete` — not a cancellation. The main app must open.

7. **Existing Fetching phase behaviour is unchanged.**
   Given the user clicked "Fetch now" and `Phase::Fetching` is active, all four steps complete and `SetupOutcome::Complete` is set — identical to current behaviour.

8. **Fetch error handling in Fetching phase is unchanged.**
   Given the user clicked "Fetch now" and a fetch step fails, the "Continue anyway" button must appear and close setup with `SetupOutcome::Complete`.

9. **oauth_status reset on transition to FetchPrompt.**
   When transitioning to `Phase::FetchPrompt`, `self.oauth_status` must be set to `OAuthStatus::Idle` to prevent the `update()` loop from re-triggering the `OAuthStatus::Success` branch on subsequent frames.

10. **No regression on Phase::Fields or Phase::OAuth cancel paths.**
    Closing the window during `Phase::Fields` produces `CancelledPhase1`; closing during `Phase::OAuth` produces `CancelledOAuth`. Both still return an error to the caller.

## Out of Scope
- Implementing a manual "fetch" trigger in Settings (existing feature).
- Persisting a "fetch deferred" flag to disk.
- Allowing cancellation of a fetch once started in `Phase::Fetching`.
- Changes to the four-step fetch sequence itself.

## Edge Cases
1. OAuth failure then retry → must still go to FetchPrompt, not directly to Fetching (covered by AC 9).
2. Rapid double-click on "Fetch now" → `start_fetch()` called only once (existing guard in `start_fetch()` handles this).
3. Window close during `OAuthStatus::Pending` → still produces `CancelledOAuth` (FetchPrompt not yet entered).

## UI/UX Notes
Layout follows the existing OAuth screen conventions (`show_oauth`):

```
[space 20]
Heading: "Set up your music library"
[space 8]
Label: "Gurdo can fetch your Last.fm listening history and Spotify library now.
        This takes a few minutes on the first run."
[space 4]
Weak label: "You can also do this later from Settings."
[space 16]
[vertical_centered]
  [ Fetch now ]
  [space 8]
  [ Skip for now ]
```

No progress indicators on this screen — those belong to `Phase::Fetching`.

## Security / Backwards Compatibility
- `SetupOutcome` variants unchanged; caller sees `Ok(())` for both fetch and skip paths.
- No new file I/O or threads on the skip path.
- `Phase::FetchPrompt` is a private enum variant — no public API impact.
- Treating window-close on FetchPrompt as `Complete` is intentional: credentials are already saved.
