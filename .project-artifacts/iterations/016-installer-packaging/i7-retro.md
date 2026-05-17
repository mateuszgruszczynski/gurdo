# Iteration 16 Retrospective — Installer packaging (EP-14)

## What went well

- Single shell script with `set -euo pipefail` gave AC-6 (build-failure exit) for free.
- `tar -C` double-flag trick cleanly placed both files at archive root without a staging directory.
- All 7 scenarios passed first run; no regressions in `cargo test`.

## What was harder than expected

Nothing significant. `file` and `xxd` absent in the container required falling back to `od -c` for the ELF check — trivial workaround.

## Plan changes

None. EP-15 (Traditional Chinese font) remains parked. Backlog is otherwise exhausted.

## Updated backlog

| # | Name | Priority | Status |
|---|------|----------|--------|
| EP-14 | Installer packaging | P3 | DONE |
| EP-15 | Traditional Chinese font (on demand) | P3 | parked |

## Proposed next epic

Backlog exhausted (only EP-15 remains, parked). Pipeline complete.
