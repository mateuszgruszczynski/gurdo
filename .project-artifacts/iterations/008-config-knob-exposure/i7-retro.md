# Iteration 8 Retrospective — Full config-knob exposure (EP-8)

*Epic: EP-8 · Phase: Retrospective · Date: 2026-05-12*

---

## Action items

**Design note: dead import cleanup when refactoring function call sites**
- Moving `ops::token_exists` from `player.rs` into `settings.rs` left `use super::ops` in
  `player.rs` as an unused import (+1 warning). When a function call moves to a different
  file, scan all callers of the old import for stale `use` declarations.

**Design note: knob metadata module vs. inline**
- Defined `KnobSpec` statics in `knobs.rs` but used inline strings in `settings.rs`. The
  statics required `#[allow(dead_code)]`. For EP-10 (recommendation preview), if the statics
  are actually used then remove the allow; otherwise delete them and keep inline only.

No backlog changes from this retro.

---

## Backlog snapshot (post-iteration 8)

| # | Name | Type | Priority | Status |
|---|---|---|---|---|
| EP-1 | UI module split | REFACTOR | P1 | DONE |
| EP-2 | CLI removal & entry-point collapse | MIGRATION | P1 | ready |
| EP-3 | Embedded assets + CJK font fix | FIX | P1 | DONE |
| EP-4 | Cover-blur background painter | FEATURE | P1 | DONE |
| EP-5 | Idle-state placeholder cover | FEATURE | P1 | DONE |
| EP-6 | Settings viewport window | FEATURE | P1 | DONE |
| EP-7 | In-process operations + progress | FEATURE | P1 | DONE |
| EP-8 | Full config-knob exposure | FEATURE | P1 | **DONE** |
| EP-9 | Combined "Update everything" action | FEATURE | P2 | ready |
| EP-10 | Recommendation preview-while-tuning | FEATURE | P2 | ready |
| EP-11 | Secrets hardening & multi-user config | SECURITY | P2 | ready |
| EP-12 | Test scaffolding | QA | P2 | ready |
| EP-13 | Schema cleanup (similar_tracks drop) | TECH_DEBT | P3 | ready |
| EP-14 | Installer packaging | INFRA | P3 | ready |
| EP-15 | Traditional Chinese font (on demand) | REFINEMENT | P3 | parked |
| EP-16 | Dead-code cleanup (orphaned API surface) | TECH_DEBT | P3 | ready |
| EP-17 | Spotify API error suppression + status indicator | FIX | P1 | DONE |

**Remaining P1:** EP-2 only — the last epic before MVP close.

---

## Proposed next epic

**EP-2 — CLI removal & entry-point collapse** (P1, S, unblocked now that EP-7 ✓ and EP-8 ✓)

Rationale: EP-2 is the last P1 epic. Completing it closes the MVP milestone defined in Vision
§8. `clap` is removed from Cargo.toml; `main.rs` becomes ~30 lines launching directly into
the UI with an optional `-c <path>` config flag.
