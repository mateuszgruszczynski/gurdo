# Iteration 9 Integration — CLI removal & entry-point collapse (EP-2)

## Build

`cargo build --release` → 53 warnings, 0 errors. ✓
`cargo tree | grep clap` → empty. ✓

## Smoke

`./target/release/gurdo` — UI launches immediately. No subcommand prompt.
`./target/release/gurdo -c config.toml` — launches with explicit config path.
`./target/release/gurdo --unknown-flag` — ignored gracefully (parse_config_arg skips
  unrecognised flags and falls back to default path).

## AC pass/fail

| AC | Result |
|---|---|
| AC-1 | PASS — clap absent from cargo tree |
| AC-2 | PASS — `gurdo` launches UI directly |
| AC-3 | PASS — `-c path` config override works |
| AC-4 | PASS — no Cli/Command/clap in main.rs |
| AC-5 | PASS — 53 warnings |

## Integration issues

None. This is the **MVP close** — all P1 epics are now DONE.
