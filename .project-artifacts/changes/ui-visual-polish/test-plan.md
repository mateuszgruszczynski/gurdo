# Test Plan: ui-visual-polish

All visual changes produce no in-process-testable logic (no state mutations, no new data structures). Unit-level coverage is N/A for pure layout/rendering code in an immediate-mode GUI. All scenarios are E2E / UI / manual.

---

## T-01 — Setup Fields: centred layout + narrow input
**Covers AC:** 1, 2, 3
**Level:** E2E | **Type:** UI | **Owned by:** Verification (manual)

```
Given  fresh setup (delete ~/.gurdo/secrets.toml), launch gurdo
When   the Fields phase renders
Then   the heading, instruction label, and username input are horizontally centred
And    the input field is noticeably narrower than the full window width
And    the Continue button is centred, not right-aligned
```

---

## T-02 — Setup Fields: error uses lighter red
**Covers AC:** 4
**Level:** E2E | **Type:** UI | **Owned by:** Verification (manual)

```
Given  the Fields phase is shown
When   a write error is injected (e.g. make ~/.gurdo unwritable) and the error label appears
Then   the error text is displayed in a soft red — visibly lighter than pure #FF0000
```
**Notes:** Can be approximated by temporarily chmod 000 ~/.gurdo during testing.

---

## T-03 — Setup heading text is visibly larger
**Covers AC:** 5
**Level:** E2E | **Type:** UI | **Owned by:** Verification (manual)

```
Given  any phase of the setup wizard
When  the screen renders
Then  the main heading text is clearly larger than body label text
```

---

## T-04 — Setup OAuth: centred layout regression
**Covers AC:** 6
**Level:** E2E | **Type:** UI | **Owned by:** Verification (manual)

```
Given  the setup wizard reaches the OAuth phase
When  the screen renders
Then  the status label and both buttons are horizontally centred (no regression)
```

---

## T-05 — Setup Fetching: all 4 steps visible simultaneously
**Covers AC:** 7
**Level:** E2E | **Type:** UI | **Owned by:** Verification (manual)

```
Given  the setup wizard has entered the Fetching phase (user clicked "Fetch now")
When  step 2 or later is active
Then  four labelled rows are visible: Sync Last.fm, Expand similar artists, Fetch top tracks, Recalculate scores
And   all four rows are on screen at the same time
```

---

## T-06 — Setup Fetching: completed / active / pending states
**Covers AC:** 8, 9, 10
**Level:** E2E | **Type:** UI | **Owned by:** Verification (manual)

```
Given  step 2 (Expand similar artists) is active
When  the screen renders
Then  step 1 shows a full progress bar with a completion indicator (✓ or equivalent)
And   step 2 shows a progress bar that is partially filled or animating
And   steps 3 and 4 show empty bars and are rendered in a weaker/greyed colour
```

---

## T-07 — Setup Fetching: error state preserved
**Covers AC:** 11
**Level:** E2E | **Type:** UI | **Owned by:** Verification (manual)

```
Given  step 2 fails (network unavailable or invalid credentials)
When  the error is reported
Then  the failed step's row shows an error indicator (✗ or red text)
And   a "Continue anyway" button appears
And   all four step rows remain visible
```

---

## T-08 — Settings Data: buttons centred
**Covers AC:** 12
**Level:** E2E | **Type:** UI | **Owned by:** Verification (manual)

```
Given  Settings is open
When  the Data section renders
Then  the "Update everything" button is horizontally centred
And   the row of individual sync buttons is horizontally centred
```

---

## T-09 — Settings Data: error uses lighter red
**Covers AC:** 13
**Level:** E2E | **Type:** UI | **Owned by:** Verification (manual)

```
Given  a sync operation has failed and the error result is displayed
When  the result label renders
Then  the error text is in soft red matching the setup error colour (rgb 220, 80, 80)
```

---

## T-10 — Settings: UpdateAll shows 4 simultaneous progress bars
**Covers AC:** 14
**Level:** E2E | **Type:** UI | **Owned by:** Verification (manual)

```
Given  Settings is open and "Update everything" has been clicked
When  the operation is in progress (any step active)
Then  four labelled progress bars are visible simultaneously
And   completed steps show full bars, the active step shows a partial or animating bar, pending steps show empty dimmed bars
```

---

## T-11 — Settings: individual op shows single progress bar
**Covers AC:** 15
**Level:** E2E | **Type:** UI | **Owned by:** Verification (manual)

```
Given  Settings is open and a single sync button (e.g. "Sync Last.fm") has been clicked
When  the operation is in progress
Then  exactly one progress bar is visible (not four)
```

---

## T-12 — Settings: Save / Discard / Close buttons centred
**Covers AC:** 16
**Level:** E2E | **Type:** UI | **Owned by:** Verification (manual)

```
Given  Settings is open and a knob value has been changed (making Save/Discard visible)
When  the bottom of the settings panel is shown
Then  the Save and Discard changes buttons are horizontally centred
And   the Close button is horizontally centred
```

---

## AC Coverage Table

| AC | Scenario | Level / Type |
|----|----------|-------------|
| AC 1 — Fields heading/label centred | T-01 | E2E / UI |
| AC 2 — Input narrow + centred | T-01 | E2E / UI |
| AC 3 — Continue centred | T-01 | E2E / UI |
| AC 4 — Lighter red error | T-02 | E2E / UI |
| AC 5 — Larger heading text | T-03 | E2E / UI |
| AC 6 — OAuth centred (regression) | T-04 | E2E / UI |
| AC 7 — 4 steps visible at once | T-05 | E2E / UI |
| AC 8 — Completed = full bar | T-06 | E2E / UI |
| AC 9 — Active = animating bar | T-06 | E2E / UI |
| AC 10 — Pending = empty + dimmed | T-06 | E2E / UI |
| AC 11 — Error state preserved | T-07 | E2E / UI |
| AC 12 — Settings buttons centred | T-08 | E2E / UI |
| AC 13 — Settings lighter error | T-09 | E2E / UI |
| AC 14 — UpdateAll → 4 bars | T-10 | E2E / UI |
| AC 15 — Individual op → 1 bar | T-11 | E2E / UI |
| AC 16 — Save/Discard/Close centred | T-12 | E2E / UI |
