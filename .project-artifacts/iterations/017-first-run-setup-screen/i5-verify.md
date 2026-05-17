# i5-verify: EP-20 — First-run Setup Screen + User-Scoped Config/Secrets

**Iteration:** 017 · **Date:** 2026-05-17

---

## Test environment

**Type:** Desktop application (egui/eframe). No HTTP server, no message queue, no external service interface.

**Reproduction:** `cargo run --release` on the host machine with a display.

Per Architecture §8: "GUI rendering is verified by the developer running the app on the host." The dev container has no display; all E2E/UI scenarios run on the host at Integration time.

---

## External-service stubs

None required for this epic's automated tests. The OAuth flow (`spotify::auth::run_oauth_flow`) is not exercised by automated tests; it is verified manually during Integration smoke.

---

## System-integration tests

**Count: 0**

This epic has no System-integration scenarios. All functionally-significant behaviour is either:
- Covered at Unit/Component level in Development (ACs 1–9, 14, 15, 23), or
- Covered as E2E/UI manual verification during Integration (ACs 10–22).

There is no networked application interface (no HTTP endpoint, no CLI commands, no file-batch interface) that could be tested out-of-process in a headless environment.

---

## E2E tests (automated)

**Count: 0**

All E2E scenarios from `i3-test-plan.md` are tagged **Level: E2E, Type: UI, Owned by: Verification (manual)**. An egui/eframe window requires a display and user interaction; these cannot be scripted headlessly. They are executed as manual smoke checks during Integration.

---

## Run results

No automated Verification suite to run. All 30 in-process tests from Development pass (confirmed in `i4-dev.md`).

---

## Quarantined tests

None.

---

## AC coverage table

| AC | Coverage | Level | Phase |
|---|---|---|---|
| AC-1 (default config path) | `tests::parse_config_arg_default` | Unit | Development ✓ |
| AC-2 (secrets_path invariant) | `config::tests::secrets_path_always_returns_gurdo_path` + overlay regression tests | Unit/Component | Development ✓ |
| AC-3 (create_dir_all) | `config::tests::migrate_*` tests (implicitly); edge case via `needs_setup_true_when_file_absent` | Unit | Development ✓ — also manual smoke (Integration) |
| AC-4 (dirs dep) | `cargo check` exit 0 | Build | Development ✓ |
| AC-5 (migrate copies) | `config::tests::migrate_copies_when_only_source_exists` | Unit | Development ✓ |
| AC-6 (migrate no-op dest present) | `config::tests::migrate_noop_when_dest_exists` + `migrate_noop_when_only_dest_exists` | Unit | Development ✓ |
| AC-7 (migrate no-op both absent) | `config::tests::migrate_noop_when_both_absent` | Unit | Development ✓ |
| AC-8 (needs_setup true cases) | 5 unit tests covering absent/whitespace/empty/absent-key/unparseable | Unit | Development ✓ |
| AC-9 (needs_setup false case) | `config::tests::needs_setup_false_when_all_keys_present` | Unit | Development ✓ |
| AC-10 (setup window opens) | TS-17 — manual smoke during Integration | E2E/UI | Verification (manual) |
| AC-11 (Phase 1 cancel → error) | TS-19 — manual smoke during Integration | E2E/UI | Verification (manual) |
| AC-12 (Phase 1 fields present) | TS-17 — manual smoke during Integration | E2E/UI | Verification (manual) |
| AC-13 (Continue gating) | TS-18 — manual smoke during Integration | E2E/UI | Verification (manual) |
| AC-14 (secrets.toml written, trimmed) | `ui::setup::tests::write_secrets_trims_and_produces_valid_toml` | Unit | Development ✓ |
| AC-15 (config.toml written if absent) | `ui::setup::tests::write_default_config_creates_when_absent` + `write_default_config_does_not_overwrite` | Unit | Development ✓ |
| AC-16 (write error shown inline) | TS-22 — manual smoke during Integration | E2E/UI | Verification (manual) |
| AC-17 (Phase 2 layout) | TS-23 — manual smoke during Integration | E2E/UI | Verification (manual) |
| AC-18 (OAuth pending state) | TS-26 — manual smoke during Integration | E2E/UI | Verification (manual) |
| AC-19 (OAuth success → player) | TS-24 — manual smoke during Integration | E2E/UI | Verification (manual) |
| AC-20 (OAuth failure → retry/skip) | TS-25 — manual smoke during Integration | E2E/UI | Verification (manual) |
| AC-21 (Phase 2 cancel → error) | TS-27 — manual smoke during Integration | E2E/UI | Verification (manual) |
| AC-22 (post-setup launch same as normal) | TS-24, TS-30 — manual smoke during Integration | E2E/UI | Verification (manual) |
| AC-23 (chmod 600) | `ui::setup::tests::write_secrets_applies_chmod_600` (unix) | Unit | Development ✓ |
| AC-24 (no credential logging) | In-process only — no credential values in any `tracing::*` / `eprintln!` / `dbg!` call; confirmed by code audit in T10 | Component/Audit | Development ✓ |

**In-process only (no out-of-process observable for automated testing):** AC-10, 11, 12, 13, 16, 17, 18, 19, 20, 21, 22 — all are UI-only behaviours requiring a display. Covered by manual Integration smoke per Architecture §8.

**All non-quarantined in-process tests: 30/30 pass.**
