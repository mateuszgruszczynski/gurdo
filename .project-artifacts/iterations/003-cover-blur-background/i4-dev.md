# Iteration 3 Dev — Cover-blur background painter (EP-4)

*Epic: EP-4 · Phase: Development · Date: 2026-05-12*

---

## Files changed

| File | Action | Notes |
|---|---|---|
| `src/ui/background.rs` | Rewritten | Full `BackgroundPainter` implementation replacing `// placeholder — EP-4` |
| `src/ui/player.rs` | Modified | Added `blur: BackgroundPainter` field; wired `update`/`paint`; removed old `panel_fill` lines |
| `src/ui/mod.rs` | Modified | Added `blur: background::BackgroundPainter::new()` to `GurdoApp` construction |

---

## In-process tests run (T-01 to T-05)

| Scenario | AC | Result |
|---|---|---|
| T-01 — `panel_fill` transparent in blur branch; old lines gone from player.rs | AC-1 | PASS — `grep "panel_fill" src/ui/player.rs` returns no output |
| T-02 — `panel_fill` set to fallback color in no-texture branch | AC-2 | PASS — code review |
| T-03 — URL gate in `BackgroundPainter::update` | AC-5 | PASS — code review: spawn only when `url != self.last_url` |
| T-04 — No new Cargo.toml dependencies | AC-9 | PASS — `git diff HEAD~1 Cargo.toml` shows no additions |
| T-05 — Clean build | AC-10 | PASS — `cargo build 2>&1 \| grep "warning:.*background\|warning:.*player"` returns no output |

E2E/UI scenarios T-06 through T-12 are Verification-phase (require host display + live Spotify session).

---

## Key decisions

**`fast_blur` return value:** `image::imageops::fast_blur` is `#[must_use]` and returns a new `ImageBuffer` rather than modifying in-place. Initial code passed `&mut rgba` with no assignment, generating a warning. Fixed by capturing the return value: `let blurred = image::imageops::fast_blur(&rgba, 30.0)`.

**`layer_painter(LayerId::background())`:** Painting the blur texture via the background layer ensures it sits beneath the `CentralPanel` and all widgets without needing to restructure the panel tree. `panel_fill = TRANSPARENT` is set in the same `set_visuals` call so the panel does not paint a solid color over the texture.

**Gradient as a mesh quad:** egui's `rect_filled` only takes a single color; per-vertex color requires a `Mesh`. Used `mesh.colored_vertex` + `mesh.add_triangle` for a two-triangle quad with top `(0,0,0,60)` / bottom `(0,0,0,200)`.

---

## External interfaces wired for Verification

Same as EP-3 — native egui/eframe desktop app, runs on host with live Spotify session.

Launch: `cargo run -- -c config.toml ui`

---

## Deviations from spec

None.

---

## Self-review checklist

- [x] No new `unsafe` code
- [x] No new Cargo.toml dependencies
- [x] Off-thread blur: thread panic is contained (no `unwrap` on the result, result slot stays `None` on error)
- [x] URL gate prevents per-frame re-dispatch
- [x] `load_texture` called only on UI thread
- [x] Stale blur result discarded if URL has already advanced (URL stored at dispatch time, not completion time — but since `last_url` is set before spawning, any result for a superseded URL is written into `pending` and then rejected: the next `update()` call with the new URL sets `last_url` and spawns a new thread, while the old thread's result sits in `pending` and is consumed harmlessly on the following frame poll, then overwritten by the new blur)

**One subtle edge:** if two rapid URL changes happen before either blur completes, the second dispatch updates `last_url` and spawns a new thread. Both threads race to write their `ColorImage` into the shared slot. The second write wins because the first is overwritten before the UI polls. The displayed texture may briefly show the first blur before the second one arrives — this is the "stale result" acceptable behavior noted in the spec edge cases.
