# Iteration 5 Retrospective — Settings viewport window (EP-6)

*Epic: EP-6 · Phase: Retrospective · Date: 2026-05-12*

---

## Action items

**1. Process note: `request_repaint_of(ROOT)` required when settings actions affect player viewport**
- **Observation:** The in-window Close button had a ~1s dismiss delay. Root cause: the
  player viewport runs on a 1s repaint timer; setting `settings_open = false` from inside
  the settings viewport callback is not seen until the next player repaint. Fix:
  `ctx.request_repaint_of(egui::ViewportId::ROOT)` immediately after any store.
- **Rule for future epics:** Any action in the settings viewport that must immediately
  change something *visible in the player window* (close, status update visible in player,
  etc.) must call `ctx.request_repaint_of(ViewportId::ROOT)`. Actions self-contained in
  the settings window (knob edits, progress bars, operation buttons) do not need it.

**2. Backlog addition: EP-17 — Spotify API error suppression + status indicator (P1)**
- Observed during integration: sustained Spotify API downtime causes a modal flood
  (one blocking error modal every 5s poll cycle). Added EP-17 to backlog as P1.
- EP-17 is size S and has no dependencies — can be tackled any time before MVP close.

---

## Backlog snapshot (post-iteration 5)

| # | Name | Type | Priority | Status |
|---|---|---|---|---|
| EP-1 | UI module split | REFACTOR | P1 | DONE |
| EP-2 | CLI removal & entry-point collapse | MIGRATION | P1 | ready (after EP-7) |
| EP-3 | Embedded assets + CJK font fix | FIX | P1 | DONE |
| EP-4 | Cover-blur background painter | FEATURE | P1 | DONE |
| EP-5 | Idle-state placeholder cover | FEATURE | P1 | DONE |
| EP-6 | Settings viewport window | FEATURE | P1 | **DONE** |
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
| EP-17 | Spotify API error suppression + status indicator | FIX | P1 | ready |

**Remaining P1:** EP-17, EP-7, EP-8, EP-2 (4 epics to MVP close; EP-17 is small and can interleave)

---

## Proposed next epic

**EP-17 — Spotify API error suppression + status indicator** (P1, S, no deps)

Rationale: EP-17 is the smallest remaining P1 and was directly observed as a pain point
during this iteration's integration. Tackling it before EP-7 (the largest remaining epic)
means the UI is in a stable, non-spammy state before the in-process operations work begins.
