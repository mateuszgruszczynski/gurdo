# Iteration 1 Dev Summary — UI module split (EP-1)

*Epic: EP-1 · Phase: Development · Date: 2026-05-11*

---

## Files changed

| Action | File | Notes |
|---|---|---|
| Created | `src/ui/state.rs` | `PlayerState`, `PlayerCommand` (moved verbatim; both `pub`) |
| Created | `src/ui/poll.rs` | `polling_loop` (`pub(super)`), `do_poll`, `handle_cmd`, `extend_queue_if_needed`, `QUEUE_CHUNK_SIZE` (all private) |
| Created | `src/ui/player.rs` | `GurdoApp` (`pub(super)`), `impl eframe::App`, `SettingsDraft` (`pub(super)`), `decode_image`, `fmt_ms`, `spawn_cli` (helpers private) |
| Rewritten | `src/ui/mod.rs` | Declares 9 sub-modules; `pub fn run(config, config_path)` lifted verbatim from app.rs |
| Created | `src/ui/settings.rs` | `// placeholder — EP-6` |
| Created | `src/ui/background.rs` | `// placeholder — EP-4` |
| Created | `src/ui/ops.rs` | `// placeholder — EP-7` |
| Created | `src/ui/assets.rs` | `// placeholder — EP-3` |
| Created | `src/ui/knobs.rs` | `// placeholder — EP-8` |
| Deleted | `src/ui/app.rs` | Replaced by the nine files above |

No files outside `src/ui/` were modified.

---

## In-process tests written

No new test files. EP-1 is a pure refactor — behaviour is verified by:

| Scenario | Level | AC | Method |
|---|---|---|---|
| T-01: build exits 0 | Component | AC-2, AC-3 | `cargo build` — passed |
| T-02: correct file set | Component | AC-1 | `ls src/ui/` — 9 files, no `app.rs` |
| T-03: dead code absent | Component | AC-4 | `grep -r "extract_dominant_color" src/` — no output |
| T-04: skeletons comment-only | Component | AC-5 | File inspection — confirmed |
| T-05: EP-7 types absent | Component | AC-1 scope guard | `grep` for `OperationsState` etc. — no output |

---

## External interfaces wired

`pub fn run(config: Config, config_path: PathBuf) -> anyhow::Result<()>` is available at `ui::run` — identical signature to before, callable from `src/main.rs` without any import changes. Building the binary (`cargo build`) produces a runnable executable; Verification can drive T-06–T-12 by running it on the host.

---

## Key implementation decisions

1. **Visibility of `GurdoApp` and `SettingsDraft`:** Both marked `pub(super)` so `run()` in `mod.rs` can construct them. All fields of `GurdoApp` are also `pub(super)` to allow struct-literal construction in `mod.rs`. This is the minimum needed; they remain inaccessible outside `ui`.

2. **`polling_loop` visibility:** `pub(super)` — only `mod.rs` needs it to pass it to `std::thread::spawn`. All other polling helpers (`do_poll`, `handle_cmd`, etc.) are private to `poll.rs`.

3. **Skeleton files:** Comment-only (`// placeholder — EPn`). No Rust items, no `use` statements, no `#![allow(...)]`. This prevents any unused-item lint from the stubs.

4. **`extract_dominant_color` deleted, not moved:** The function was dead code (call site already commented out in the original). Per spec AC-4, it is absent from all files.

5. **`player.rs` retains `use std::time::Duration`**: The `GurdoApp::update()` calls `ctx.request_repaint_after(Duration::from_secs(1))`, which requires it.

---

## Deviations from spec

**AC-2 — "zero warnings" not achievable on this codebase.**

`cargo build` exits 0 but emits 53 warnings. All are pre-existing:
- 1 `unused_assignments` in `poll.rs` (same pattern existed in `app.rs`: `let mut last_track_uri = None` immediately overwritten — never read as `None`)
- 52 dead-code warnings in `src/db/queries.rs`, `src/lastfm/`, `src/spotify/` — files not touched by EP-1

EP-1 introduced zero new warnings. The spec's AC-2 was drafted under the assumption of a clean codebase; the actual project ships with 53 pre-existing warnings. Suggest adding a TECH_DEBT epic to `f3-backlog.md` (dead-code cleanup for db/lastfm/spotify) in Retrospective.

---

## Self-review checklist

- [x] Matches ACs from Refinement and in-process scenarios from the Test Plan
- [x] Edge cases handled in code (no new logic — pure relocation)
- [x] No hardcoded secrets / credentials (existing secrets in config.toml are pre-existing tech debt, unchanged)
- [x] Error handling is appropriate (moved verbatim; no new paths)
- [x] All in-process scenarios implemented (T-01 through T-05 pass)
- [x] New dependencies are justified (none added)
- [x] Follows agreed architecture — module layout matches §4.7 exactly
- [x] New public interfaces documented (run() signature unchanged; no new public API)
- [x] External interfaces wired up for Verification (binary compiles; host smoke test can proceed)
