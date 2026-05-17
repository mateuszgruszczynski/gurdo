# Iteration 5 Development — Settings viewport window (EP-6)

*Epic: EP-6 · Phase: Development · Date: 2026-05-12*

---

## Baseline

`cargo build` before changes: 53 warnings (pre-existing baseline). Zero failures.

---

## Files changed

| File | Change |
|---|---|
| `src/config.rs` | Added `player_window_size`, `settings_window_size` to `UiConfig`; `#[allow(dead_code)]` on `Config::save` (used EP-8) |
| `src/ui/settings.rs` | Full implementation: 7 placeholder sections |
| `src/ui/player.rs` | Removed `SettingsDraft`/`spawn_cli`/modal; `settings_open: Arc<AtomicBool>`; `settings_initial_pos`; `show_viewport_deferred`; `#[allow(dead_code)]` on `config_path` |
| `src/ui/mod.rs` | Removed `SettingsDraft` init; reads `player_window_size` from config |

---

## Tasks completed

| # | Task | Status |
|---|---|---|
| T-1 | Config: `player_window_size` + `settings_window_size` | [x] |
| T-2 | `settings.rs`: 7 placeholder sections | [x] |
| T-3 | Remove `SettingsDraft`, `spawn_cli`, old modal | [x] |
| T-4 | `settings_open: Arc<AtomicBool>` + `⚙` handler | [x] |
| T-5 | `show_viewport_deferred` + close_requested handler | [x] |
| T-6 | `mod.rs`: window size from config, remove draft init | [x] |
| T-7 | `cargo build` — 53 warnings, zero new | [x] |

---

## Key decisions / fixes

- **`egui::Rect` has no `Default`:** `ctx.input(|i| i.viewport().outer_rect).unwrap_or_default()` failed. Fixed with `.unwrap_or(egui::Rect::ZERO)`. Similarly `settings_initial_pos.unwrap_or_default()` → `.unwrap_or(egui::Pos2::ZERO)`.
- **Two new dead_code warnings** after removing `SettingsDraft`: `config_path` field and `Config::save` method — both needed in EP-8. Suppressed with `#[allow(dead_code)] // used by EP-8`.
- **`SettingsDraft` removal** is intentional regression: knob DragValues return in EP-8.

---

## External interfaces

None — pure UI restructuring.

---

## Self-review

- No logic beyond spec. ✓
- No new dependencies. ✓
- `cargo build` green, 53 warnings. ✓
- Committed: `feat(ui): settings as deferred OS viewport` (21a4f0c)
