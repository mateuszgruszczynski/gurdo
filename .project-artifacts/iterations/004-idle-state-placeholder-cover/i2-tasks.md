# Iteration 4 Tasks — Idle-state placeholder cover (EP-5)

*Epic: EP-5 · Phase: Decomposition · Date: 2026-05-12*

---

## DEV tasks

| # | Task | AC(s) | Status |
|---|---|---|---|
| T-1 | Add `placeholder_texture: Option<egui::TextureHandle>` field to `GurdoApp` in `player.rs`; add `placeholder_texture: None` to struct literal in `mod.rs` | AC-1 | [ ] |
| T-2 | Add lazy-init block at top of `GurdoApp::update()` — decode `assets::PLACEHOLDER_COVER` on first frame, store result | AC-2 | [ ] |
| T-3 | Update album art rendering branch: show placeholder when `album_texture` is `None` | AC-3 | [ ] |
| T-4 | Remove `#[allow(dead_code)]` annotation from `PLACEHOLDER_COVER` in `assets.rs` | AC-6 | [ ] |

## QA / cross-cutting tasks

| # | Task | AC(s) | Status |
|---|---|---|---|
| T-5 | `cargo build` — confirm zero new warnings (baseline: 53 pre-existing) | AC-7 | [ ] |

## Notes

- T-1 through T-4 are sequential (all in `player.rs`/`mod.rs`/`assets.rs`).
- AC-4 (real cover unaffected) and AC-5 (static bg during idle) are verified by smoke test in Integration — no separate task needed.
- No new test files for this size-S epic; Integration smoke covers the ACs.
