# Iteration 12 Spec — Secrets hardening & multi-user config (EP-11)

*Epic: EP-11 · Phase: Refinement · Date: 2026-05-12*

---

## Problem

`config.toml` in the repo contains three personal/sensitive values:
- `[lastfm].api_key = "ccc2281a177faccd7eb7835515ee1ed9"`
- `[lastfm].username = "grucha666"`
- `[spotify].client_id = "b5e0f935d5b74b1cb7c2fc40a0e9b45e"`

A new user cloning the repo picks up someone else's credentials. A contributor submitting
a PR diff risks leaking keys. EP-11 moves these three fields to a separate
`secrets.toml` file that lives alongside `config.toml` but is never committed.

---

## Scope

### In scope

**`config.toml`** (committed copy):
- Replace real values with placeholders: `api_key = "YOUR_LASTFM_API_KEY"`,
  `username = "YOUR_LASTFM_USERNAME"`, `client_id = "YOUR_SPOTIFY_CLIENT_ID"`.

**`config.toml.example`**:
- Apply the same placeholder cleanup.
- Add a comment block explaining `secrets.toml` and its expected format.

**`.gitignore`**:
- Add `secrets.toml` so the file is never accidentally committed.

**`src/config.rs`**:
- Add `SecretsConfig { lastfm_api_key: String, lastfm_username: String, spotify_client_id: String }`.
- Add `Config::secrets_path(config_path: &Path) -> PathBuf` — returns `config_dir/secrets.toml`.
- Extend `Config::load(path)`: after loading the base config, call
  `Config::load_secrets(secrets_path)` and overlay the three fields if the file exists.
  If secrets file is absent, the values from `config.toml` are used as-is (backward compat).

**`src/main.rs`**:
- No changes required — `Config::load` handles secrets internally.

### Out of scope

- First-launch wizard in the UI (EP-11 is file-based only).
- Environment variable fallback.
- Auto-migration: existing users create `secrets.toml` manually (documented in
  `config.toml.example`). Their existing `config.toml` continues working until they
  switch.
- Changing `Config::save` behavior (knob saves still serialize the full Config; users who
  switch to `secrets.toml` should keep placeholder values in `config.toml` to avoid
  Config::save round-tripping real values back).
- Encryption at rest of the secrets file.
- Multi-account support.

---

## Acceptance criteria

| # | Criterion |
|---|---|
| AC-1 | `config.toml` in the repo contains no real api_key, username, or client_id. |
| AC-2 | `config.toml.example` contains only placeholder values and explains `secrets.toml`. |
| AC-3 | `.gitignore` excludes `secrets.toml`. |
| AC-4 | When `secrets.toml` exists alongside `config.toml`, `Config::load` overlays those three fields. |
| AC-5 | When `secrets.toml` is absent, `Config::load` works as before (backward compat). |
| AC-6 | `cargo build` produces zero new warnings beyond the 53 baseline. |

---

## Implementation notes

### `SecretsConfig` and overlay

```rust
#[derive(Debug, Deserialize)]
struct SecretsConfig {
    #[serde(default)]
    lastfm:   SecretsLastfm,
    #[serde(default)]
    spotify:  SecretsSpotify,
}

#[derive(Debug, Default, Deserialize)]
struct SecretsLastfm {
    api_key:  Option<String>,
    username: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct SecretsSpotify {
    client_id: Option<String>,
}
```

Nested to match the natural TOML structure a user would write:

```toml
# secrets.toml
[lastfm]
api_key  = "ccc2281a177faccd7eb7835515ee1ed9"
username = "grucha666"

[spotify]
client_id = "b5e0f935d5b74b1cb7c2fc40a0e9b45e"
```

### `Config::load` extension

```rust
pub fn load(path: &Path) -> Result<Self> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Cannot read config file: {}", path.display()))?;
    let mut config: Config = toml::from_str(&content)
        .with_context(|| format!("Invalid config file: {}", path.display()))?;
    let secrets_path = Self::secrets_path(path);
    if secrets_path.exists() {
        let sc = Self::load_secrets(&secrets_path)?;
        if let Some(k) = sc.lastfm.api_key   { config.lastfm.api_key   = k; }
        if let Some(u) = sc.lastfm.username  { config.lastfm.username  = u; }
        if let Some(c) = sc.spotify.client_id { config.spotify.client_id = c; }
    }
    Ok(config)
}

pub fn secrets_path(config_path: &Path) -> PathBuf {
    config_path.with_file_name("secrets.toml")
}

fn load_secrets(path: &Path) -> Result<SecretsConfig> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Cannot read secrets file: {}", path.display()))?;
    toml::from_str(&content)
        .with_context(|| format!("Invalid secrets file: {}", path.display()))
}
```

### `config.toml.example` addition

```toml
# Sensitive values (api_key, username, client_id) can be kept in a separate
# secrets.toml alongside this file so config.toml is safe to commit.
# Example secrets.toml:
#
#   [lastfm]
#   api_key  = "your-lastfm-api-key"
#   username = "your-lastfm-username"
#
#   [spotify]
#   client_id = "your-spotify-client-id"
```

---

## Files changed (expected)

| File | Change |
|------|--------|
| `config.toml` | Replace 3 real values with placeholders |
| `config.toml.example` | Replace real values; add secrets.toml documentation |
| `.gitignore` | Add `secrets.toml` |
| `src/config.rs` | `SecretsConfig` structs; `secrets_path`; `load_secrets`; `Config::load` overlay |
