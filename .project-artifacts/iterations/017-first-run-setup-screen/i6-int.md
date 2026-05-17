# i6-int: EP-20 — First-run Setup Screen + User-Scoped Config/Secrets

**Iteration:** 017 · **Date:** 2026-05-17

---

## Build status

**Command:** `cargo build --release`
**Result:** ✅ Success
**Warnings:** 1 pre-existing (`last_track_uri` unused assignment in `poll.rs` — deferred, requires logic change, tracked since iteration 015)

---

## Environment preparation

No `.env` file required. This is a native desktop app; credentials are stored in `~/.gurdo/secrets.toml` (written by the setup wizard on first run). No external services need to be pre-configured for the smoke test.

---

## Application start

**Native GUI — hybrid mode.** The binary is built in the container but must run on the host. See smoke test instructions below.

---

## Packaging

**Skipped** — `policy.md` has `packaging: milestone`. `dist/` not produced this iteration.

---

## Manual smoke test

**This section requires execution on the host machine (with display).**

### Instructions

To test the first-run setup wizard, ensure `~/.gurdo/secrets.toml` does not exist on the host, OR temporarily rename it:

```sh
mv ~/.gurdo/secrets.toml ~/.gurdo/secrets.toml.bak
```

Then launch the app (from the workspace, on the host):

```sh
cargo run --release
```

Or use the pre-built binary:

```sh
./target/release/gurdo
```

**Expected flow:**

1. A window titled "Gurdo — Setup" (440×400 px, not resizable) opens immediately — no player window.
2. **Phase 1:** Three labeled full-width text fields: "Last.fm API Key", "Last.fm Username", "Spotify Client ID".
3. While any field is blank/whitespace, the "Continue" button is grey/disabled.
4. Fill all three fields with non-empty values → "Continue" becomes clickable.
5. Click Continue → `~/.gurdo/secrets.toml` is written (chmod 600); `~/.gurdo/config.toml` is created if absent.
6. **Phase 2:** Fields disappear; status label reads "Connect your Spotify account to enable playback."; two buttons: "Connect Spotify" and "Skip for now".
7. Click "Skip for now" → setup window closes; player opens normally.

**Restore secrets after testing:**
```sh
mv ~/.gurdo/secrets.toml.bak ~/.gurdo/secrets.toml
```

**Test returning user (AC-22):** With `~/.gurdo/secrets.toml` present and all three keys non-empty, launch the app — the setup window must NOT appear; the player opens directly.

### Smoke result

*Pending user confirmation — see checkpoint below.*

---

## Verification roll-up

See `i5-verify.md`. All 30 in-process tests pass. All E2E/UI scenarios (TS-17 through TS-30) are manual-only; they are the smoke walk-through above. No automated Verification failures.

---

## AC pass/fail table

| AC | Scenario(s) | Status |
|---|---|---|
| AC-1 (default config path) | `tests::parse_config_arg_default` | ✅ in-process |
| AC-2 (secrets_path invariant) | `config::tests::secrets_path_always_returns_gurdo_path` | ✅ in-process |
| AC-3 (create_dir_all) | Migration tests (implicit); smoke confirms dir created | ✅ in-process + smoke |
| AC-4 (dirs dep) | `cargo build --release` exit 0 | ✅ build |
| AC-5 (migrate copies) | `config::tests::migrate_copies_when_only_source_exists` | ✅ in-process |
| AC-6 (migrate no-op dest present) | migration noop tests | ✅ in-process |
| AC-7 (migrate no-op both absent) | `config::tests::migrate_noop_when_both_absent` | ✅ in-process |
| AC-8 (needs_setup true cases) | 5 unit tests | ✅ in-process |
| AC-9 (needs_setup false case) | `config::tests::needs_setup_false_when_all_keys_present` | ✅ in-process |
| AC-10 (window 440×400, title) | TS-17 — manual smoke | ⏳ smoke |
| AC-11 (Phase 1 cancel → error msg) | TS-19 — manual smoke | ⏳ smoke |
| AC-12 (three labeled fields) | TS-17 — manual smoke | ⏳ smoke |
| AC-13 (Continue gating) | TS-18 — manual smoke | ⏳ smoke |
| AC-14 (secrets written + trimmed) | `ui::setup::tests::write_secrets_trims_and_produces_valid_toml` | ✅ in-process |
| AC-15 (config written if absent) | `ui::setup::tests::write_default_config_*` | ✅ in-process |
| AC-16 (write error inline) | TS-22 — manual smoke | ⏳ smoke |
| AC-17 (Phase 2 layout) | TS-23 — manual smoke | ⏳ smoke |
| AC-18 (OAuth pending state) | TS-26 — manual smoke | ⏳ smoke |
| AC-19 (OAuth success → player) | TS-24 — manual smoke | ⏳ smoke |
| AC-20 (OAuth failure → retry/skip) | TS-25 — manual smoke | ⏳ smoke |
| AC-21 (Phase 2 cancel → error msg) | TS-27 — manual smoke | ⏳ smoke |
| AC-22 (returning user bypasses setup) | TS-30 — manual smoke | ⏳ smoke |
| AC-23 (chmod 600) | `ui::setup::tests::write_secrets_applies_chmod_600` | ✅ in-process |
| AC-24 (no credential logging) | code audit in T10 (confirmed no `tracing::*` / `eprintln!` emits values) | ✅ audit |

---

## Integration-phase issues

None found during build or in-process testing.

---

## Demo outcome

**Demonstrable:** Yes. The setup wizard is the first thing a new user sees.

**Demo script:**
1. Delete or rename `~/.gurdo/secrets.toml`
2. `cargo run --release` (or pre-built binary)
3. Setup window appears → fill three fields → Continue → Phase 2 appears → Skip for now → player opens
4. Kill player; restore `~/.gurdo/secrets.toml`
5. Relaunch → setup window does NOT appear; player opens directly → confirms returning-user path (AC-22)
