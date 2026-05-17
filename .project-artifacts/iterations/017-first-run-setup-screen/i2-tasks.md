# i2-tasks: EP-20 — First-run Setup Screen + User-Scoped Config/Secrets

**Iteration:** 017 · **Date:** 2026-05-17

---

## Task List

---

**T01**
**Title:** Add `dirs` crate dependency
**Role:** DEV
**Description:** Add `dirs = "5"` to `[dependencies]` in `Cargo.toml`. Run `cargo check` to confirm the crate resolves with no new warnings.
**Depends on:** —
**Done when:** `Cargo.toml` contains `dirs = "5"`, `Cargo.lock` is updated, `cargo check` exits 0.

---

**T02**
**Title:** Change default config path to `~/.gurdo/config.toml`
**Role:** DEV
**Description:** In `parse_config_arg()` (`src/main.rs`), replace the `"config.toml"` default with `dirs::home_dir().unwrap_or_default().join(".gurdo/config.toml")`. Handle `home_dir()` returning `None` with an early exit and human-readable error.
**Depends on:** T01
**Done when:** Default config path resolves to `$HOME/.gurdo/config.toml`; `cargo check` passes.

---

**T03**
**Title:** Fix `Config::secrets_path()` to always return `~/.gurdo/secrets.toml`
**Role:** DEV
**Description:** Change `Config::secrets_path(config_path: &Path)` in `src/config.rs` so it always returns `dirs::home_dir().unwrap_or_default().join(".gurdo/secrets.toml")`, ignoring `config_path`. Remove the sibling-file derivation.
**Depends on:** T01
**Done when:** `Config::secrets_path()` returns `$HOME/.gurdo/secrets.toml` for any input path; existing callers compile without change.

---

**T04**
**Title:** Create `~/.gurdo/` directory on launch
**Role:** DEV
**Description:** In `main()` (before `parse_config_arg`), call `fs::create_dir_all(dirs::home_dir().unwrap_or_default().join(".gurdo"))`. Propagate errors with context: `"Cannot create config directory ~/.gurdo/: <OS error>"`.
**Depends on:** T01
**Done when:** On a clean run `~/.gurdo/` is created if absent; idempotent when it already exists.

---

**T05**
**Title:** Implement `migrate_secrets_if_needed()`
**Role:** DEV
**Description:** In `src/config.rs` (or a new `src/migration.rs`), implement `pub fn migrate_secrets_if_needed() -> anyhow::Result<()>`. Logic: if `~/.gurdo/secrets.toml` is absent AND `./secrets.toml` (relative to CWD) exists → `fs::copy` source to dest, emit `tracing::info!` (no key values). Otherwise no-op.
**Depends on:** T04
**Done when:** Function compiles; unit tests in T11 pass.

---

**T06**
**Title:** Implement `needs_setup()`
**Role:** DEV
**Description:** In `src/config.rs`, implement `pub fn needs_setup() -> bool`. Returns `true` if `~/.gurdo/secrets.toml` is absent, unreadable, unparseable as TOML, or any of `api_key` / `username` / `client_id` is absent or empty after `.trim()`. Returns `false` only when all three are present and non-empty.
**Depends on:** T03
**Done when:** Function compiles; unit tests in T12 pass.

---

**T08**
**Title:** Implement setup window Phase 1 — credentials form
**Role:** DEV
**Description:** Create `src/ui/setup.rs`. Implement `pub fn run(config_path: &Path) -> anyhow::Result<()>` as a standalone eframe window (title "Gurdo — Setup", inner size 440×400 px, resizable false). Phase 1: three `egui::TextEdit::singleline` fields (full-width) labeled "Last.fm API Key", "Last.fm Username", "Spotify Client ID". "Continue" button disabled while any field is whitespace-only. On Continue: serialize `api_key`/`username`/`client_id` as trimmed values, write `~/.gurdo/secrets.toml`, apply `chmod 0o600` (AC-14 / T10 logic here), write default `~/.gurdo/config.toml` if absent, advance to Phase 2. Closing the window before Continue → return `Err("Setup cancelled")`.
**Depends on:** T03
**Done when:** Window opens, fields accept input, Continue is gated, files are written on Continue, `cargo check` passes.

---

**T09**
**Title:** Implement setup window Phase 2 — Spotify OAuth
**Role:** DEV
**Description:** Phase 2 within the same eframe window (Phase 1 fields no longer visible). Status label ("Connect your Spotify account to enable playback."), "Connect Spotify" button, "Skip for now" button. On "Connect Spotify": call `spotify::auth::run_oauth_flow`; while pending, disable both buttons and update status label to "Waiting for Spotify authorisation…". On `Ok(())`: close window, return `Ok(())`. On `Err(e)`: show "OAuth failed: <e>" in red, enable "Retry" (re-runs flow) and "Skip for now" (closes window, returns `Ok(())`). Closing window during Phase 2 → return `Err("Setup cancelled during OAuth")`.
**Depends on:** T08
**Done when:** Phase 2 UI is present and all three outcomes (success / retry / skip) compile and are reachable.

---

**T10**
**Title:** Apply `chmod 600` and audit no-credential-logging
**Role:** SECURITY
**Description:** Verify that `src/ui/setup.rs` applies `std::fs::set_permissions` with `0o600` immediately after writing `~/.gurdo/secrets.toml` (via `#[cfg(unix)]` + `PermissionsExt::from_mode`; no-op `#[cfg(not(unix))]`). Grep all `tracing::`, `log::`, `eprintln!`, `println!`, `dbg!` calls in the secrets write path and confirm no key values are emitted. Add a brief audit comment block at the top of `src/ui/setup.rs`.
**Depends on:** T08
**Done when:** `stat ~/.gurdo/secrets.toml` shows `-rw-------`; audit comment present; no credential values reachable in log output.

---

**T07**
**Title:** Wire migration + setup check into `main`
**Role:** DEV
**Description:** After `create_dir_all` (T04) in `main()`: call `migrate_secrets_if_needed()`, then `if needs_setup() { setup::run(&config_path)?; }`. After `run` returns `Ok(())`, proceed to `Config::load(&config_path)` then `ui::run(config, config_path)` as before.
**Depends on:** T04, T05, T06, T09
**Done when:** `main` compiles; binary without `~/.gurdo/secrets.toml` launches setup window; binary with valid secrets skips setup and opens player.

---

**T11**
**Title:** Unit tests for `migrate_secrets_if_needed()`
**Role:** DEV
**Description:** Factor the function to accept home-dir and cwd as arguments for testability (or use `tempfile::tempdir`). Cover: (a) both absent → no-op; (b) source present, dest absent → dest created with source contents; (c) both present → dest unchanged; (d) source absent, dest present → no-op.
**Depends on:** T05
**Done when:** `cargo test` runs all four cases green.

---

**T12**
**Title:** Unit tests for `needs_setup()`
**Role:** DEV
**Description:** Factor the function to accept a path argument for testability. Cover: (a) file absent → `true`; (b) all three non-empty → `false`; (c) `api_key` empty → `true`; (d) `username` empty → `true`; (e) `client_id` empty → `true`; (f) file unparseable → `true`.
**Depends on:** T06
**Done when:** `cargo test` runs all six cases green.

---

**T13**
**Title:** Update `parse_config_arg_default` test
**Role:** DEV
**Description:** In `src/main.rs`, update the existing `parse_config_arg_default` test to assert the path ends with `.gurdo/config.toml` (rather than the literal value, to avoid depending on the runner's `$HOME`).
**Depends on:** T02
**Done when:** `cargo test parse_config_arg_default` passes green; no other tests regress.

---

**T14**
**Title:** Unit test for `Config::secrets_path()` invariant
**Role:** DEV
**Description:** Add a `#[cfg(test)]` test asserting `Config::secrets_path(&any_path)` always ends with `.gurdo/secrets.toml` for at least two distinct inputs (default path and an arbitrary override).
**Depends on:** T03
**Done when:** `cargo test` includes this test and it passes green.

---

**T15**
**Title:** DESIGN review of setup window layout
**Role:** DESIGN
**Description:** With T08 and T09 implemented, run the binary in setup mode and evaluate: labels legible and aligned, TextEdit fields sized appropriately, Continue button state visually distinct when disabled, Phase 2 error message readable inline. Apply trivial egui spacing/padding fixes directly in `src/ui/setup.rs`; document structural blockers as comments.
**Depends on:** T08, T09
**Done when:** Window presents acceptably at 440×400 px; all layout issues fixed or documented.

---

**T17**
**Title:** Update README and config example for new default paths
**Role:** DEV
**Description:** Update `README.md` to document `~/.gurdo/config.toml` and `~/.gurdo/secrets.toml` as the new defaults; note that first launch creates them automatically. Update `config.toml.example` header comment if present. Remove any references to local-directory `config.toml` / `secrets.toml` as the required location.
**Depends on:** T02, T03
**Done when:** README and example accurately describe new paths; no stale local-path references.

---

**T18**
**Title:** Full test suite green + `cargo clippy` clean
**Role:** DEV
**Description:** Run `cargo test --all` and `cargo clippy -- -D warnings`. Fix any clippy lints introduced by this epic.
**Depends on:** T07, T11, T12, T13, T14
**Done when:** Both commands exit 0.

---

## Dependency graph

```
T01 → T02 → T13
T01 → T03 → T06 → T12
         └→ T14
T01 → T04 → T05 → T11
         └→ (T07)
T03, T04, T05, T06, T09 → T07
T03 → T08 → T09 → T07
          └→ T10
          └→ T15 (with T09)
T10, T08, T09 → T16 (SECURITY audit)
T02, T03 → T17
T07, T11, T12, T13, T14 → T18
```

## AC coverage

| AC | Task(s) |
|---|---|
| AC-1 (default path) | T02, T13 |
| AC-2 (secrets_path invariant) | T03, T14 |
| AC-3 (create_dir_all) | T04 |
| AC-4 (dirs dep) | T01 |
| AC-5/6/7 (migration) | T05, T11 |
| AC-8/9 (needs_setup) | T06, T12 |
| AC-10/11 (setup::run lifecycle) | T08 |
| AC-12/13 (Phase 1 fields + gating) | T08 |
| AC-14/15/16 (file writes + errors) | T08, T10 |
| AC-17–21 (Phase 2 OAuth) | T09 |
| AC-22 (post-setup launch) | T07 |
| AC-23 (chmod 600) | T10 |
| AC-24 (no credential logging) | T10 |
| Docs | T17 |
| Design | T15 |
| Suite health | T18 |
