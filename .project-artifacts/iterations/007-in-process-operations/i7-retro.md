# Iteration 7 Retrospective — In-process operations + progress (EP-7)

*Epic: EP-7 · Phase: Retrospective · Date: 2026-05-12*

---

## Action items

**Design note: shared trait module for cross-layer concerns**
- `ProgressReporter` cannot live in `src/ui/ops.rs` when sync/engine functions need it — that
  creates a circular import. The fix is a top-level `src/progress.rs` module. Apply this
  pattern for any future trait that must be shared between the UI layer and the sync/engine
  layer (e.g. a future `CancellationToken` or logging trait).

**Design note: `#[allow(dead_code)]` for specced-but-not-yet-called members**
- `ProgressReporter::message` and `ActiveOperation::message` are wired for future callers
  but currently unused. Suppress with `#[allow(dead_code)]` and a comment; do not remove
  because they are part of the deliberately-specced public API.

No backlog changes from this retro.

---

## Backlog snapshot (post-iteration 7)

| # | Name | Type | Priority | Status |
|---|---|---|---|---|
| EP-1 | UI module split | REFACTOR | P1 | DONE |
| EP-2 | CLI removal & entry-point collapse | MIGRATION | P1 | ready (after EP-7 ✓) |
| EP-3 | Embedded assets + CJK font fix | FIX | P1 | DONE |
| EP-4 | Cover-blur background painter | FEATURE | P1 | DONE |
| EP-5 | Idle-state placeholder cover | FEATURE | P1 | DONE |
| EP-6 | Settings viewport window | FEATURE | P1 | DONE |
| EP-7 | In-process operations + progress | FEATURE | P1 | **DONE** |
| EP-8 | Full config-knob exposure | FEATURE | P1 | ready |
| EP-9 | Combined "Update everything" action | FEATURE | P2 | ready |
| EP-10 | Recommendation preview-while-tuning | FEATURE | P2 | ready |
| EP-11 | Secrets hardening & multi-user config | SECURITY | P2 | ready |
| EP-12 | Test scaffolding | QA | P2 | ready |
| EP-13 | Schema cleanup (similar_tracks drop) | TECH_DEBT | P3 | ready |
| EP-14 | Installer packaging | INFRA | P3 | ready |
| EP-15 | Traditional Chinese font (on demand) | REFINEMENT | P3 | parked |
| EP-16 | Dead-code cleanup (orphaned API surface) | TECH_DEBT | P3 | ready |
| EP-17 | Spotify API error suppression + status indicator | FIX | P1 | DONE |

**Remaining P1:** EP-8, EP-2 (2 epics to MVP close)

---

## Proposed next epic

**EP-8 — Full config-knob exposure** (P1, unblocked)

Rationale: EP-8 fills the remaining Settings sections (Recommendations, Engine, Artist Scoring,
Sync, Appearance) with live config knobs, completing the Settings window. After EP-8 and EP-2,
the P1 backlog is fully done.
