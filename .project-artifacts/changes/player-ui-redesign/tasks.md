# Tasks — Player UI polish: consistent ghost-style controls

## DEV

- [x] T-1 DESIGN: Define ghost-button visuals helper — a small inline function or closure that
  sets `ui.visuals_mut().widgets.{inactive,hovered,active}.{bg_fill,weak_bg_fill}` to the
  agreed transparent/tint values before rendering a button group, and optionally restores
  afterwards. Covers AC-1, AC-2, AC-7, AC-8.

- [x] T-2: Fix progress bar colours — change `extreme_bg_color` to `rgba(255,255,255,25)` and
  bar fill to `rgba(255,255,255,160)`. Covers AC-3.

- [x] T-3: Apply ghost style to transport row — wrap the 5-button `ui.horizontal` block with the
  visuals helper from T-1; remove per-button `.fill()` calls with individual alphas. Covers
  AC-1, AC-2.

- [x] T-4: Split action row — move Like/Dislike into a new `ui.horizontal` block (160×38
  buttons), then Queue/Settings into a second `ui.horizontal` block (38×38 buttons), both below
  transport. Remove the old combined row. Apply ghost visuals to both new rows. Covers AC-4,
  AC-5, AC-6, AC-7, AC-8.

## QA

- [ ] T-5 E2E/UI: Verify ghost style — `cargo build` clean; `cargo test` green; manual check
  that all buttons render with consistent appearance in the running app. Covers AC-9, AC-10.
