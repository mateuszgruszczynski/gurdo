# Iteration 5 Spec — Settings viewport window (EP-6)

*Epic: EP-6 · Phase: Refinement · Date: 2026-05-12*

---

## Problem

Settings is currently an `egui::Window` modal embedded in `player.rs::update()`.
It contains a `SettingsDraft` with inline `DragValue` knobs and `spawn_cli` subprocess
buttons. The Architecture calls for Settings to become a proper second OS window
(eframe deferred viewport), centered on the player at open time, that the user can
move independently. This epic builds the window shell; EP-7 and EP-8 will fill the
content sections.

---

## Scope

### In scope

1. **`src/config.rs`** — add `player_window_size: [u32; 2]` (default `[440, 660]`)
   and `settings_window_size: [u32; 2]` (default `[800, 900]`) to `UiConfig`.
   Both fields use `#[serde(default = ...)]` so existing `config.toml` files load
   without error.

2. **`src/ui/player.rs`** — remove `SettingsDraft`, `spawn_cli`, and the existing
   `egui::Window::new("Settings")` block. Replace `settings_open: bool` with
   `settings_open: Arc<AtomicBool>` so the deferred viewport callback can close
   itself. Add `settings_initial_pos: Option<egui::Pos2>` to record where the
   window should open (computed once per open gesture). Wire `show_viewport_deferred`
   in `update()`.

3. **`src/ui/settings.rs`** — replace the `// placeholder — EP-6` stub with
   `pub(super) fn render(ctx: &egui::Context)` that renders a scrollable window with
   7 visually-distinct placeholder sections.

4. **`src/ui/mod.rs`** — read `config.ui.player_window_size` for
   `NativeOptions::viewport.with_inner_size`. Remove the `SettingsDraft` construction
   and initialization.

### Out of scope

- Populating sections with real controls (EP-7 for Data/Spotify, EP-8 for knobs).
- `OperationsState` and `OperationCommand` (EP-7).
- Saving knob changes (EP-8).
- The existing inline knob DragValue widgets are intentionally removed and not migrated
  — they will return in EP-8 in the correct sections.

---

## Acceptance criteria

| # | Criterion |
|---|---|
| AC-1 | `UiConfig` has `player_window_size: [u32; 2]` (default `[440, 660]`) and `settings_window_size: [u32; 2]` (default `[800, 900]`); `config.toml` without these fields loads cleanly. |
| AC-2 | Player window size is driven by `config.ui.player_window_size` — not hardcoded. |
| AC-3 | Clicking `⚙` in the player opens a new OS-level window via `show_viewport_deferred`; the player remains fully interactive (play/pause, like/dislike, seek, etc.). |
| AC-4 | The settings window opens centred over the player's current position; the user can drag it away and it does not snap back. |
| AC-5 | The settings window contains 7 labelled, visually-distinct sections: **Data**, **Spotify**, **Recommendations**, **Engine**, **Artist Scoring**, **Sync**, **Appearance** — each with a placeholder label. |
| AC-6 | Clicking the OS close button on the settings window dismisses it and re-enables the `⚙` button to reopen it. |
| AC-7 | `SettingsDraft`, `spawn_cli`, and the old `egui::Window("Settings")` block are deleted from `player.rs`. |
| AC-8 | `cargo build` produces zero new warnings beyond the 53 pre-existing baseline. |

---

## Implementation notes

### `src/config.rs`

Add to `UiConfig`:
```rust
#[serde(default = "default_player_window_size")]
pub player_window_size: [u32; 2],
#[serde(default = "default_settings_window_size")]
pub settings_window_size: [u32; 2],
```
Add fns:
```rust
fn default_player_window_size() -> [u32; 2] { [440, 660] }
fn default_settings_window_size() -> [u32; 2] { [800, 900] }
```
Update `impl Default for UiConfig` to include both fields.

### `src/ui/settings.rs`

```rust
use eframe::egui;

pub(super) fn render(ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            section(ui, "Data",            "Sync Last.fm · Expand · Fetch Tracks · Score — coming in EP-7");
            section(ui, "Spotify",         "Login & device status — coming in EP-7");
            section(ui, "Recommendations", "Queue size and sampling knobs — coming in EP-8");
            section(ui, "Engine",          "Similar-artist weight knobs — coming in EP-8");
            section(ui, "Artist Scoring",  "Score formula knobs — coming in EP-8");
            section(ui, "Sync",            "Sync limit knobs — coming in EP-8");
            section(ui, "Appearance",      "Read-only config display — coming in EP-8");
        });
    });
}

fn section(ui: &mut egui::Ui, title: &str, placeholder: &str) {
    ui.add_space(8.0);
    ui.heading(title);
    ui.separator();
    ui.add_space(4.0);
    ui.label(egui::RichText::new(placeholder).color(egui::Color32::GRAY).italics());
    ui.add_space(8.0);
}
```

### `src/ui/player.rs`

**Struct changes:**
```rust
pub(super) struct GurdoApp {
    // ... existing fields ...
    // remove: settings_draft: SettingsDraft
    // remove: settings_open: bool
    pub(super) settings_open:        Arc<std::sync::atomic::AtomicBool>,
    pub(super) settings_initial_pos: Option<egui::Pos2>,
}
```

**`⚙` button handler** (in `update()`):
```rust
if ui.add_sized(icon_size, egui::Button::new("⚙")...).clicked() {
    let was_open = self.settings_open.load(Ordering::Relaxed);
    if !was_open {
        let player_rect = ctx.input(|i| i.viewport().outer_rect).unwrap_or_default();
        let [sw, sh] = self.shared_config.lock().unwrap().ui.settings_window_size;
        self.settings_initial_pos = Some(egui::pos2(
            player_rect.center().x - sw as f32 / 2.0,
            player_rect.center().y - sh as f32 / 2.0,
        ));
    }
    self.settings_open.store(!was_open, Ordering::Relaxed);
}
```

**Deferred viewport call** (after `CentralPanel`, before error modal):
```rust
if self.settings_open.load(Ordering::Relaxed) {
    let [sw, sh] = self.shared_config.lock().unwrap().ui.settings_window_size;
    let pos = self.settings_initial_pos.unwrap_or_default();
    let settings_open = Arc::clone(&self.settings_open);
    ctx.show_viewport_deferred(
        egui::ViewportId::from_hash_of("settings"),
        egui::ViewportBuilder::default()
            .with_title("Gurdo — Settings")
            .with_inner_size([sw as f32, sh as f32])
            .with_position(pos),
        move |ctx, _class| {
            if ctx.input(|i| i.viewport().close_requested()) {
                settings_open.store(false, Ordering::Relaxed);
            }
            super::settings::render(ctx);
        },
    );
}
```

Remove: `SettingsDraft` struct + impl, `spawn_cli` fn, `if self.settings_open { egui::Window::new("Settings") ... }` block.

### `src/ui/mod.rs`

- Remove `SettingsDraft` import and `SettingsDraft::from_config(&...)` call.
- Read window size from config for `NativeOptions`:
  ```rust
  let [pw, ph] = shared_config.lock().unwrap().ui.player_window_size;
  // in NativeOptions viewport builder:
  .with_inner_size([pw as f32, ph as f32])
  ```
- Initialize new fields:
  ```rust
  GurdoApp {
      // ...
      settings_open:        Arc::new(AtomicBool::new(false)),
      settings_initial_pos: None,
  }
  ```

---

## Files changed (expected)

| File | Change |
|---|---|
| `src/config.rs` | Add `player_window_size`, `settings_window_size` to `UiConfig` |
| `src/ui/settings.rs` | Full implementation (7 placeholder sections) |
| `src/ui/player.rs` | Remove `SettingsDraft`/`spawn_cli`/modal; add `AtomicBool` + deferred viewport |
| `src/ui/mod.rs` | Read player window size from config; remove `SettingsDraft` init |

No new Cargo.toml dependencies.
