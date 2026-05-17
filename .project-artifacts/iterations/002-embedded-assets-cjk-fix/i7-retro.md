# Iteration 2 Retrospective — Embedded assets + CJK font fix (EP-3)

*Epic: EP-3 · Phase: Retrospective · Date: 2026-05-12*

---

## Action items

**1. Note: Noto CJK font download source has moved**
- **Observation:** The spec referenced `notofonts/noto-fonts` monorepo for per-language OTF files. Those files are no longer present there. The authoritative per-language OTF packages are now in `notofonts/noto-cjk` release `Sans2.004` (assets `16_NotoSansJP.zip`, `17_NotoSansKR.zip`, `18_NotoSansSC.zip`).
- **Action:** Process note only — no backlog change. If EP-15 (Traditional Chinese) is ever picked up, it should pull `NotoSansTC` from the same `noto-cjk` release rather than the monorepo.

**2. Note: Pillow not installed in dev container**
- **Observation:** The spec suggested a Pillow one-liner to generate the placeholder PNG. Pillow is not installed. The workaround (Python stdlib `zlib`/`struct` PNG writer) produced a fully valid 400×400 PNG at 1 KB with no additional dependencies.
- **Action:** None. The stdlib approach is simpler and reproducible with no installed packages. If other epics need image generation, the same pattern applies.

**3. Note: CLI invocation order**
- **Observation:** Smoke test failed on first attempt with `cargo run -- ui -c config.toml`. The `-c` flag belongs to the top-level `Cli` struct and must precede the subcommand: `cargo run -- -c config.toml ui`. This is the existing CLI before EP-2 lands.
- **Action:** None — EP-2 removes clap entirely. Until then, the correct invocation is `cargo run -- -c config.toml ui` (or `gurdo -c config.toml ui`). No backlog change needed.

No backlog changes from this retro.

---

## Backlog snapshot (post-iteration 2)

| # | Name | Type | Priority | Status |
|---|---|---|---|---|
| EP-1 | UI module split | REFACTOR | P1 | **DONE** |
| EP-2 | CLI removal & entry-point collapse | MIGRATION | P1 | ready (after EP-7) |
| EP-3 | Embedded assets + CJK font fix | FIX | P1 | **DONE** |
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

**Remaining P1:** EP-4, EP-5, EP-6, EP-7, EP-8, EP-2 (6 epics to MVP close)

---

## Proposed next epic

**EP-4 — Cover-blur background painter** (P1, M, depends on EP-1 ✓)

Rationale: EP-3 is done, unblocking EP-5. The recommended sequence (from f3-backlog.md §Sequencing) puts EP-4 before EP-5 — EP-4 implements the blur pipeline and EP-5 uses the assets EP-3 just embedded. Doing EP-4 next keeps work flowing in dependency order and delivers another visible quality improvement.
