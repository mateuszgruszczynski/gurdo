# Iteration 5 Tasks — Settings viewport window (EP-6)

*Epic: EP-6 · Phase: Decomposition · Date: 2026-05-12*

---

## DEV tasks

| # | Task | AC(s) | Status |
|---|---|---|---|
| T-1 | `src/config.rs`: add `player_window_size` and `settings_window_size` to `UiConfig`; add default fns; update `impl Default for UiConfig` | AC-1 | [ ] |
| T-2 | `src/ui/settings.rs`: implement `pub(super) fn render(ctx)` with 7 placeholder sections | AC-5 | [ ] |
| T-3 | `src/ui/player.rs`: remove `SettingsDraft` struct + impl, `spawn_cli` fn, old `egui::Window("Settings")` block | AC-7 | [ ] |
| T-4 | `src/ui/player.rs`: replace `settings_open: bool` with `Arc<AtomicBool>`; add `settings_initial_pos: Option<egui::Pos2>` field; update `⚙` button handler | AC-3, AC-4, AC-6 | [ ] |
| T-5 | `src/ui/player.rs`: add `show_viewport_deferred` call after `CentralPanel`; wire `close_requested()` → flag; call `settings::render` | AC-3, AC-4, AC-6 | [ ] |
| T-6 | `src/ui/mod.rs`: read `player_window_size` from config for `NativeOptions`; remove `SettingsDraft` construction; init `settings_open: Arc::new(AtomicBool::new(false))`, `settings_initial_pos: None` | AC-2, AC-7 | [ ] |

## QA / cross-cutting tasks

| # | Task | AC(s) | Status |
|---|---|---|---|
| T-7 | `cargo build` — confirm zero new warnings (baseline: 53) | AC-8 | [ ] |

## Notes

- T-1 is independent; T-2 is independent. T-3–T-6 are sequential (all in `player.rs`/`mod.rs`).
- T-3 before T-4/T-5 to avoid referencing removed code.
- No new test files for this size-M epic; ACs verified by Integration smoke.
