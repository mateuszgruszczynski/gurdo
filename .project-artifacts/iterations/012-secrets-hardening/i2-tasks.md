# Iteration 12 Decomposition — Secrets hardening & multi-user config (EP-11)

## Tasks

### DEV-1 — `src/config.rs`: SecretsConfig structs + overlay
- Add private `SecretsConfig`, `SecretsLastfm`, `SecretsSpotify` structs.
- Add `pub fn secrets_path(config_path: &Path) -> PathBuf`.
- Add private `fn load_secrets(path: &Path) -> Result<SecretsConfig>`.
- Extend `Config::load`: after base load, call overlay if `secrets_path` exists.
- **AC:** AC-4, AC-5

### DEV-2 — `config.toml`: replace real secrets with placeholders
- `api_key = "YOUR_LASTFM_API_KEY"`, `username = "YOUR_LASTFM_USERNAME"`,
  `client_id = "YOUR_SPOTIFY_CLIENT_ID"`.
- **AC:** AC-1

### DEV-3 — `config.toml.example`: cleanup + secrets.toml documentation
- Apply the same placeholder substitution.
- Add comment block explaining `secrets.toml` format.
- **AC:** AC-2

### DEV-4 — `.gitignore`: add `secrets.toml`
- **AC:** AC-3

### DEV-5 — `secrets.toml`: create with real values (never committed)
- Create at repo root for the current user's dev workflow.
- Covered by the `.gitignore` rule from DEV-4.

### Cross-cutting — Warning budget
- `cargo build` must produce ≤ 53 warnings.
- **AC:** AC-6

## Decision notes

All tasks map to ACs. No scope added. Auto-continue to Test Plan.
