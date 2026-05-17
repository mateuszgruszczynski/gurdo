# Iteration 3 Tasks — Cover-blur background painter (EP-4)

*Epic: EP-4 · Phase: Decomposition · Date: 2026-05-12*

---

## Task list

| # | Title | Role | Depends on | Maps to AC |
|---|---|---|---|---|
| T-1 | Implement `BackgroundPainter` in `src/ui/background.rs` | DEV | — | AC-5, AC-8, AC-9, AC-10 |
| T-2 | Wire `BackgroundPainter` into `GurdoApp` in `src/ui/player.rs` | DEV | T-1 | AC-1, AC-2, AC-3, AC-4, AC-6, AC-7 |
| T-3 | Verify clean build | DEV | T-2 | AC-9, AC-10 |
| T-4 | Manual smoke test on host — visual + functional | DEV + DESIGN | T-3 | AC-1, AC-2, AC-3, AC-4, AC-6, AC-7, AC-8 |

Tasks are sequentially dependent: T-1 → T-2 → T-3 → T-4.

---

## Task details

### T-1 — Implement `BackgroundPainter` in `src/ui/background.rs`

**Role:** DEV  
**Depends on:** —

Replace `// placeholder — EP-4` with a complete implementation:

```rust
use std::sync::{Arc, Mutex};
use eframe::egui;

pub(super) struct BackgroundPainter {
    texture:       Option<egui::TextureHandle>,
    last_url:      String,
    pending:       Arc<Mutex<Option<egui::ColorImage>>>,
}
```

**`new()`** — returns a default-initialised painter (no texture, empty URL, empty pending slot).

**`update(ctx, cover_url, cover_bytes)`:**
- If `cover_bytes` is `None`: clear `texture`, reset `last_url` to `""`.
- If `cover_url` differs from `last_url`: clone `pending` Arc, spawn a thread that runs the blur pipeline and writes to the slot. Store the new URL as `last_url`.
- Poll `pending`: if `Some(img)` is present, call `ctx.load_texture("cover_blur", img, Default::default())` and store the handle; clear the slot back to `None`.

**Blur pipeline (inside spawned thread):**
```
image::load_from_memory(bytes)
  → .resize(256, 256, image::imageops::FilterType::Lanczos3)
  → convert to RgbaImage
  → image::imageops::fast_blur(&mut rgba_img, 30.0)
  → egui::ColorImage::from_rgba_unmultiplied([256, 256], rgba_pixels)
  → write to Arc<Mutex<Option<...>>>
```
Silently discard on any error (no panic, no UI crash).

**`paint(ctx, fallback_rgb)`:**
- If `texture` is `Some(tex)`: set `visuals.panel_fill = TRANSPARENT`, then draw the texture full-window using `ctx.layer_painter(LayerId::background())` + draw the gradient overlay (mesh quad: top vertex color `(0,0,0,60)`, bottom vertex color `(0,0,0,200)`).
- If `texture` is `None`: set `visuals.panel_fill = Color32::from_rgb(r,g,b)` (the fallback color); no texture drawing.

**Done when:** `cargo build` succeeds with `background.rs` implemented; no stale placeholder comment remains.

---

### T-2 — Wire `BackgroundPainter` into `GurdoApp` in `src/ui/player.rs`

**Role:** DEV  
**Depends on:** T-1

1. Add `blur: background::BackgroundPainter` field to `GurdoApp`.
2. Initialise it in the `GurdoApp { ... }` construction in `mod.rs` as `blur: background::BackgroundPainter::new()`.
3. In `GurdoApp::update()`, before `CentralPanel::default().show(...)`:
   ```rust
   let (cover_url, cover_bytes) = {
       let s = self.state.lock().unwrap();
       (s.album_art_url.clone(), s.album_art_bytes.clone())
   };
   self.blur.update(ctx, cover_url.as_deref(), cover_bytes.as_deref());
   let fallback = self.shared_config.lock().unwrap().ui.background_color;
   self.blur.paint(ctx, fallback);
   ```
4. Delete the existing lines that set `panel_fill` from `background_color` (currently lines 87–90 in player.rs). The `paint()` call now handles both the solid fallback and the transparent-when-blurred cases.

**Done when:** `GurdoApp` compiles with `blur` field wired; old `panel_fill` lines gone; `grep "panel_fill" src/ui/player.rs` returns no output.

---

### T-3 — Verify clean build

**Role:** DEV  
**Depends on:** T-2

Run `cargo build`. Confirm:
- Exit code 0.
- `cargo build 2>&1 | grep "^warning:.*background\|^warning:.*player"` returns no output.
- `grep -r "placeholder.*EP-4" src/ui/background.rs` returns no output.

**Done when:** All three checks pass.

---

### T-4 — Manual smoke test on host — visual + functional

**Role:** DEV + DESIGN  
**Depends on:** T-3

Launch `cargo run -- -c config.toml ui` on the host. Verify:

1. **AC-1** — Play a track with cover art. Background changes from solid dark to a visibly blurred full-window version of the art.
2. **AC-2** — Stop playback / queue empty. Background reverts to solid near-black.
3. **AC-3** — Play a track with a very bright cover (e.g. white/yellow album). Confirm the dark gradient overlay is visible and track name + controls remain readable.
4. **AC-4** — Skip to a different track with different art. Background updates to the new track's blurred art.
5. **AC-6** — The album art thumbnail in the layout is unchanged — still renders at 400×400 with rounding.
6. **AC-7** — Click ⏮ ⏪ ⏸ ⏩ ⏭ — all respond correctly while blur is active.
7. **AC-8** — Rapidly skip through 5 tracks. No crash, freeze, or visual corruption.

DESIGN check during step 1–3: Confirm the gradient visually darkens the lower portion without making the background look flat/muddy. Confirm legibility on both a dark and a bright cover.

**Done when:** All 7 items above confirmed on the host.

---

## Auto-continue assessment

All tasks map to ACs. Roles are DEV throughout; DESIGN is applied as a visual quality judgement during T-4's smoke test (no separate role task needed for an M-size feature). No scope added beyond the spec. **Auto-continue condition met → proceeding to Test Plan.**
