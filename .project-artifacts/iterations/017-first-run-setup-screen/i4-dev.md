# i4-dev: EP-20 — First-run Setup Screen + User-Scoped Config/Secrets

**Iteration:** 017 · **Date:** 2026-05-17

---

## Files changed

| File | Change |
|---|---|
| `Cargo.toml` | Added `dirs = "5"` dependency |
| `Cargo.lock` | Updated (dirs 5.0.1, dirs-sys 0.4.1, option-ext 0.2.0) |
| `src/config.rs` | `secrets_path()` → always `~/.gurdo/secrets.toml`; `dirs_home()` → `dirs::home_dir()`; added `gurdo_dir()`, `needs_setup()`, `migrate_secrets_if_needed()`; refactored `load()` → `load_inner()` + `load_with_secrets_at()` for testability; updated + added 10 tests |
| `src/main.rs` | Default config path → `~/.gurdo/config.toml`; added `create_dir_all`, `migrate_secrets_if_needed`, `needs_setup`, `setup::run` preamble; updated `parse_config_arg_default` test |
| `src/ui/mod.rs` | Added `pub mod setup;` |
| `src/ui/setup.rs` | New file: `SetupApp` eframe window (Phase 1 + Phase 2), `write_secrets()`, `write_default_config_if_absent()`, `run()`; 4 unit tests |
| `README.md` | Created; documents `~/.gurdo/` paths and setup wizard |
| `config.toml.example` | Updated header to reference `~/.gurdo/config.toml` as default location |

---

## In-process tests written

### Unit (Development-owned)

| Test | AC covered |
|---|---|
| `config::tests::secrets_path_always_returns_gurdo_path` | AC-2 |
| `config::tests::load_overlays_secrets_when_present` (updated) | AC-2 regression |
| `config::tests::load_uses_config_values_when_secrets_absent` (updated) | AC-2 regression |
| `config::tests::needs_setup_true_when_file_absent` | AC-8 |
| `config::tests::needs_setup_false_when_all_keys_present` | AC-9 |
| `config::tests::needs_setup_true_when_api_key_whitespace` | AC-8 |
| `config::tests::needs_setup_true_when_username_empty` | AC-8 |
| `config::tests::needs_setup_true_when_client_id_absent` | AC-8 |
| `config::tests::needs_setup_true_when_file_unparseable` | AC-8 |
| `config::tests::migrate_copies_when_only_source_exists` | AC-5 |
| `config::tests::migrate_noop_when_both_absent` | AC-7 |
| `config::tests::migrate_noop_when_dest_exists` | AC-6 |
| `config::tests::migrate_noop_when_only_dest_exists` | AC-6 |
| `tests::parse_config_arg_default` (updated) | AC-1 |
| `ui::setup::tests::write_secrets_trims_and_produces_valid_toml` | AC-14 |
| `ui::setup::tests::write_secrets_applies_chmod_600` (unix only) | AC-23 |
| `ui::setup::tests::write_default_config_creates_when_absent` | AC-15 |
| `ui::setup::tests::write_default_config_does_not_overwrite` | AC-15 |

Total: 18 new/updated tests (+ 12 pre-existing = 30 total). All 30 green.

### Out-of-process / E2E
None — all UI scenarios (Phase 1/2 window, OAuth) are verified manually during Integration (no display in dev container).

---

## External interfaces wired

None new. The binary's CLI interface (`-c` flag) is unchanged. The setup runs entirely before `ui::run` and produces no new network endpoints.

---

## Key decisions

1. **`load_with_secrets_at` for testability:** `Config::secrets_path` now returns a fixed path (`~/.gurdo/secrets.toml`), which would contaminate tests that write to temp dirs. Added `#[cfg(test)] pub(crate) fn load_with_secrets_at(config_path, secrets_path)` so existing overlay tests remain hermetic. This is the minimal testability refactor.

2. **`migrate_secrets_if_needed` accepts path arguments:** Rather than reading from the real `~/.gurdo/` and `cwd`, the function accepts `gurdo_dir` and `cwd` params. This allows unit tests to use temp dirs without modifying the real home directory. `main.rs` passes the real paths.

3. **`write_credentials` logic for config target:** When `-c /custom/path` is supplied and that file exists (returning user), we write the default config only to `~/.gurdo/config.toml`, not to the custom path. The custom path is the user's existing config; we shouldn't create a duplicate. When the default path is in use, we write there. This matches AC-15 ("only when absent").

4. **OAuth uses `~/.gurdo/config.toml` directly:** Inside Phase 2, `run_oauth_flow` is called with a `Config` loaded from `~/.gurdo/config.toml` (written in Phase 1), not from `config_path`. This avoids the edge case where `-c` points to a non-existent file during a first-run session. The player subsequently loads from the user-specified `config_path` as normal.

5. **`SetupOutcome::InProgress` mapped to CancelledPhase1 error:** If `eframe::run_native` returns without the outcome being set (unexpected crash in the window), we treat it as Phase 1 cancellation rather than success. Safe-fail behavior.

---

## Deviations from spec

None. All 24 ACs implemented.

---

## Self-review checklist

- [x] Matches ACs from Refinement and in-process scenarios from the Test Plan
- [x] Edge cases handled (home_dir None, file absent, TOML parse error, whitespace-only keys, permission errors surfaced via error label)
- [x] No hardcoded secrets/credentials
- [x] Error handling appropriate (anyhow context on every fs/io call)
- [x] All in-process scenarios implemented (18 unit tests covering ACs 1–9, 14, 15, 23)
- [x] `dirs = "5"` justified (cross-platform home dir, replaces manual `$HOME` env parse)
- [x] Follows agreed architecture — no unilateral structural changes
- [x] New default paths documented in README + config.toml.example
- [x] No new compiler warnings (pre-existing `last_track_uri` warning unchanged)
