# Iteration 3 Test Plan — Cover-blur background painter (EP-4)

*Epic: EP-4 · Phase: Test Plan · Date: 2026-05-12*

---

## Scenario summary

| ID | Scenario | Covers AC | Level | Type | Owned by |
|---|---|---|---|---|---|
| T-01 | `panel_fill` is transparent when blur texture is present | AC-1 | Component | File-batch | Development |
| T-02 | `panel_fill` falls back to solid color when no texture | AC-2 | Component | File-batch | Development |
| T-03 | URL gate prevents re-dispatching blur for the same track | AC-5 | Component | File-batch | Development |
| T-04 | Cargo.toml has no new crate dependencies | AC-9 | Component | File-batch | Development |
| T-05 | Clean build — no new warnings in background.rs or player.rs | AC-10 | Component | CLI | Development |
| T-06 | Blurred background appears when a track with art is playing | AC-1 | E2E | UI | Verification |
| T-07 | Background reverts to solid color when playback stops | AC-2 | E2E | UI | Verification |
| T-08 | Gradient overlay keeps text and controls legible over bright art | AC-3 | E2E | UI | Verification |
| T-09 | Background updates when track changes | AC-4 | E2E | UI | Verification |
| T-10 | Album art thumbnail renders correctly — regression | AC-6 | E2E | UI | Verification |
| T-11 | Playback controls respond correctly while blur is active — regression | AC-7 | E2E | UI | Verification |
| T-12 | No crash during rapid track skipping | AC-8 | E2E | UI | Verification |

---

## Scenarios

---

### T-01 — `panel_fill` is transparent when blur texture is present

**Covers AC:** AC-1  
**Level:** Component  
**Type:** File-batch  
**Owned by:** Development

```
Given the implementation is complete
When I read src/ui/background.rs and src/ui/player.rs
Then BackgroundPainter.paint sets panel_fill to transparent when a blur texture handle is held
And the old lines that set panel_fill directly from background_color are absent from player.rs
```

**Verification command:** `grep "panel_fill" src/ui/player.rs` — must return no output (responsibility moved to `BackgroundPainter::paint`).

---

### T-02 — `panel_fill` falls back to solid color when no texture

**Covers AC:** AC-2  
**Level:** Component  
**Type:** File-batch  
**Owned by:** Development

```
Given the implementation is complete
When I read src/ui/background.rs
Then BackgroundPainter.paint sets panel_fill to the fallback RGB color when no TextureHandle is present
And no texture drawing is attempted in that branch
```

---

### T-03 — URL gate prevents re-dispatching blur for the same track

**Covers AC:** AC-5  
**Level:** Component  
**Type:** File-batch  
**Owned by:** Development

```
Given the implementation is complete
When I read BackgroundPainter.update in src/ui/background.rs
Then the function compares the incoming cover URL to a stored last-blurred URL before spawning a thread
And a new thread is only spawned when the URL is different from the stored value
```

**Notes:** Code review — the URL comparison must gate thread spawning, not just texture upload. This ensures per-frame calls to `update()` with the same URL do not spawn redundant threads.

---

### T-04 — Cargo.toml has no new crate dependencies

**Covers AC:** AC-9  
**Level:** Component  
**Type:** File-batch  
**Owned by:** Development

```
Given the implementation is complete
When I compare Cargo.toml to the pre-iteration baseline
Then no new entries appear in [dependencies]
And cargo build uses only the pre-existing image crate for blur and resize operations
```

**Verification command:** `git diff HEAD~1 Cargo.toml` shows no added dependency lines.

---

### T-05 — Clean build — no new warnings in background.rs or player.rs

**Covers AC:** AC-10  
**Level:** Component  
**Type:** CLI  
**Owned by:** Development

```
Given the implementation is complete
When I run cargo build
Then the build exits with status 0
And no warning lines reference background.rs or player.rs
```

**Verification command:** `cargo build 2>&1 | grep "^warning:.*background\|^warning:.*player"` — must return no output.

---

### T-06 — Blurred background appears when a track with art is playing

**Covers AC:** AC-1  
**Level:** E2E  
**Type:** UI  
**Owned by:** Verification

```
Given the app is launched on the host and no track is playing (solid dark background visible)
When I start playing a track that has album cover art
Then the window background changes from the solid color to a visibly blurred, full-window image of the cover art
And the blur is clearly recognisable as a heavily defocused version of the art (not sharp, not a solid wash)
```

---

### T-07 — Background reverts to solid color when playback stops

**Covers AC:** AC-2  
**Level:** E2E  
**Type:** UI  
**Owned by:** Verification

```
Given a track with cover art is playing and the blurred background is visible
When playback stops and no track is active
Then the background returns to the solid near-black color (the configured background_color)
And no blurred image remnant is visible
```

---

### T-08 — Gradient overlay keeps text and controls legible over bright art

**Covers AC:** AC-3  
**Level:** E2E  
**Type:** UI  
**Owned by:** Verification

```
Given a track with a very bright or predominantly white album cover is playing
When the blurred background is rendered
Then a dark gradient overlay is visibly present — darker at the bottom, lighter at the top
And the track title, artist name, and all playback buttons are clearly readable against the background
```

**Notes:** Test with a known bright-cover track (e.g. an album with white/yellow artwork). If no bright-cover track is available at test time, any track with art suffices for the presence-of-gradient check.

---

### T-09 — Background updates when track changes

**Covers AC:** AC-4  
**Level:** E2E  
**Type:** UI  
**Owned by:** Verification

```
Given track A is playing with its blurred art as the background
When the user skips to track B (different cover art)
Then the background transitions to track B's blurred cover art after the new art loads
And the previous track's art is no longer visible in the background
```

---

### T-10 — Album art thumbnail renders correctly — regression

**Covers AC:** AC-6  
**Level:** E2E  
**Type:** UI  
**Owned by:** Verification

```
Given a track with cover art is playing and the blur background is active
When I look at the album art widget in the player layout
Then the 400×400 rounded album art thumbnail is displayed correctly
And it appears at the same position, size, and rounding as before this feature was introduced
```

---

### T-11 — Playback controls respond correctly while blur is active — regression

**Covers AC:** AC-7  
**Level:** E2E  
**Type:** UI  
**Owned by:** Verification

```
Given a track is playing and the blurred background is active
When I click the previous, seek-back, play/pause, seek-forward, and next buttons in turn
Then each button responds correctly — playback state changes as expected
And no button is obscured, unresponsive, or visually broken by the background layer
```

---

### T-12 — No crash during rapid track skipping

**Covers AC:** AC-8  
**Level:** E2E  
**Type:** UI  
**Owned by:** Verification

```
Given the player is running
When I skip through five different tracks in quick succession (clicking next repeatedly before the blur has time to update)
Then the player does not crash, freeze, or display visual corruption
And the background eventually shows the blurred art of the last active track (or solid fallback if playback ended)
```

---

## Coverage table

| AC | Covered by | Level | Phase |
|---|---|---|---|
| AC-1 | T-01, T-06 | Component + E2E | Dev + Verification |
| AC-2 | T-02, T-07 | Component + E2E | Dev + Verification |
| AC-3 | T-08 | E2E | Verification |
| AC-4 | T-09 | E2E | Verification |
| AC-5 | T-03 | Component | Development |
| AC-6 | T-10 | E2E | Verification |
| AC-7 | T-11 | E2E | Verification |
| AC-8 | T-12 | E2E | Verification |
| AC-9 | T-04 | Component | Development |
| AC-10 | T-05 | Component | Development |

All 10 ACs covered. AC-3, AC-4, AC-6, AC-7, AC-8 are E2E/UI only — they have no in-process observable (glyph rendering and interactive behavior require a live display host).

---

## Notes on Verification automation

All E2E scenarios (T-06 through T-12) are manual. The egui/eframe player requires a GPU and display context; it cannot run headlessly inside the dev container. Verification is a manual smoke test on the host machine with a live Spotify session.
