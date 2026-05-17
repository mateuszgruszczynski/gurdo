# Iteration 1 Retrospective — UI module split (EP-1)

*Epic: EP-1 · Phase: Retrospective · Date: 2026-05-12*

---

## Action items

**1. Add EP-16 — Dead-code cleanup (orphaned API surface)**
- **Why:** `cargo build` emits 52 pre-existing dead-code warnings from unused Last.fm / Spotify / db functions that were called by CLI subcommands. EP-2 will remove the CLI but not the dead model/client code. EP-13 covers `similar_tracks`; the rest is not covered.
- **Action:** EP-16 added to `f3-backlog.md` as P3, TECH_DEBT, depends on EP-2. Sequenced after EP-12 (to avoid conflicts with test fixtures).

**2. AC wording note for future iterations**
- AC-2 of i1-spec.md said "zero warnings". For existing codebases with pre-existing warnings, future Refinement phases should phrase build-cleanliness ACs as "no new warnings introduced relative to the pre-refactor baseline". No backlog change — this is a process note for the next Refinement phase.

---

## Backlog snapshot (post-iteration 1)

| # | Name | Type | Priority | Status |
|---|---|---|---|---|
| EP-1 | UI module split | REFACTOR | P1 | **DONE** |
| EP-2 | CLI removal & entry-point collapse | MIGRATION | P1 | ready |
| EP-3 | Embedded assets + CJK font fix | FIX | P1 | ready |
| EP-4 | Cover-blur background painter | FEATURE | P1 | ready |
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

**Remaining P1:** EP-2, EP-3, EP-4, EP-5, EP-6, EP-7, EP-8 (7 epics to MVP close)

---

## Proposed next epic

**EP-3 — Embedded assets + CJK font fix** (P1, S, no dependencies)

Rationale: EP-3 has no dependencies on anything (EP-1 is done), fixes a current user-facing bug (CJK tofu), and is small. EP-4 and EP-5 both depend on EP-3's assets, so landing it now unblocks two downstream epics.
