# Iteration 16 Dev — Installer packaging (EP-14)

## Files changed

| File | Change |
|------|--------|
| `scripts/package.sh` | New — OS-detecting packaging script (49 lines) |
| `.gitignore` | Added `/dist` entry |

## In-process tests by level

| Scenario | Level | AC | Result |
|----------|-------|----|--------|
| S-5: script is executable | Component | AC-1 | PASS (`test -x`) |
| S-6: cargo test green | Component | AC-7 | PASS (16/16) |
| S-7: macOS zip branch (inspection) | Component | AC-3, AC-5 | PASS (code review) |

## External interfaces wired

None — no server or database changes; script is standalone.

## Key decisions

- `set -euo pipefail` — any command failure propagates as non-zero exit (AC-6 for free).
- `tar -C` flag used twice to place both files at archive root without a subdirectory prefix.
- `zip -j` (junk paths) for the same reason on macOS.
- Version extracted via `grep '^version' Cargo.toml | head -1 | sed` rather than `cargo metadata` to avoid a slow Cargo invocation at packaging time.
- `uname | tr '[:upper:]' '[:lower:]'` normalises `Darwin` → `darwin` for consistent branching; macOS label mapped to `macos` for the archive name.

## Deviations

None.

## Self-review

- No hardcoded paths — all derived from `$REPO_ROOT`.
- Script is idempotent; re-running overwrites the previous archive.
- `dist/` added to `.gitignore`; archives not committed.
- No secrets or credentials involved.
