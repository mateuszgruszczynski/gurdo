# Iteration 3 Retrospective — Cover-blur background painter (EP-4)

*Epic: EP-4 · Phase: Retrospective · Date: 2026-05-12*

---

## Action items

**1. Spec/architecture note: `resize_exact` required for `ColorImage::from_rgba_unmultiplied`**
- **Observation:** `image::DynamicImage::resize` preserves aspect ratio, so non-square inputs produce outputs smaller than the declared size. Passing a hardcoded `[256, 256]` to `ColorImage::from_rgba_unmultiplied` with fewer actual pixels causes an assertion panic. Architecture §4.2 used the word "resize" without specifying exact vs. aspect-ratio-preserving.
- **Action:** Process note — any future epic that downscales images and feeds the result to `ColorImage::from_rgba_unmultiplied` must use `resize_exact` (or explicitly compute the actual output dimensions from `resize`). No backlog change needed.

**2. Spec/architecture note: off-thread work must trigger `ctx.request_repaint()`**
- **Observation:** Background work that writes to an `Arc<Mutex<...>>` slot and expects the UI to poll it on the next frame is only effective if a repaint is scheduled. With `request_repaint_after(1s)`, the result can wait up to 1 second. Architecture §4.2 described the delivery slot correctly but omitted the repaint trigger. This pattern will recur in EP-7 (in-process operations + progress).
- **Action:** Note for EP-7: any tokio task or thread that pushes state changes into a shared slot must call `ctx.request_repaint()` (or send via a channel that the egui integration polls) to ensure the UI responds promptly. No backlog change needed — this is a process note.

No backlog changes from this retro.

---

## Backlog snapshot (post-iteration 3)

| # | Name | Type | Priority | Status |
|---|---|---|---|---|
| EP-1 | UI module split | REFACTOR | P1 | DONE |
| EP-2 | CLI removal & entry-point collapse | MIGRATION | P1 | ready (after EP-7) |
| EP-3 | Embedded assets + CJK font fix | FIX | P1 | DONE |
| EP-4 | Cover-blur background painter | FEATURE | P1 | **DONE** |
| EP-5 | Idle-state placeholder cover | FEATURE | P1 | ready |
| EP-6 | Settings viewport window | FEATURE | P1 | ready |
| EP-7 | In-process operations + progress | FEATURE | P1 | ready |
| EP-8 | Full config-knob exposure | FEATURE | P1 | ready |
| EP-9 | Combined "Update everything" action | FEATURE | P2 | ready |
| EP-10 | Recommendation preview-while-tuning | FEATURE | P2 | ready |
| EP-11 | Secrets hardening & multi-user config | SECURITY | P2 | ready |
| EP-12 | Test scaffolding | QA | P2 | ready |
| EP-13 | Schema cleanup (similar_tracks drop) | TECH_DEBT | P3 | ready |
| EP-14 | Installer packaging | INFRA | P3 | ready |
| EP-15 | Traditional Chinese font (on demand) | REFINEMENT | P3 | parked |
| EP-16 | Dead-code cleanup (orphaned API surface) | TECH_DEBT | P3 | ready |

**Remaining P1:** EP-5, EP-6, EP-7, EP-8, EP-2 (5 epics to MVP close)

---

## Proposed next epic

**EP-5 — Idle-state placeholder cover** (P1, S, depends on EP-1 ✓, EP-3 ✓)

Rationale: EP-5 is the smallest remaining P1 (size S). Both its dependencies are now done. It directly uses `assets::PLACEHOLDER_COVER` embedded in EP-3 and displays in the same 400×400 slot refined in EP-4. Knocking it out now clears the idle-state visual gap before the larger EP-6/EP-7 work begins.
