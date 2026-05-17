# Spec — Player UI polish: consistent ghost-style controls

## Context

`src/ui/player.rs` renders the main player window. Transport buttons have five different fill
alpha values (100, 150, 200, 50, 75 — leftovers from early prototyping). The Like/Dislike/Queue/
Settings row mixes two different button sizes (110×40 feedback vs 40×40 icons) with the same
dark fill, producing an unbalanced row. The progress bar uses hardcoded dark colours that clash
on light-tinted cover-art backgrounds. The background is dynamic (blurred cover art with a
top-to-bottom gradient overlay), so controls must read cleanly at any tint.

## Out of scope

- Settings window (separate viewport)
- Album art display, sizing, or rounding
- Track info typography
- Queue display
- Any new functionality or commands

## Before / After

| Element | Before | After |
|---|---|---|
| Transport buttons | 5 different fill alphas (100/150/200/50/75) | Transparent fill; uniform subtle stroke |
| Progress bar | Dark hardcoded colours (`rgba(60,60,60,100)` track, `rgba(25,25,25,200)` fill) | Semi-transparent white track + white fill |
| Action row layout | Like + Dislike + Queue + Settings in one mixed-size row | Like/Dislike on their own row; Queue + Settings on a row below |
| Button styling | Mixed fill opacities | All buttons: transparent fill, consistent rounding, white text |

## Design

### Ghost button style

All interactive buttons (transport, feedback, utility) use the same visual language:
- **Fill (idle):** transparent — background shows through
- **Fill (hover):** faint white tint, `rgba(255,255,255,20)`
- **Fill (pressed):** slightly stronger tint, `rgba(255,255,255,40)`
- **Stroke:** subtle white border, consistent across all buttons
- **Rounding:** `Rounding::same(6.0)` for all buttons
- **Text colour:** `Color32::WHITE` (set via global visuals — overrides egui default grey)

The hover/press tints are applied by overriding `ui.visuals_mut().widgets.*` before each button
group, then restoring afterwards so the rest of the UI is unaffected.

### Progress bar

- Track background (`extreme_bg_color`): `rgba(255,255,255,25)` — subtle white ghost track
- Fill: `rgba(255,255,255,160)` — white, partially transparent

### Layout (three rows, all vertically centred)

```
[ ⏮  ⏪  ▶/⏸  ⏩  ⏭ ]          ← transport  60×60 each
[ ♥ Like        👎 Dislike ]     ← feedback   160×38 each
[ ☰  ⚙ ]                         ← utility    38×38 each
```

- Transport row: unchanged 60×60 per button, 5 buttons, centred
- Feedback row: two wider buttons (160×38), centred; keep text labels
- Utility row: two square icon buttons (38×38), centred

### Active states (unchanged behaviour, style adjusted)

- Liked track: Like button label colour → `rgb(29,185,84)` (Spotify green)
- Disliked track: Dislike label colour → `Color32::RED`
- Neither: both labels `Color32::WHITE`

## Acceptance Criteria

| # | Criterion |
|---|-----------|
| AC-1 | All five transport buttons have identical fill, stroke, and rounding. |
| AC-2 | Transport button fills are transparent in idle state (no hardcoded alpha variation). |
| AC-3 | Progress bar track is `rgba(255,255,255,25)` and fill is `rgba(255,255,255,160)`. |
| AC-4 | Like and Dislike appear on their own row, vertically separated from transport. |
| AC-5 | Like and Dislike retain text labels ("♥ Like"/"♥ Unlike" and "👎 Dislike"). |
| AC-6 | Queue (☰) and Settings (⚙) appear on a separate row below Like/Dislike. |
| AC-7 | All button rows use `Rounding::same(6.0)`. |
| AC-8 | Liked state shows green label; disliked state shows red label; ghost fills otherwise. |
| AC-9 | `cargo build` produces no new warnings. |
| AC-10 | `cargo test` passes all existing tests. |
