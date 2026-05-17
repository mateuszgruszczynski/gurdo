# Iteration 16 Test Plan — Installer packaging (EP-14)

## Scenarios

### S-1 Linux archive produced (System-integration / File-batch)
**Given** the dev container (Linux aarch64)
**When** `./scripts/package.sh` is run
**Then** `dist/gurdo-0.1.0-linux-aarch64.tar.gz` exists

Covers: AC-2, AC-5

---

### S-2 Archive contains exactly binary + OFL.txt (System-integration / File-batch)
**Given** S-1 has completed
**When** the tar listing is inspected (`tar -tzf dist/gurdo-0.1.0-linux-aarch64.tar.gz`)
**Then** output is exactly two lines: `gurdo` and `OFL.txt` (order irrelevant)

Covers: AC-4

---

### S-3 Binary is executable (System-integration / File-batch)
**Given** S-1 has completed and the archive is extracted
**When** `file gurdo` is run on the extracted binary
**Then** output contains `ELF` (Linux) and the binary has execute permission

Covers: AC-2 (runtime validity)

---

### S-4 Build failure → non-zero exit, no archive (System-integration / File-batch)
**Given** a broken build (temporarily introduce a syntax error in `src/main.rs`)
**When** `./scripts/package.sh` is run
**Then** exit code is non-zero AND `dist/` contains no new archive

Covers: AC-6

---

### S-5 Script is executable (File-batch)
**Given** the repository as committed
**When** `test -x scripts/package.sh`
**Then** exit 0

Covers: AC-1

---

### S-6 `cargo test` green (regression)
**Given** the script has been added
**When** `cargo test` is run
**Then** all 16 tests pass, no new failures

Covers: AC-7

---

### S-7 macOS archive name (inspection)
**Given** the script source
**When** the macOS branch is read
**Then** it produces `dist/gurdo-<version>-macos-<arch>.zip` and calls `zip -j`

Covers: AC-3, AC-5 (macOS branch; cannot run in dev container — validated by code inspection)

---

## Level / type assignments

| Scenario | Level | Type | Phase |
|----------|-------|------|-------|
| S-1 | System-integration | File-batch | Verification |
| S-2 | System-integration | File-batch | Verification |
| S-3 | System-integration | File-batch | Verification |
| S-4 | System-integration | File-batch | Verification |
| S-5 | Component | File-batch | Development |
| S-6 | Component | CLI | Development |
| S-7 | Component | File-batch | Development (inspection) |

## AC coverage

| AC | Scenario(s) |
|----|-------------|
| AC-1 | S-5 |
| AC-2 | S-1, S-3 |
| AC-3 | S-7 (inspection) |
| AC-4 | S-2 |
| AC-5 | S-1, S-7 |
| AC-6 | S-4 |
| AC-7 | S-6 |
