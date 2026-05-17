# Iteration 12 Test Plan — Secrets hardening & multi-user config (EP-11)

## Scenarios

### Unit — In-process

#### SC-1 · secrets_path returns sibling file (AC-4)
**Level:** Unit  
**Given** `config_path = "/some/dir/config.toml"`  
**When** `Config::secrets_path(config_path)` is called  
**Then** it returns `"/some/dir/secrets.toml"`

#### SC-2 · Config::load overlays secrets when secrets.toml exists (AC-4)
**Level:** Unit  
**Given** a temp dir with `config.toml` (placeholder values) and `secrets.toml` (real values)  
**When** `Config::load` is called  
**Then** `config.lastfm.api_key`, `config.lastfm.username`, `config.spotify.client_id`
  equal the values from `secrets.toml`

#### SC-3 · Config::load uses config values when secrets.toml absent (AC-5)
**Level:** Unit  
**Given** a temp dir with only `config.toml` (direct values, no secrets.toml)  
**When** `Config::load` is called  
**Then** the three fields come from `config.toml` unchanged

#### SC-4 · Existing StateReporter tests pass unchanged (AC-6 / regression)
**Level:** Unit  
`stage_resets_current_and_total`, `tick_updates_progress`,
`reporter_is_noop_when_active_is_none` — all must pass.

### Out-of-process

None — this epic is pure file I/O; AC-1/AC-2/AC-3 are verified by inspection.

## AC coverage

| AC | Scenario(s) |
|----|-------------|
| AC-1 | Inspection of committed `config.toml` |
| AC-2 | Inspection of committed `config.toml.example` |
| AC-3 | Inspection of `.gitignore` |
| AC-4 | SC-1, SC-2 |
| AC-5 | SC-3 |
| AC-6 | `cargo build` warning count; SC-4 |
