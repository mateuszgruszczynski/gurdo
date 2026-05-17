# Iteration 12 Retrospective — Secrets hardening & multi-user config (EP-11)

*Epic: EP-11 · Phase: Retrospective · Date: 2026-05-13*

---

## What went well

Clean, minimal implementation. The overlay pattern keeps full backward compatibility:
existing users with secrets in `config.toml` continue working without any changes.

## Issues

Test `minimal_config_toml()` initially missing required `[sync]` and `[engine]` sections
— those structs have per-field `#[serde(default)]` but the section itself is not
defaulted at the `Config` struct level, so parsing fails without the section header.
Lesson: when writing fixture TOML for tests, include all non-`#[serde(default)]` sections.

## Action items

- Optionally: add `#[serde(default)]` to `sync: SyncConfig` and `engine: EngineConfig`
  at the top-level `Config` struct so partial configs work. Low priority — not a user-facing issue.
- Migration docs: a one-paragraph README section explaining `secrets.toml` would help
  new contributors. Belongs in EP-14 (installer/packaging) or a standalone docs epic.

No backlog changes.

---

## Backlog snapshot (post-iteration 12)

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
| EP-10 | Recommendation preview-while-tuning          | FEATURE   | P2 | DONE  |
| EP-11 | Secrets hardening & multi-user config        | SECURITY  | P2 | **DONE** |
| EP-12 | Test scaffolding                             | QA        | P2 | ready |
| EP-13 | Schema cleanup (similar_tracks drop)         | TECH_DEBT | P3 | ready |
| EP-14 | Installer packaging                          | INFRA     | P3 | ready |
| EP-15 | Traditional Chinese font (on demand)         | REFINEMENT| P3 | parked |
| EP-16 | Dead-code cleanup (orphaned API surface)     | TECH_DEBT | P3 | ready |
| EP-17 | Spotify API error suppression                | FIX       | P1 | DONE  |

## Proposed next epic

Last P2: **EP-12 Test scaffolding** — the only remaining P2 epic, and it directly
unblocks the deferred unit tests from EP-10 and EP-11.
