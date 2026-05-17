# Iteration 12 Development — Secrets hardening & multi-user config (EP-11)

## Files changed

| File | Change |
|------|--------|
| `src/config.rs` | Added `SecretsConfig`, `SecretsLastfm`, `SecretsSpotify` private structs; `Config::secrets_path()`; `Config::load_secrets()`; `Config::load` overlay; 3 unit tests (SC-1/2/3) |
| `Cargo.toml` | Added `[dev-dependencies] tempfile = "3"` (required for temp-dir tests) |
| `config.toml` | Replaced real `api_key`, `username`, `client_id` with `YOUR_*` placeholders |
| `config.toml.example` | Same placeholder cleanup; added `secrets.toml` format documentation block at top |
| `.gitignore` | Added `secrets.toml` |
| `secrets.toml` | Created with real values for the current user's dev workflow (gitignored) |

## In-process tests

| Scenario | Level | Result | AC |
|----------|-------|--------|----|
| SC-1 `secrets_path_is_sibling` | Unit | PASS | AC-4 |
| SC-2 `load_overlays_secrets_when_present` | Unit | PASS | AC-4 |
| SC-3 `load_uses_config_values_when_secrets_absent` | Unit | PASS | AC-5 |
| SC-4 existing StateReporter tests | Unit | PASS | AC-6 |

## Key decisions / issues

- First attempt at `minimal_config_toml()` omitted required `[sync]` and `[engine]`
  sections (those structs have per-field `#[serde(default)]` but the section header is
  still required). Fixed by including all non-defaulted sections.
- `SecretsConfig` and sub-structs are `pub(crate)` only via the `fn load_secrets` private
  function — no public API surface added.
- `tempfile` added as a `[dev-dependencies]` entry so test infra doesn't affect the
  production binary.

## Warnings

`cargo build --release`: 53 (baseline maintained).
