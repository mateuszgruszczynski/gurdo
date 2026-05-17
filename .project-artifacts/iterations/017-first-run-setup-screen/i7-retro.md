# i7-retro: EP-20 — First-run Setup Screen + User-Scoped Config/Secrets

**Iteration:** 017 · **Date:** 2026-05-17

---

## Action items

1. **No new epics.** Nothing surfaced during this iteration that requires a backlog addition.

2. **Note for future credential-path changes:** When a `Config` method returns a fixed path (not derived from the config_path arg), the existing unit tests that write secrets to a temp dir must use an injectable overload (`load_with_secrets_at`). Pattern: add `#[cfg(test)] fn load_with_secrets_at` alongside `fn load`. Applied this iteration; follow the same pattern next time.

3. **OAuth with `-c` flag (known limitation, no action needed):** During Phase 2 of setup, the config loaded for `run_oauth_flow` is always read from `~/.gurdo/config.toml`, not from a custom `-c` path. The Spotify token is therefore written to `~/.gurdo/`'s `data_dir`. For a `-c /custom/path.toml` user whose custom config has a different `data_dir`, the token may end up in the wrong place. This is acceptable for the initial implementation (first-run users will overwhelmingly use the default path). No follow-up epic added; document in a code comment if it causes a support issue.

---

## Backlog delta

No epics added, removed, or re-prioritised.

**EP-20** → DONE.

All P1 and P2 epics are now DONE. EP-15 (Traditional Chinese font) remains parked (on-demand only).

---

## Updated backlog snapshot

| # | Name | Type | Priority | Status |
|---|---|---|---|---|
| EP-1 | UI module split | REFACTOR | P1 | DONE |
| EP-2 | CLI removal & entry-point collapse | MIGRATION | P1 | DONE |
| EP-3 | Embedded assets + CJK font fix | FIX | P1 | DONE |
| EP-4 | Cover-blur background painter | FEATURE | P1 | DONE |
| EP-5 | Idle-state placeholder cover | FEATURE | P1 | DONE |
| EP-6 | Settings viewport window | FEATURE | P1 | DONE |
| EP-7 | In-process operations + progress | FEATURE | P1 | DONE |
| EP-8 | Full config-knob exposure | FEATURE | P1 | DONE |
| EP-9 | Combined "Update everything" action | FEATURE | P2 | DONE |
| EP-10 | Recommendation preview-while-tuning | FEATURE | P2 | DONE |
| EP-11 | Secrets hardening & multi-user config | SECURITY | P2 | DONE |
| EP-12 | Test scaffolding | QA | P2 | DONE |
| EP-13 | Schema cleanup (similar_tracks drop) | TECH_DEBT | P3 | DONE |
| EP-14 | Installer packaging | INFRA | P3 | DONE |
| EP-15 | Traditional Chinese font (on demand) | REFINEMENT | P3 | parked |
| EP-16 | Dead-code cleanup (orphaned API surface) | TECH_DEBT | P3 | DONE |
| EP-17 | Spotify API error suppression + status indicator | FIX | P1 | DONE |
| EP-18 | Remove recommendation preview + plain-English settings descriptions | REFINEMENT | P2 | DONE |
| EP-19 | Replace behavioral knobs with contextual level selectors | REFINEMENT | P2 | DONE |
| EP-20 | First-run setup screen + user-scoped config/secrets | FEATURE | P2 | DONE |

**Next epic:** None — backlog exhausted (only EP-15 parked). **Pipeline complete.**
