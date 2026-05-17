# Iteration 9 Test Plan — CLI removal & entry-point collapse (EP-2)

## Scenarios

### S-01 — clap absent from cargo tree (AC-1)
`cargo tree | grep clap` → empty output.

### S-02 — default config path used with no args (AC-2, AC-3)
`parse_config_arg()` with no args returns `PathBuf::from("config.toml")`.

### S-03 — -c flag overrides config path (AC-3)
`parse_config_arg()` with args `["-c", "/tmp/other.toml"]` returns `/tmp/other.toml`.

### S-04 — No Cli/Command in main.rs (AC-4)
`grep -n "struct Cli\|enum Command\|use clap" src/main.rs` → empty.

### S-05 — Zero new warnings (AC-5)
`cargo build` warning count == 53.

## Level assignments

| Scenario | Level | Runs in |
|---|---|---|
| S-01 | compile check | Integration |
| S-02 – S-03 | Unit | Development (in-process) |
| S-04 | grep check | Development |
| S-05 | compile check | Development |
