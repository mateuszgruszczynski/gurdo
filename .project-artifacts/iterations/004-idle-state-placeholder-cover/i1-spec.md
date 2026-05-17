# Iteration 4 Spec — Idle-state placeholder cover (EP-5)

*Epic: EP-5 · Phase: Refinement · Date: 2026-05-12*

---

## Problem

When no track is playing (first launch, playback gaps, paused with no track loaded),
`src/ui/player.rs` renders an empty 400×400 invisible gap via `ui.allocate_space`.
The result is a layout that looks broken.

`assets::PLACEHOLDER_COVER` (a 400×400 gray PNG, ~1 KB) was embedded in EP-3 and
is currently suppressed with `#[allow(dead_code)]`. This epic plugs it in.

---

## Scope

### In scope
- Add `placeholder_texture: Option<egui::TextureHandle>` to `GurdoApp`.
- Lazy-init: decode `assets::PLACEHOLDER_COVER` once on the first `update()` frame
  via the existing `decode_image` helper; store in the field.
- Render: when `album_texture` is `None`, show the placeholder using the exact same
  `egui::Image` parameters as the real cover (`400×400`, `rounding(10.0)`).
- Remove `#[allow(dead_code)]` from `PLACEHOLDER_COVER` in `assets.rs`.
- Background behaviour unchanged: `BackgroundPainter` already falls back to the
  static config colour when `cover_bytes` is `None` — no additional work needed.

### Out of scope
- Animated placeholders, multiple variations, "no track" text overlay.
- Any changes to background, blur, or layout logic beyond the image slot.

---

## Acceptance criteria

| # | Criterion |
|---|---|
| AC-1 | `GurdoApp` has a `placeholder_texture: Option<egui::TextureHandle>` field, initialised to `None`. |
| AC-2 | On the first `update()` frame, `PLACEHOLDER_COVER` is decoded and stored in `placeholder_texture`; it is never decoded again after that. |
| AC-3 | When `album_texture` is `None`, the 400×400 slot renders the placeholder with `rounding(10.0)` — identical widget call to the real cover. |
| AC-4 | When a track is playing (`album_texture` is `Some`), the real cover renders normally — no regression. |
| AC-5 | Background colour during idle state is the static config colour (no blur artifact). |
| AC-6 | `#[allow(dead_code)]` annotation is removed from `PLACEHOLDER_COVER` in `assets.rs`. |
| AC-7 | `cargo build` produces zero new compiler warnings beyond the 53 pre-existing baseline. |

---

## Implementation notes

**`src/ui/player.rs`**

1. Add field to `GurdoApp`:
   ```rust
   pub(super) placeholder_texture: Option<egui::TextureHandle>,
   ```

2. At the top of `update()`, lazy-init once:
   ```rust
   if self.placeholder_texture.is_none() {
       if let Ok(img) = decode_image(super::assets::PLACEHOLDER_COVER) {
           self.placeholder_texture = Some(
               ctx.load_texture("placeholder_cover", img, Default::default())
           );
       }
   }
   ```

3. Album art rendering section (currently lines ~101–106):
   ```rust
   if let Some((_, tex)) = &self.album_texture {
       ui.add(egui::Image::new((tex.id(), egui::vec2(400.0, 400.0))).rounding(10.0));
   } else if let Some(tex) = &self.placeholder_texture {
       ui.add(egui::Image::new((tex.id(), egui::vec2(400.0, 400.0))).rounding(10.0));
   } else {
       ui.allocate_space(egui::vec2(400.0, 400.0));
   }
   ```
   (The final `else` branch is unreachable after first frame but kept for safety.)

**`src/ui/mod.rs`**

Add `placeholder_texture: None` to the `GurdoApp { ... }` initialisation block.

**`src/ui/assets.rs`**

Remove the `#[allow(dead_code)] // used by EP-5` attribute above `PLACEHOLDER_COVER`.

---

## Files changed (expected)

| File | Change |
|---|---|
| `src/ui/player.rs` | Add field + lazy-init + updated album art branch |
| `src/ui/mod.rs` | Add `placeholder_texture: None` to struct literal |
| `src/ui/assets.rs` | Remove `#[allow(dead_code)]` line |

No new dependencies. No new test files (size S; covered by smoke test in Integration).
