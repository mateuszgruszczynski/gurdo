# Iteration 16 Verification — Installer packaging (EP-14)

## Environment

Dev container: Linux aarch64, Ubuntu 24.04. `cargo build --release` used cached incremental output from the development phase run.

## Stubs

None — script calls `cargo` and standard shell utilities only.

## Test results

| Scenario | Description | Result |
|----------|-------------|--------|
| S-1 | `dist/gurdo-0.1.0-linux-aarch64.tar.gz` exists (24 MB) | PASS |
| S-2 | `tar -tzf` lists exactly `gurdo` and `OFL.txt` | PASS |
| S-3 | ELF magic `\x7fELF` confirmed via `od`; `test -x` exit 0 | PASS |
| S-4 | Broken `src/main.rs` → `cargo build --release` exit 101; no archive written | PASS |
| S-5 | `test -x scripts/package.sh` exit 0 | PASS |
| S-6 | `cargo test` 16/16 pass | PASS |
| S-7 | macOS branch (`zip -j`) verified by inspection | PASS |

## AC coverage

| AC | Scenario(s) | Result |
|----|-------------|--------|
| AC-1 | S-5 | PASS |
| AC-2 | S-1, S-3 | PASS |
| AC-3 | S-7 (inspection) | PASS |
| AC-4 | S-2 | PASS |
| AC-5 | S-1, S-7 | PASS |
| AC-6 | S-4 | PASS |
| AC-7 | S-6 | PASS |

## Quarantined items

None.

## Notes

- `file` and `xxd` not available in the container; ELF check done via `od -c` (equivalent — reads same magic bytes).
- macOS path (S-3 variant, S-7) cannot be executed in the dev container; validated by code inspection only. A host-side smoke of `zip -j` is documented in i6-int.md.
