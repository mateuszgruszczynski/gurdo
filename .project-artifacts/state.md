# Pipeline State

mode: improve
status: COMPLETE
current_epic:
current_phase: Idle
iteration: 17
Foundation phases: Analysis | Vision | Architecture | Backlog | Environment

## Completed phases
- Analysis ✓ → .project-artifacts/ana-analysis.md
- Vision ✓ → .project-artifacts/f1-vision.md
- Architecture ✓ → .project-artifacts/f2-architecture.md
- Backlog ✓ → .project-artifacts/f3-backlog.md
- Environment ✓ → .devcontainer/ (Dockerfile, devcontainer.json, .claude/settings.json)

## Current iteration
Idle

## Backlog

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

## Completed iterations

| # | Epic | Status | Date | Highlight | Artifacts |
|---|---|---|---|---|---|
| 001 | UI module split | DONE | 2026-05-12 | EP-16 added (dead-code cleanup) | iterations/001-ui-module-split/i7-retro.md |
| 002 | Embedded assets + CJK font fix | DONE | 2026-05-12 | Font source: noto-cjk Sans2.004; binary +17 MB | iterations/002-embedded-assets-cjk-fix/i7-retro.md |
| 003 | Cover-blur background painter | DONE | 2026-05-12 | resize_exact + ctx.request_repaint() lessons | iterations/003-cover-blur-background/i7-retro.md |
| 004 | Idle-state placeholder cover | DONE | 2026-05-12 | Lazy-init in update() preferred over CreationContext for static UI textures | iterations/004-idle-state-placeholder-cover/i7-retro.md |
| 005 | Settings viewport window | DONE | 2026-05-12 | request_repaint_of(ROOT) needed for cross-viewport immediate close; EP-17 added | iterations/005-settings-viewport-window/i7-retro.md |
| 006 | Spotify API error suppression + status indicator | DONE | 2026-05-12 | Warning in time-label slot avoids layout shift | iterations/006-spotify-error-suppression/i7-retro.md |
| 007 | In-process operations + progress | DONE | 2026-05-12 | Shared ProgressReporter trait in src/progress.rs avoids circular imports | iterations/007-in-process-operations/i7-retro.md |
| 008 | Full config-knob exposure | DONE | 2026-05-12 | Draft via Arc<Mutex<Option<Config>>>; knob inline over static slices | iterations/008-config-knob-exposure/i7-retro.md |
| 009 | CLI removal & entry-point collapse | DONE | 2026-05-12 | MVP close — all P1 epics done; clap removed | iterations/009-cli-removal/i7-retro.md |
| 010 | Combined "Update everything" action | DONE | 2026-05-12 | Dispatcher match restructure; step field on ActiveOperation | iterations/010-update-everything/i7-retro.md |
| 011 | Recommendation preview-while-tuning | DONE | 2026-05-12 | settings_draft passed to dispatcher; replace_all indentation gotcha | iterations/011-recommendation-preview/i7-retro.md |
| 012 | Secrets hardening & multi-user config | DONE | 2026-05-13 | secrets.toml sibling overlay; tempfile added as dev-dep for tests | iterations/012-secrets-hardening/i7-retro.md |
| 013 | Test scaffolding | DONE | 2026-05-15 | upsert_artist_external preserves case; tests use lowercase fixtures | iterations/013-test-scaffolding/i7-retro.md |
| 014 | Schema cleanup (similar_tracks drop) | DONE | 2026-05-15 | warnings 53→47; migration in existing DROP TABLE batch | iterations/014-schema-cleanup-similar-tracks-drop/i7-retro.md |
| 015 | Dead-code cleanup (orphaned API surface) | DONE | 2026-05-15 | warnings 47→1; field mbid confusion — grep callers before removing fields | iterations/015-dead-code-cleanup/i7-retro.md |
| 016 | Installer packaging | DONE | 2026-05-15 | set -euo pipefail gives build-failure AC for free; tar -C for flat archive layout | iterations/016-installer-packaging/i7-retro.md |

## History
- 2026-05-11 — pipeline initialised in improve mode.
- 2026-05-11 — Analysis approved (CLI removal added as 4th scope item).
- 2026-05-11 — Vision approved (8 success criteria incl. placeholder + CJK fix).
- 2026-05-11 — Architecture approved (3-font fallback chain, centered settings viewport, one-at-a-time ops).
- 2026-05-11 — Backlog approved (15 epics; EP-2 sequenced after EP-7).
- 2026-05-12 — Iteration 1 closed: UI module split (EP-1). EP-16 added to backlog.
