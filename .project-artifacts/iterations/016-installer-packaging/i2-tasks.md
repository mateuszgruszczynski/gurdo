# Iteration 16 Tasks — Installer packaging (EP-14)

## DEV tasks

- [x] T-1 Create `scripts/package.sh` — OS detection (`uname`), version extraction from `Cargo.toml`, `cargo build --release`, archive assembly (AC-1, AC-2, AC-3, AC-4, AC-5, AC-6)
- [x] T-2 `chmod +x scripts/package.sh` (AC-1)
- [x] T-3 Add `dist/` to `.gitignore` (keeps generated archives out of git)

## Cross-cutting

- [x] T-4 Verify `cargo test` still green after script addition (AC-7)

## Notes

- No Rust code changes; all work is a single shell file.
- Verification phase: run the script in the dev container (Linux path); macOS path validated by inspection + documented smoke steps.
