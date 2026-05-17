# Iteration 1 Tasks — UI module split (EP-1)

*Epic: EP-1 · Phase: Decomposition · Date: 2026-05-11*

---

## Task list

| # | Title | Role | Depends on | Maps to AC |
|---|---|---|---|---|
| T-1 | Create `state.rs` | DEV | — | AC-1, AC-2 |
| T-2 | Create `poll.rs` | DEV | T-1 | AC-1, AC-2 |
| T-3 | Create `player.rs` | DEV | T-1 | AC-1, AC-2, AC-3 |
| T-4 | Create skeleton files | DEV | — | AC-1, AC-5 |
| T-5 | Rewrite `mod.rs`, delete `app.rs`, remove dead code | DEV | T-1, T-2, T-3, T-4 | AC-1, AC-2, AC-3, AC-4 |
| T-6 | Verify clean build | DEV | T-5 | AC-2 |
| T-7 | Manual smoke test on host | DEV | T-6 | AC-6 through AC-12 |

---

## Task details

### T-1 — Create `src/ui/state.rs`
**Role:** DEV  
**Depends on:** —

Move `PlayerState` (struct + `#[derive(Clone, Default)]`) and `PlayerCommand` (enum) verbatim from `app.rs` into a new file `src/ui/state.rs`. Make both items `pub`. No other changes.

**Done when:** `state.rs` exists and contains only `PlayerState` and `PlayerCommand`; no other items from `app.rs` are present.

---

### T-2 — Create `src/ui/poll.rs`
**Role:** DEV  
**Depends on:** T-1

Move `do_poll`, `handle_cmd`, `extend_queue_if_needed`, `polling_loop`, and `QUEUE_CHUNK_SIZE` verbatim from `app.rs` into a new file `src/ui/poll.rs`. Update imports to reference `super::state::{PlayerState, PlayerCommand}`. No logic changes.

**Done when:** `poll.rs` exists and contains the five items above; all imports resolve.

---

### T-3 — Create `src/ui/player.rs`
**Role:** DEV  
**Depends on:** T-1

Move `GurdoApp` struct, `impl eframe::App for GurdoApp`, `SettingsDraft` struct + impl, `decode_image`, `fmt_ms`, and `spawn_cli` verbatim from `app.rs` into a new file `src/ui/player.rs`. Make `SettingsDraft` and `SettingsDraft::from_config` at minimum `pub(super)` so `mod.rs` can call them. Update imports to reference `super::state::{PlayerState, PlayerCommand}`. Do not add or remove any logic.

**Done when:** `player.rs` exists with all six items; `SettingsDraft` is visible from `mod.rs`; no logic changes.

---

### T-4 — Create skeleton files
**Role:** DEV  
**Depends on:** —

Create five new files, each containing only the specified comment and nothing else (no `use`, no `pub fn`, no structs):

- `src/ui/settings.rs` — `// placeholder — EP-6`
- `src/ui/background.rs` — `// placeholder — EP-4`
- `src/ui/ops.rs` — `// placeholder — EP-7`
- `src/ui/assets.rs` — `// placeholder — EP-3`
- `src/ui/knobs.rs` — `// placeholder — EP-8`

**Done when:** five files exist; each contains only a comment line; `cargo build` emits no warnings from these files.

---

### T-5 — Rewrite `mod.rs`, delete `app.rs`, remove dead code
**Role:** DEV  
**Depends on:** T-1, T-2, T-3, T-4

1. Rewrite `src/ui/mod.rs` to:
   - Declare all nine sub-modules (`mod state; mod poll; mod player; mod settings; mod background; mod ops; mod assets; mod knobs; mod knobs` — note: `knobs` is separate). Actually: `mod state; pub(super) mod state;` — use the correct visibility for each module. `state`, `poll`, `player` need at least `pub(super)` visibility so other files within `src/ui/` can import from them.
   - Move `pub fn run(config: Config, config_path: PathBuf) -> anyhow::Result<()>` here (lifted verbatim from `app.rs`, minus the `extract_dominant_color` function body and the commented call site).
2. Delete `src/ui/app.rs`.
3. Confirm `extract_dominant_color` and its commented call site (`// self.bg_color = extract_dominant_color(bytes);`) are absent from all files.

**Done when:** `mod.rs` re-exports `pub use` nothing extra (the public API is just `pub fn run`); `app.rs` does not exist; `grep -r "extract_dominant_color" src/` returns empty.

---

### T-6 — Verify clean build
**Role:** DEV  
**Depends on:** T-5

Run `cargo build` (debug profile). Confirm: exit code 0, zero `warning:` lines in stderr.

**Done when:** `cargo build 2>&1 | grep "^warning:"` returns no output and the command exits 0.

---

### T-7 — Manual smoke test on host
**Role:** DEV  
**Depends on:** T-6

Run the app on the host machine (`cargo run -- -c config.toml` or equivalent). Verify each of the following manually (these mirror ACs 6–12):

- [ ] Player window opens with title "Gurdo", correct layout (~440×660)
- [ ] Transport buttons work (play/pause, next, previous, seek ±10 s)
- [ ] Like/Unlike and Dislike buttons record feedback correctly
- [ ] Settings gear opens modal; slider changes save to `config.toml`
- [ ] Error modal appears when token is invalid; OK dismisses it
- [ ] Queue button (☰) starts a recommendation queue on Spotify
- [ ] CJK track/artist names render without tofu blocks

**Done when:** all seven checklist items confirmed; no regressions observed.

---

## Auto-continue assessment

All tasks map to ACs from `i1-spec.md`. Roles are unambiguous (DEV throughout — DESIGN and QA have no tasks in a pure refactor of internal module boundaries). No scope was added beyond the spec. **Auto-continue condition met → Decomposition checkpoint is waived; proceeding to Test Plan.**
