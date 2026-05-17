# Iteration 16 Spec — Installer packaging (EP-14)

## Goal

Produce a distributable release archive for gurdo via a single shell script that works on both Linux and macOS.

## Background

- Binary is self-contained: all fonts are embedded via `include_bytes!` at compile time.
- Dev container is Linux aarch64 (Ubuntu 24.04). The macOS path runs on the host.
- `cargo-bundle` is not installed and is out of scope.
- App version: `0.1.0` (from `[package] version` in `Cargo.toml`).

## Scope

### In scope

- `scripts/package.sh` — OS-detecting packaging script.
- On Linux: produces `dist/gurdo-<version>-linux-<arch>.tar.gz` containing the release binary and `OFL.txt`.
- On macOS: produces `dist/gurdo-<version>-macos-<arch>.zip` containing the release binary and `OFL.txt`.
- `OFL.txt` is sourced from `assets/fonts/OFL.txt` (already in the repository).
- Script exits non-zero on `cargo build --release` failure.
- Script prints progress: build start, archive path, archive contents.

### Out of scope

- `.app` bundle (requires macOS tooling; script produces a plain zip).
- `config.toml.example` or any other files in the archive.
- Windows packaging.
- CI/CD pipeline integration.
- Code-signing or notarisation.
- `cargo-bundle` or any additional tooling beyond the standard shell utilities (`tar`, `zip`, `uname`, `cargo`).

## Acceptance criteria

| AC | Description |
|----|-------------|
| AC-1 | `scripts/package.sh` exists and is executable (`chmod +x`). |
| AC-2 | Running the script on Linux produces `dist/gurdo-<version>-linux-<arch>.tar.gz`. |
| AC-3 | Running the script on macOS produces `dist/gurdo-<version>-macos-<arch>.zip`. |
| AC-4 | The archive contains exactly two entries: the release binary (`gurdo`) and `OFL.txt`. |
| AC-5 | Archive name embeds the version from `Cargo.toml` and the detected OS + arch (e.g. `gurdo-0.1.0-linux-aarch64.tar.gz`). |
| AC-6 | If `cargo build --release` fails, the script exits with a non-zero code and does not produce an archive. |
| AC-7 | `cargo test` passes with no regressions (script is a shell file; no Rust changes). |

## Archive layout

```
gurdo-0.1.0-linux-aarch64.tar.gz
├── gurdo          (release binary)
└── OFL.txt
```

## Script interface

```
Usage: ./scripts/package.sh [--version <override>]
```

- Default version: read from `Cargo.toml` via `grep`.
- `dist/` is created by the script if absent.
- Running the script twice overwrites the previous archive (idempotent).

## Key decisions

- Version sourced from `Cargo.toml` at run time (not hardcoded) so the script stays correct after a version bump.
- `zip` chosen over `tar.gz` for macOS to match platform conventions; `tar.gz` for Linux.
- No subdirectory inside the archive — files at archive root — to make extraction one step simpler.
