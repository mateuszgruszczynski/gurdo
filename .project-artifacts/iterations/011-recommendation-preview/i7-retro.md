# Iteration 11 Retrospective — Recommendation preview-while-tuning (EP-10)

*Epic: EP-10 · Phase: Retrospective · Date: 2026-05-12*

---

## What went well

Straightforward feature addition. The existing Arc pattern for draft config extended
cleanly to the dispatcher, and the Preview branch is the simplest branch in the loop
(no active-op indicator, no step counter).

## Issues

One minor: `replace_all` on `for (artist, track)` only caught the first of two loops
in `poll.rs` because the two had different leading indentation (16 vs 8 spaces). Fixed
individually. Remember: `replace_all` matches byte-for-byte, so indentation differences
will silently miss other occurrences. Verify with grep after bulk replacements.

## Action items

- SC-1 (`weighted_sample` determinism) and SC-2 (`generate_recommendations` with SQLite
  fixture) remain unautomated. Land these in EP-12 test scaffolding.

No backlog changes.

---

## Backlog snapshot (post-iteration 11)

| # | Name | Type | Priority | Status |
|---|---|---|---|---|
| EP-1  | UI module split                              | REFACTOR  | P1 | DONE  |
| EP-2  | CLI removal & entry-point collapse           | MIGRATION | P1 | DONE  |
| EP-3  | Embedded assets + CJK font fix               | FIX       | P1 | DONE  |
| EP-4  | Cover-blur background painter                | FEATURE   | P1 | DONE  |
| EP-5  | Idle-state placeholder cover                 | FEATURE   | P1 | DONE  |
| EP-6  | Settings viewport window                     | FEATURE   | P1 | DONE  |
| EP-7  | In-process operations + progress             | FEATURE   | P1 | DONE  |
| EP-8  | Full config-knob exposure                    | FEATURE   | P1 | DONE  |
| EP-9  | Combined "Update everything" action          | FEATURE   | P2 | DONE  |
| EP-10 | Recommendation preview-while-tuning          | FEATURE   | P2 | **DONE** |
| EP-11 | Secrets hardening & multi-user config        | SECURITY  | P2 | ready |
| EP-12 | Test scaffolding                             | QA        | P2 | ready |
| EP-13 | Schema cleanup (similar_tracks drop)         | TECH_DEBT | P3 | ready |
| EP-14 | Installer packaging                          | INFRA     | P3 | ready |
| EP-15 | Traditional Chinese font (on demand)         | REFINEMENT| P3 | parked |
| EP-16 | Dead-code cleanup (orphaned API surface)     | TECH_DEBT | P3 | ready |
| EP-17 | Spotify API error suppression                | FIX       | P1 | DONE  |

## Proposed next epic

Two P2 epics remain: **EP-11** (Secrets hardening) and **EP-12** (Test scaffolding).
EP-12 directly unblocks deferred SC-1 and SC-2.
EP-11 is the security prerequisite for multi-user distribution.
