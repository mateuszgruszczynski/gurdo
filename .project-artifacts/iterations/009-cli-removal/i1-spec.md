# Iteration 9 Spec — CLI removal & entry-point collapse (EP-2)

*Epic: EP-2 · Phase: Refinement · Date: 2026-05-12*

---

## Problem

`main.rs` is 217 lines with 8 `clap` subcommands (SyncLastfm, Expand, Score, Recommend,
FetchTracks, Login, Devices, Play) whose in-process equivalents now live in the Settings
window (EP-7). The `clap` dependency adds ~2 MB to the binary and parse overhead. EP-2
deletes the CLI layer and makes `gurdo` a UI-only binary.

---

## Scope

### In scope

**`src/main.rs`** — rewrite to ~30 lines:
```rust
mod config;
mod db;
mod engine;
mod lastfm;
mod progress;
mod spotify;
mod sync;
mod ui;

fn main() -> anyhow::Result<()> {
    // parse -c / --config manually (no clap)
    let config_path = parse_config_arg();
    let config = config::Config::load(&config_path)?;
    std::fs::create_dir_all(config.data_dir())?;
    ui::run(config, config_path)
}

fn parse_config_arg() -> std::path::PathBuf {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "-c" || arg == "--config" {
            if let Some(val) = args.next() {
                return val.into();
            }
        }
    }
    "config.toml".into()
}
```

**`Cargo.toml`** — remove the `clap` dependency line entirely.

**Tracing init** — keep the `tracing_subscriber` init (moves from `main` into `ui::run` or
stays at the top of the new `main`). Keep the `rustls` provider install line.

### Out of scope

- Removing the now-unreachable sync/engine modules themselves (their code is actively called
  from the UI dispatcher — EP-7). The modules stay; only the CLI wiring is deleted.
- Changing the `progress` or `ui` modules.
- Any behaviour change to the running app.

---

## Acceptance criteria

| # | Criterion |
|---|---|
| AC-1 | `Cargo.toml` contains no `clap` entry; `cargo tree` shows `clap` is not a transitive dependency. |
| AC-2 | `gurdo` (no args) launches the UI directly. |
| AC-3 | `gurdo -c /path/to/config.toml` uses the specified config. |
| AC-4 | `main.rs` contains no `Cli`/`Command` struct definitions, no `clap` imports. |
| AC-5 | `cargo build` produces zero new warnings beyond the 53 pre-existing baseline. |

---

## Implementation notes

### Tracing init placement

Keep tracing init in the new `main.rs` before `ui::run`. It is a one-time global setup
and does not belong inside the UI module.

### `#[tokio::main]` removal

The old `main` was `async fn main` with `#[tokio::main]`. The new `main` is sync — tokio
runtime is started by `ui::run` via `std::thread::spawn` + `tokio::runtime::Builder`.
Remove `#[tokio::main]` attribute; change `async fn main` → `fn main`.

### `rustls` provider

Keep `let _ = rustls::crypto::ring::default_provider().install_default();` — it must run
before any TLS connection is made (Spotify OAuth, API calls).

---

## Files changed (expected)

| File | Change |
|---|---|
| `src/main.rs` | Rewrite: ~30 lines; no clap; parse_config_arg; sync fn main |
| `Cargo.toml` | Remove `clap = { version = "4", features = ["derive"] }` line |
