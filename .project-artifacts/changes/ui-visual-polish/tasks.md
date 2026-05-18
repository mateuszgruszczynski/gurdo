# Tasks: ui-visual-polish

## DEV-1 — Setup Fields: centred layout + narrow input + centred button
**Role:** DEV
**Description:** Wrap `show_fields()` content in `ui.vertical_centered`. Replace `desired_width(f32::INFINITY)` with a max-width `TextEdit` (≤ 260 px) inside a centred container. Replace right-to-left "Continue" layout with `ui.vertical_centered`. Centre error label.
**Dependencies:** none
**Done when:** Fields phase is fully centred; input is narrow; Continue button centred. ACs 1–3.

## DEV-2 — Setup all phases: larger heading + lighter error colour
**Role:** DEV
**Description:** Apply `egui::RichText::new(...).size(18.0)` (or `.heading()` override) to setup headings. Replace `Color32::RED` with `Color32::from_rgb(220, 80, 80)` in `show_fields()` error label and `show_fetching()` error label.
**Dependencies:** none (independent of DEV-1)
**Done when:** Headings visibly larger; errors are soft red. ACs 4–5.

## DEV-3 — Setup Fetching: 4 simultaneous progress bars
**Role:** DEV
**Description:** Rewrite `show_fetching()` to render all 4 steps at once. Derive step state from `active.step`: completed (< current), active (= current), pending (> current). Completed → `ProgressBar::new(1.0)` + ✓ prefix. Active → determinate `ProgressBar::new(frac)` when total known, `.animate(true)` otherwise. Pending → `ProgressBar::new(0.0)` + weak label colour. Keep error/Continue-anyway logic.
**Dependencies:** none (independent of DEV-1/2)
**Done when:** All 4 rows visible during fetch; states render correctly. ACs 7–11.

## DEV-4 — Settings Data section: centre buttons + lighter error
**Role:** DEV
**Description:** Wrap "Update everything" button and the `ui.horizontal` row of individual sync buttons in `ui.vertical_centered`. Change `Color32::RED` in the result label to `Color32::from_rgb(220, 80, 80)`.
**Dependencies:** none
**Done when:** Data buttons centred; error colour softened. ACs 12–13.

## DEV-5 — Settings: 4-bar progress for UpdateAll, single bar for individual ops
**Role:** DEV
**Description:** Replace the existing single active-op label block in settings with a helper that checks `active.step.map(|(_, t)| t) == Some(4)`. If true (UpdateAll), render 4 progress bars (same logic as DEV-3). If false or None, render a single progress bar for the active op. Use `egui::ProgressBar`.
**Dependencies:** DEV-4
**Done when:** UpdateAll shows 4 bars; individual ops show 1 bar. ACs 14–15.

## DEV-6 — Settings: centre Save/Discard/Close buttons
**Role:** DEV
**Description:** Wrap the Save/Discard `ui.horizontal` in `ui.vertical_centered`. Wrap the "Close" button call in `ui.vertical_centered`.
**Dependencies:** none
**Done when:** Save, Discard, and Close are centred. AC 16.

## QA-1 — E2E UI: 4 bars visible during setup Fetching and Settings UpdateAll
**Role:** QA
**Description:** Manual E2E — run setup through to Fetching phase and confirm 4 labelled rows with progress bars are visible simultaneously. Then in Settings trigger "Update everything" and confirm 4 bars appear there too.
**Dependencies:** DEV-3, DEV-5
**Done when:** Both locations confirmed by human tester.

## QA-2 — E2E UI: individual sync op shows single bar
**Role:** QA
**Description:** Manual E2E — in Settings trigger a single op (e.g. "Sync Last.fm") and confirm only one progress bar appears, not four.
**Dependencies:** DEV-5
**Done when:** Confirmed by human tester.

## DEV-7 — CHANGELOG entry
**Role:** DEV
**Description:** Add entry under [Unreleased] in CHANGELOG.md.
**Dependencies:** DEV-1 through DEV-6
**Done when:** Entry present.
