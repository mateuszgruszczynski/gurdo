# Iteration 3 Verification — Cover-blur background painter (EP-4)

*Epic: EP-4 · Phase: Verification · Date: 2026-05-12*

---

## Environment

Native egui/eframe desktop app — manual smoke test on host machine with live Spotify session.
Launch: `cargo run -- -c config.toml ui`

Two bugs surfaced and fixed during host smoke test before sign-off:

| Bug | Root cause | Fix |
|---|---|---|
| Panic: `assertion left==right failed (262144 vs 195584)` | `image::resize` preserves aspect ratio — non-square source produced e.g. 256×191 pixels but `ColorImage::from_rgba_unmultiplied` was called with hardcoded `[256,256]` | `resize` → `resize_exact(256, 256, Triangle)` |
| ~500ms–1s delay before background updates | Blur result sat in `pending` slot until next `request_repaint_after(1s)` tick | Pass `ctx.clone()` to blur thread; call `ctx.request_repaint()` after writing result |

---

## Test results

### In-process scenarios (from Development)

| ID | Scenario | AC | Result |
|---|---|---|---|
| T-01 | `panel_fill` transparent when blur present; old lines gone | AC-1 | PASS |
| T-02 | `panel_fill` set to fallback when no texture | AC-2 | PASS |
| T-03 | URL gate prevents per-frame re-dispatch | AC-5 | PASS |
| T-04 | No new Cargo.toml dependencies | AC-9 | PASS |
| T-05 | Clean build, no new warnings | AC-10 | PASS |

### E2E/UI scenarios — manual on host

| ID | Scenario | AC | Result |
|---|---|---|---|
| T-06 | Blurred background appears when track with art plays | AC-1 | PASS |
| T-07 | Solid fallback when playback stops | AC-2 | PASS |
| T-08 | Gradient overlay keeps text/controls legible over bright art | AC-3 | PASS |
| T-09 | Background updates on track change | AC-4 | PASS |
| T-10 | Album art thumbnail renders correctly | AC-6 | PASS |
| T-11 | Playback controls respond correctly while blur active | AC-7 | PASS |
| T-12 | No crash during rapid track skipping | AC-8 | PASS |

---

## Quarantined items

None.

---

## AC coverage table

| AC | Result |
|---|---|
| AC-1 | PASS |
| AC-2 | PASS |
| AC-3 | PASS |
| AC-4 | PASS |
| AC-5 | PASS |
| AC-6 | PASS |
| AC-7 | PASS |
| AC-8 | PASS |
| AC-9 | PASS |
| AC-10 | PASS |
