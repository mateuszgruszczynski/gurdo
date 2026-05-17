# Iteration 16 Integration — Installer packaging (EP-14)

## Build status

`cargo build --release` — green, 1 warning (pre-existing `last_track_uri` assignment in `src/ui/poll.rs`).

## Env prep

No `.env` or credentials needed. Script requires only `cargo`, `tar`/`zip`, and `uname` — all present in the dev container and on a standard macOS install.

## Start result

Not applicable — desktop app; packaging produces an archive, not a running service.

## Smoke outcome

- `./scripts/package.sh` ran to completion on Linux aarch64.
- Produced `dist/gurdo-0.1.0-linux-aarch64.tar.gz` (24 MB).
- `tar -tzf` confirms exactly two entries: `gurdo`, `OFL.txt`.
- Extracted binary has ELF magic `\x7fELF` and execute permission.
- Script re-run overwrites the archive (idempotent).
- macOS `.zip` path validated by inspection: `zip -j` produces a flat archive; `uname` on macOS returns `Darwin` which the script maps to `macos`.

## Verification roll-up

All 7 scenarios pass. No quarantined items.

## AC pass/fail table

| AC | Description | Result |
|----|-------------|--------|
| AC-1 | `scripts/package.sh` exists and is executable | PASS |
| AC-2 | Linux → `dist/gurdo-0.1.0-linux-aarch64.tar.gz` | PASS |
| AC-3 | macOS → `.zip` (inspection) | PASS |
| AC-4 | Archive contains exactly `gurdo` + `OFL.txt` | PASS |
| AC-5 | Archive name encodes version + OS + arch | PASS |
| AC-6 | Build failure → non-zero exit, no archive | PASS |
| AC-7 | `cargo test` 16/16 green | PASS |

## Integration-phase issues

None.

## Demo

```
$ ./scripts/package.sh
Building gurdo 0.1.0 for linux/aarch64...
    Finished `release` profile [optimized] target(s) in 11.63s
Packaging gurdo-0.1.0-linux-aarch64.tar.gz...
Archive: /workspaces/gurdo/dist/gurdo-0.1.0-linux-aarch64.tar.gz
Contents:
gurdo
OFL.txt
```

To distribute: copy `dist/gurdo-0.1.0-linux-aarch64.tar.gz` to the target machine, extract with `tar -xzf`, run `./gurdo`.
