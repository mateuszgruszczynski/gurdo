# Iteration 4 Retrospective — Idle-state placeholder cover (EP-5)

*Epic: EP-5 · Phase: Retrospective · Date: 2026-05-12*

---

## Action items

No plan changes from this retro. EP-5 was a clean size-S delivery with no surprises.

**Process note: lazy-init preferred over `CreationContext` for UI-only assets**
- Decoding the placeholder in `update()` on the first frame (rather than in `mod.rs`'s
  `CreationContext` closure) keeps the decode logic co-located with `player.rs` and
  reuses the existing `decode_image` helper without duplicating imports. The one-frame
  latency is imperceptible. Apply this pattern to any future static UI texture that
  can be loaded at first render.

---

## Backlog snapshot (post-iteration 4)

| # | Name | Type | Priority | Status |
|---|---|---|---|---|
| EP-1 | UI module split | REFACTOR | P1 | DONE |
| EP-2 | CLI removal & entry-point collapse | MIGRATION | P1 | ready (after EP-7) |
| EP-3 | Embedded assets + CJK font fix | FIX | P1 | DONE |
| EP-4 | Cover-blur background painter | FEATURE | P1 | DONE |
| EP-5 | Idle-state placeholder cover | FEATURE | P1 | **DONE** |
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

**Remaining P1:** EP-6, EP-7, EP-8, EP-2 (4 epics to MVP close)

---

## Proposed next epic

**EP-6 — Settings viewport window** (P1, M, depends on EP-1 ✓)

Rationale: EP-6 is the next unblocked P1. It opens the Settings as a proper OS-level
deferred viewport (replacing the current modal), adds `[ui].player_window_size` /
`[ui].settings_window_size` config fields, and scaffolds the empty section placeholders
that EP-7 and EP-8 will fill. Unblocking EP-6 now clears the path for the two large
epics (EP-7/EP-8) that depend on it.
