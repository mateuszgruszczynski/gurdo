# i1 Spec — EP-4: Cover-Blur Background Painter

## 1. Context and Goal

Gurdo currently renders a flat, solid background color behind all playback controls. This iteration replaces that static fill with a blurred, downscaled copy of the current track's album art, giving the player a visually immersive look that reacts to what is playing. A dark vertical gradient is painted over the blur to ensure all text and controls remain legible at all times. When no track is playing, the background reverts seamlessly to the solid color already configured by the user.

## 2. Scope

### In Scope
- Implement `BackgroundPainter` in `src/ui/background.rs` (currently a placeholder).
- Off-thread blur pipeline: decode → resize to 256×256 → blur → deliver to UI thread via a shared slot.
- Full-window blur texture render + dark vertical gradient overlay, painted before any widgets each frame.
- Remove the existing lines in `src/ui/player.rs` that set `panel_fill` from `background_color`; instead let `BackgroundPainter::paint` handle the background.
- Wire `BackgroundPainter` into `GurdoApp` in `src/ui/player.rs`.
- Solid fallback to `UiConfig.background_color` when no blur texture is present.

### Out of Scope
- Changes to any file other than `src/ui/background.rs` and `src/ui/player.rs`.
- New Cargo.toml dependencies (the existing `image` crate is sufficient).
- Animated transitions between backgrounds.
- Dominant-color extraction (deleted in EP-1, not reintroduced).
- Changes to the album art thumbnail widget itself.
- Changes to `src/config.rs`, `src/ui/state.rs`, or `assets.rs`.

## 3. Files to Create / Modify

| Action   | Path                       | Description                                                      |
|----------|----------------------------|------------------------------------------------------------------|
| Modify   | `src/ui/background.rs`     | Implement `BackgroundPainter` — blur pipeline, texture upload, paint |
| Modify   | `src/ui/player.rs`         | Add `blur` field to `GurdoApp`, call update/paint, remove old panel_fill lines |

## 4. Acceptance Criteria

### AC-1 — Background displays blurred cover art when a track is playing
**Scenario:**
- Given the player is running and no track is playing (solid background visible).
- When I start playing a track that has cover art.
- Then the window background changes from the solid color to a visibly blurred, full-window version of that track's album art within a short moment of the art loading.

---

### AC-2 — Background reverts to solid fallback color when playback stops
**Scenario:**
- Given a track with cover art is playing and the blurred background is visible.
- When playback stops or the queue is cleared so no track is active.
- Then the background returns to the solid fallback color (the `background_color` value from config, default near-black `[27, 27, 27]`).

---

### AC-3 — Gradient overlay keeps text and controls legible
**Scenario:**
- Given a track with a very bright or predominantly white album cover is playing.
- When the blurred background is rendered.
- Then a dark gradient overlay (transparent at the top, darkened at the bottom) is visibly present over the blur, ensuring that track title, artist, and playback controls remain readable.

---

### AC-4 — Background updates when the track changes
**Scenario:**
- Given track A is playing and the background shows track A's blurred art.
- When the user skips to track B (which has different cover art).
- Then the background transitions to track B's blurred cover art after the new art loads.

---

### AC-5 — Blur is not recomputed on every frame for the same track
**Scenario:**
- Given a track is playing and its blurred background is already displayed.
- When many rendering frames pass without a track change.
- Then CPU usage remains stable — no repeated heavy image-processing work occurs per frame (verified by profiling or log inspection showing blur is computed once per URL change, not once per frame).

---

### AC-6 — Album art thumbnail still renders correctly
**Scenario:**
- Given a track with cover art is playing and the blur background is active.
- When I look at the album art widget in the player UI.
- Then the album art thumbnail in the player layout is displayed correctly, unchanged from its appearance before this feature was introduced.

---

### AC-7 — Playback controls remain functional
**Scenario:**
- Given a track is playing and the blurred background is active.
- When I click the play/pause, next, and previous buttons.
- Then each control responds correctly and the playback state changes as expected (no controls are obscured or non-interactive due to the background layer).

---

### AC-8 — Background remains stable during rapid track changes
**Scenario:**
- Given the player is running.
- When I skip through five different tracks in quick succession (faster than the blur pipeline can complete for each).
- Then the player does not crash, freeze, or display visual corruption; the background eventually settles on the blurred art of the last playing track (or solid fallback if playback stopped).

---

### AC-9 — No new Cargo.toml dependencies are introduced
**Scenario:**
- Given the repository state before this iteration.
- When the implementation is complete and the project builds successfully.
- Then `Cargo.toml` and `Cargo.lock` contain no crate additions compared to before the iteration (the `image` crate already present is reused).

---

### AC-10 — Build is clean with no new compiler warnings
**Scenario:**
- Given the implementation is complete.
- When `cargo build` is run targeting `src/ui/background.rs` and `src/ui/player.rs`.
- Then the build succeeds with zero new warnings attributable to those two files.

---

## 5. Edge Cases and Failure Modes

| Scenario | Expected Behavior |
|---|---|
| Album art bytes are present but corrupt or undecodable | Blur pipeline silently discards the result; background stays on the previous blur texture (or solid fallback if none). No panic or visible error in the UI. |
| Album art is very dark (near-black cover) | Gradient overlay still applies; the result may appear nearly uniform black — this is acceptable and does not break legibility. |
| Album art is very bright/white | Gradient overlay provides sufficient darkening at the bottom for legibility. The top portion may look washed out — that is acceptable for this iteration. |
| Rapid track changes (blur for old track arrives after new track is set) | The URL-comparison gate ensures a stale blur result (for an outdated URL) is discarded rather than displayed. |
| First frame after launch before any art has loaded | `panel_fill` is set to the solid fallback color; no texture is uploaded; no crash. |
| Track with no cover art URL or empty bytes | `BackgroundPainter` receives `None` for cover bytes; no blur is attempted; background remains solid fallback. |
| Off-thread blur panics internally | The spawned thread panic is contained; the shared result slot is never populated; the UI continues with the previous background. |
| Very large album art bytes | The image is decoded then immediately downscaled to 256×256 before blurring, capping memory and compute cost regardless of source resolution. |

## 6. Notes for Implementation

- **Async delivery slot:** Use `Arc<Mutex<Option<egui::ColorImage>>>`. The background thread writes `Some(img)` when done; the UI thread polls this in `update()`, takes the value (replacing with `None`), and uploads the texture. This pattern avoids channels and keeps the UI thread lock-hold time minimal.

- **URL comparison gate:** Store the URL string of the last successfully *dispatched* blur job (not the last completed one) as a `String` field on `BackgroundPainter`. Before spawning a new thread, compare the incoming URL to this stored value. Only spawn if they differ. This prevents re-dispatching on every frame.

- **Texture upload must happen on the UI thread:** `egui::Context::load_texture` is only safe to call from the UI thread. The off-thread work produces `egui::ColorImage`; the thread must not call `load_texture`. The UI thread polls the `Arc<Mutex<...>>` slot and uploads.

- **Gradient overlay:** Paint two mesh triangles (or use `egui::Painter::rect_filled` with a vertically interpolated gradient) covering the full viewport rect. Top color: `(0, 0, 0, 60)` alpha; bottom color: `(0, 0, 0, 200)` alpha. This is drawn *after* the blur texture fill but *before* any widget layer.

- **Transparent panel fill:** When a blur texture is available, set `visuals.panel_fill = egui::Color32::TRANSPARENT` so the `CentralPanel` does not paint over the background. When no texture is available, set `panel_fill` to the solid fallback color as before.

- **Clearing on no-track:** When `cover_bytes` is `None`, set the stored `TextureHandle` to `None` and reset the last-blurred URL to an empty string so the next track with art triggers a fresh blur.

- **Image processing chain:** `image::load_from_memory(bytes)` → `.resize(256, 256, Lanczos3)` → convert to `RgbaImage` → `imageops::fast_blur(&mut rgba, 30.0)` → construct `egui::ColorImage` from the raw pixel bytes. The `image` crate already in `Cargo.toml` provides all of these.

- **Paint ordering:** In `GurdoApp::update()`, the sequence must be: (1) update album texture, (2) call `self.blur.update(...)`, (3) call `self.blur.paint(...)`, (4) then open `egui::CentralPanel`. Steps (3) and (4) use `ctx.layer_painter` or the painter obtained from a full-screen transparent panel to draw below the widget tree.
