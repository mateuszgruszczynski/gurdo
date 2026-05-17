# Iteration 9 Retrospective — CLI removal & entry-point collapse (EP-2)

*Epic: EP-2 · Phase: Retrospective · Date: 2026-05-12*

---

## Action items

No surprises. EP-2 was straightforward: delete clap, shrink main.rs, done.

One minor note: `parse_config_arg` silently ignores unknown flags. If EP-11 (secrets
hardening) introduces a first-launch wizard, consider whether to print a usage hint for
unrecognised arguments at that point.

No backlog changes from this retro.

---

## 🎉 MVP close

All 8 P1 epics are now DONE:

EP-1 UI module split ✓  
EP-3 Embedded assets + CJK font fix ✓  
EP-4 Cover-blur background painter ✓  
EP-5 Idle-state placeholder cover ✓  
EP-6 Settings viewport window ✓  
EP-7 In-process operations + progress ✓  
EP-8 Full config-knob exposure ✓  
EP-17 Spotify API error suppression ✓  
EP-2 CLI removal & entry-point collapse ✓  

---

## Backlog snapshot (post-iteration 9)

| # | Name | Type | Priority | Status |
|---|---|---|---|---|
| EP-1 | UI module split | REFACTOR | P1 | DONE |
| EP-2 | CLI removal & entry-point collapse | MIGRATION | P1 | **DONE** |
| EP-3 | Embedded assets + CJK font fix | FIX | P1 | DONE |
| EP-4 | Cover-blur background painter | FEATURE | P1 | DONE |
| EP-5 | Idle-state placeholder cover | FEATURE | P1 | DONE |
| EP-6 | Settings viewport window | FEATURE | P1 | DONE |
| EP-7 | In-process operations + progress | FEATURE | P1 | DONE |
| EP-8 | Full config-knob exposure | FEATURE | P1 | DONE |
| EP-9 | Combined "Update everything" action | FEATURE | P2 | ready |
| EP-10 | Recommendation preview-while-tuning | FEATURE | P2 | ready |
| EP-11 | Secrets hardening & multi-user config | SECURITY | P2 | ready |
| EP-12 | Test scaffolding | QA | P2 | ready |
| EP-13 | Schema cleanup (similar_tracks drop) | TECH_DEBT | P3 | ready |
| EP-14 | Installer packaging | INFRA | P3 | ready |
| EP-15 | Traditional Chinese font (on demand) | REFINEMENT | P3 | parked |
| EP-16 | Dead-code cleanup (orphaned API surface) | TECH_DEBT | P3 | ready |
| EP-17 | Spotify API error suppression + status indicator | FIX | P1 | DONE |

**Remaining:** P2 (EP-9, 10, 11, 12) and P3 (EP-13, 14, 16) — post-MVP improvements.

---

## Proposed next epic

**EP-9 — Combined "Update everything" action** (P2, S) or **EP-12 — Test scaffolding** (P2, M).

EP-9 is a thin wrapper around EP-7's dispatcher; quick win.
EP-12 establishes a test harness that benefits all subsequent epics.

Both are P2; no strict ordering between them.
