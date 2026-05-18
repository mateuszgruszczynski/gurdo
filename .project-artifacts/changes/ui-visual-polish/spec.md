# Spec: UI Visual Polish — Setup & Settings

## Title
Improve setup and settings screen visuals: centered layout, larger setup text, narrower input, lighter errors, 4-step simultaneous progress bars

## Description
A set of focused visual improvements across the setup wizard and settings screen. No logic changes — purely presentation. The most significant change is replacing the single-active-step display in fetch/sync operations with four simultaneous progress bars, one per step, so the user can see overall progress at a glance.

## User Story
As a Gurdo user, I want the setup and settings screens to feel polished and readable, so that I can understand what is happening at a glance without squinting at small text or trying to infer overall fetch progress from a single moving indicator.

## Acceptance Criteria

### Setup screen — Fields phase (username entry)

1. **Heading and label text are horizontally centred.**
   Given the setup window shows the username entry screen, when it renders, the "Welcome to Gurdo" heading and the instruction label below it must be horizontally centred in the window.

2. **Username input is narrower and centred.**
   Given the username entry screen is shown, when it renders, the text input field must have a fixed maximum width (≤ 280 px) and be centred horizontally, not stretched to the full window width.

3. **"Continue" button is centred.**
   Given the username entry screen is shown, the "Continue" button must be centred, not right-aligned.

4. **Error text uses a lighter red.**
   Given an error occurs (e.g. file write failure), when the error label renders, it must use a colour visually softer than pure `#FF0000` — specifically `rgb(220, 80, 80)` or equivalent — while remaining clearly red/error-toned.

5. **Setup heading text is larger than the default label size.**
   Given any phase of the setup wizard, when the screen renders, the main heading text must be rendered at a size visibly larger than body labels (egui heading size or explicit `size(18.0)` or above is acceptable).

### Setup screen — OAuth phase

6. **Status text and buttons remain centred.**
   Given the OAuth phase is shown, the status label and both buttons ("Connect Spotify" / "Skip for now") must be horizontally centred. (Current buttons are already centred — this AC confirms no regression.)

### Setup screen — Fetching phase (4 progress bars)

7. **All four fetch steps are visible simultaneously.**
   Given the setup wizard is in the Fetching phase, when any step is active, the screen must display four labelled rows — one per step: "Sync Last.fm", "Expand similar artists", "Fetch top tracks", "Recalculate scores" — visible at the same time.

8. **Completed steps show a full progress bar.**
   Given step N is active (N > 1), steps 1 … N−1 must each show a progress bar rendered at 100% fill with a ✓ prefix or equivalent completion indicator.

9. **The active step shows an animating progress bar.**
   Given step N is active, the row for step N must show a progress bar that reflects current/total when a total is known, or an indeterminate animation when no total is available.

10. **Pending steps show an empty progress bar and are visually dimmed.**
    Given step N is active (N < 4), steps N+1 … 4 must each show an empty (0%) progress bar and be rendered in a weaker/greyed colour so the user can distinguish them from the active step.

11. **Error state still shown on failure.**
    Given a fetch step fails, the failed step's row must show an error indicator (✗ or red text) and the "Continue anyway" button must still appear. The other step rows remain visible.

### Settings screen — Data section

12. **"Update everything" and individual sync buttons are centred.**
    Given the Settings screen is open and showing the Data section, the "Update everything" button and the row of individual sync buttons must be horizontally centred.

13. **Error text in Data section uses the lighter red.**
    Given a sync operation fails and the error result is shown, the error label must use `rgb(220, 80, 80)` (matching AC 4) rather than pure `#FF0000`.

14. **"Update everything" running shows 4 simultaneous progress bars.**
    Given the user clicked "Update everything" and the operation is running, the Data section must display four labelled progress bars (one per step) with the same completed/active/pending visual distinction as ACs 8–10.

15. **Single operation running shows a single progress bar.**
    Given the user clicked one of the individual sync buttons (e.g. "Sync Last.fm"), the Data section must show a single progress bar for that operation only (not four bars).

16. **Save / Discard / Close buttons are centred.**
    Given unsaved changes exist in Settings, the "Save" and "Discard changes" buttons must be centred. The "Close" button at the bottom of the settings panel must also be centred.

## Out of Scope
- Changes to any settings knob layout (knobs stay left-aligned in horizontal rows).
- Changing fonts, installing custom typefaces, or theming beyond the targeted adjustments above.
- Resizing the setup window dimensions.
- Any logic changes to how operations run or how results are stored.
- Progress bars in the main player window.

## Edge Cases
1. **Fetch completes before re-render** — if a step completes in the same frame it started, step is shown as completed (full bar). No visual artefact.
2. **No total available** — when `active.total` is `None`, the active step's bar shows indeterminate (animated). Once total becomes available it switches to determinate.
3. **Single-operation "Update everything" with step=(n, 4)** — detect 4-step mode in Settings by checking `active.step.map(|(_, t)| t) == Some(4)`. Single ops have `step = None` — show one bar only.
4. **All steps complete** — when `active = None` and `last_result = Ok(...)`, in setup the window auto-closes (existing behaviour). In Settings all four bars should appear full until the next repaint clears them.

## UI/UX Notes

### Setup — Fields phase
```
[space 20]
[centred] Heading "Welcome to Gurdo"  (size ≥ 18)
[space 8]
[centred] "Enter your Last.fm username to get started."
[space 16]
[centred] TextEdit — max width 260 px, centred
[space 12]
[centred] Error label in rgb(220,80,80) if present
[space 8]
[centred] [ Continue ]  (disabled until input non-empty)
```

### Setup — Fetching phase / Settings UpdateAll
```
1. [✓] Sync Last.fm           [████████████] 100%
2. [▶] Expand similar artists [████░░░░░░░░]  42%   ← active, determinate
   weak: "fetching page 12..."
3. [·] Fetch top tracks        [░░░░░░░░░░░░]   0%   ← pending, dimmed
4. [·] Recalculate scores      [░░░░░░░░░░░░]   0%   ← pending, dimmed
```
Use `egui::ProgressBar`. Completed bars: `progress(1.0)`. Active: `progress(frac)` or `.animate(true)` when indeterminate. Pending: `progress(0.0)` with weak text colour on the label.

## Security / Backwards Compatibility
- Pure visual change. No state mutations, no new fields on any struct.
- `OperationKind` step inference: completed steps are inferred from `active.step.0 - 1` — no new stored state needed.
- Existing unit tests for `handle_close`, phase transitions, etc. are unaffected.
