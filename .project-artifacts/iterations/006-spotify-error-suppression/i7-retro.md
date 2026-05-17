# Iteration 6 Retrospective — Spotify API error suppression + status indicator (EP-17)

*Epic: EP-17 · Phase: Retrospective · Date: 2026-05-12*

---

## Action items

**Process note: status indicators belong in pre-allocated layout slots**
- Adding a new label between existing widgets caused layout shift. Prefer repurposing
  an existing slot (the time label) over inserting a new widget that displaces controls.
  Apply this pattern to any future inline status text in the player: find a slot whose
  content is contextually irrelevant during the status condition and substitute there.

No backlog changes from this retro.

---

## Backlog snapshot (post-iteration 6)

| # | Name | Type | Priority | Status |
|---|---|---|---|---|
| EP-1 | UI module split | REFACTOR | P1 | DONE |
| EP-2 | CLI removal & entry-point collapse | MIGRATION | P1 | ready (after EP-7) |
| EP-3 | Embedded assets + CJK font fix | FIX | P1 | DONE |
| EP-4 | Cover-blur background painter | FEATURE | P1 | DONE |
| EP-5 | Idle-state placeholder cover | FEATURE | P1 | DONE |
| EP-6 | Settings viewport window | FEATURE | P1 | DONE |
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
| EP-17 | Spotify API error suppression + status indicator | FIX | P1 | **DONE** |

**Remaining P1:** EP-7, EP-8, EP-2 (3 epics to MVP close)

---

## Proposed next epic

**EP-7 — In-process operations + progress** (P1, L, depends on EP-1 ✓, EP-6 ✓)

Rationale: EP-7 is the next unblocked P1 and the largest remaining epic. It fills the
Data and Spotify sections of the Settings window with live operations (Sync Last.fm,
Expand, Fetch Tracks, Score, Login) running in-process with real progress reporting.
Completing EP-7 also unblocks EP-2 (CLI removal) and EP-9 (Update everything).
