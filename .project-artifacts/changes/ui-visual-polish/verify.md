# Verification: ui-visual-polish

## Test environment
Binary: `cargo build` → `target/debug/gurdo`. No external stubs required — pure UI change.

## E2E tests — Type: UI (all manual)
All 12 scenarios manual — egui has no automated UI test toolchain.

## Run results

| Scenario | Status |
|----------|--------|
| T-01 Fields: centred layout, narrow input, centred button | PASS |
| T-02 Fields: lighter red error | PASS |
| T-03 Heading visibly larger | PASS |
| T-04 OAuth centred (regression) | PASS |
| T-05 Fetching: 4 steps visible at once | PASS |
| T-06 Fetching: done/active/pending states | PASS |
| T-07 Fetching: error state preserved | PASS |
| T-08 Settings Data buttons centred | PASS |
| T-09 Settings lighter red error | PASS |
| T-10 Settings UpdateAll → 4 bars | PASS |
| T-11 Settings individual op → 1 bar | PASS |
| T-12 Settings Save/Discard/Close centred | PASS |

## AC coverage table

| AC | Scenario | Result |
|----|----------|--------|
| AC 1–3 Fields layout | T-01 | PASS |
| AC 4 Lighter red error | T-02 | PASS |
| AC 5 Larger heading | T-03 | PASS |
| AC 6 OAuth centred | T-04 | PASS |
| AC 7 4 steps simultaneously | T-05 | PASS |
| AC 8–10 Done/active/pending states | T-06 | PASS |
| AC 11 Error state preserved | T-07 | PASS |
| AC 12 Settings buttons centred | T-08 | PASS |
| AC 13 Settings lighter error | T-09 | PASS |
| AC 14 UpdateAll → 4 bars | T-10 | PASS |
| AC 15 Individual op → 1 bar | T-11 | PASS |
| AC 16 Save/Discard/Close centred | T-12 | PASS |

## Packaging
Skipped — policy: milestone.
