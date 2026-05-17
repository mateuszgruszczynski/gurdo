# Iteration 9 Development — CLI removal & entry-point collapse (EP-2)

## Files changed

| File | Change |
|---|---|
| `Cargo.toml` | Removed `clap = { version = "4", features = ["derive"] }` |
| `src/main.rs` | Rewritten: 46 lines (vs. 217); sync `fn main`; `parse_config_arg` helper; no clap; 1 unit test |

## Key decisions

- `parse_config_arg` reads `std::env::args()` directly. Simple enough not to need a
  library; handles `-c`/`--config` only.
- `#[tokio::main]` removed; tokio runtime starts inside `ui::run` via `std::thread::spawn`.
  The `main` function is now a plain synchronous entry point.
- All 8 CLI subcommands (SyncLastfm, Expand, Score, Recommend, FetchTracks, Login, Devices,
  Play) deleted. Their in-process equivalents live in the Settings window (EP-7).
- Tracing init and rustls provider init kept at top of `main` (global setup).

## Tests

4/4 pass: 3 EP-7 unit tests + 1 new parse_config_arg_default sanity check.

## Grep checks

- `grep "struct Cli\|enum Command\|use clap" src/main.rs` → empty ✓
- `cargo tree | grep clap` → empty ✓

## Self-review

- No new warnings (53 baseline)
- Binary slightly smaller: clap removed (~1 MB from dep graph)
