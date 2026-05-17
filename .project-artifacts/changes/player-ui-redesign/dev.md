# Dev notes — Player UI polish

## Files changed

- `src/ui/player.rs` — single file, all changes in the `update()` method

## Changes made

**Progress bar** (lines ~100-105):
- `extreme_bg_color`: `rgba(60,60,60,100)` → `rgba(255,255,255,25)`
- Bar fill: `rgba(25,25,25,200)` → `rgba(255,255,255,160)`

**Ghost visuals block** (new, inserted before transport row):
- Sets `widgets.{inactive,hovered,active}.{weak_bg_fill,bg_fill,bg_stroke,fg_stroke}` on the
  parent `ui` inside `vertical_centered`. All child rows inherit these visuals.
- Idle: transparent fill, `rgba(255,255,255,60)` stroke, white text
- Hover: `rgba(255,255,255,20)` tint, `rgba(255,255,255,100)` stroke
- Press: `rgba(255,255,255,40)` tint, `rgba(255,255,255,140)` stroke

**Transport row**: removed per-button `.fill()` calls (had alphas 100/150/200/50/75);
rounding 5.0 → 6.0.

**Action rows split** (was one combined row):
- Feedback row: Like + Dislike, 160×38, ghost style, white idle text (was GRAY)
- Utility row: Queue ☰ + Settings ⚙, 38×38, ghost style

## In-process test results

- `cargo build`: 1 warning (pre-existing `last_track_uri` assignment), no new warnings
- `cargo test`: 16/16 pass
