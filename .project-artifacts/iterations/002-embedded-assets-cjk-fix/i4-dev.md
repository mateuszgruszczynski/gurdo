# Iteration 2 Dev — Embedded assets + CJK font fix (EP-3)

*Epic: EP-3 · Phase: Development · Date: 2026-05-12*

---

## Files changed

| File | Action | Notes |
|---|---|---|
| `assets/fonts/NotoSansJP-Regular.otf` | Created | 4.4 MB, OpenType-CFF (OTTO magic), OFL 1.1 |
| `assets/fonts/NotoSansSC-Regular.otf` | Created | 8.0 MB, OpenType-CFF (OTTO magic), OFL 1.1 |
| `assets/fonts/NotoSansKR-Regular.otf` | Created | 4.5 MB, OpenType-CFF (OTTO magic), OFL 1.1 |
| `assets/fonts/OFL.txt` | Created | SIL Open Font License 1.1, extracted from noto-cjk Sans2.004 release |
| `assets/images/placeholder_cover.png` | Created | 400×400 gray #808080, 1 KB, author-created via Python stdlib |
| `src/ui/assets.rs` | Rewritten | 4 `pub const` byte slices via `include_bytes!` + `CARGO_MANIFEST_DIR` |
| `src/ui/mod.rs` | Modified | OS-probe block deleted; embedded font registration inserted |

---

## In-process tests run

No automated test suite exists yet (EP-12 is Test scaffolding, future iteration). The following T-plan scenarios were verified manually in-process:

| Scenario | AC | Result |
|---|---|---|
| T-01 — OTF files present, OpenType-CFF format | AC-1 | PASS — Python magic-byte check confirmed `OTTO` for all three |
| T-02 — PNG 400×400, <5 KB | AC-2 | PASS — 400×400, 1,051 bytes |
| T-03 — `assets.rs` has 4 typed constants | AC-3 | PASS — manual review |
| T-04 — `cargo build` exits 0 | AC-3, AC-6 | PASS — 14.69 s, no errors |
| T-05 — probe symbols absent | AC-4 | PASS — `grep -r "cjk_paths\|..."` returns no output |
| T-06 — fallback order in `mod.rs` | AC-5 | PASS — manual review: 3 inserts + 3 pushes JP→SC→KR |
| T-07 — no new warnings in `assets.rs` or `mod.rs` | AC-6 | PASS — `grep "warning:.*assets\|warning:.*mod"` returns no output |
| T-11 — release binary delta 15–17 MB | AC-10 | PASS — 24 MB → 41 MB (+17 MB) |

E2E/UI scenarios (T-08 Japanese, T-09 Korean, T-10 Cyrillic, T-12 Latin regression) are Verification-phase — require launching the app on the host display.

---

## External interfaces wired for Verification

The Gurdo player is a native egui/eframe desktop application. There is no HTTP/gRPC interface to stub. Verification requires launching the binary on the host machine with a live Spotify session.

Steps for Verification smoke test:
1. `cargo build --release` inside the container (already done — 41 MB binary at `target/release/gurdo`)
2. Run on host: `cargo run -- ui -c config.toml` (or copy the binary and run directly)
3. Play tracks with Japanese, Korean, Cyrillic, and Latin titles in sequence

---

## Key decisions

**Font download source:** `notofonts/noto-cjk` GitHub release `Sans2.004`, per-language packages `16_NotoSansJP.zip`, `17_NotoSansKR.zip`, `18_NotoSansSC.zip`. The noto-fonts monorepo no longer contains per-language OTF files at the paths referenced in the spec; the authoritative source moved to noto-cjk.

**Placeholder PNG generation:** Python standard library (`zlib` + `struct`), not Pillow (not available in the container) or ImageMagick (not installed). A minimal PNG writer produces a valid 400×400 single-color PNG at 1 KB. Generation command documented in commit message.

**`#[allow(dead_code)]` on `PLACEHOLDER_COVER`:** The constant is intentionally unused until EP-5. Since `assets` is a private submodule of `ui`, rustc emits a dead_code warning for any unused pub constant inside it. The annotation suppresses one new warning that would otherwise violate AC-6. A brief inline comment documents the deferral reason.

---

## Deviations from spec

**None.** All AC-1 through AC-10 are addressable. T-08/T-09/T-10/T-12 (runtime CJK/Cyrillic rendering) are deferred to Verification as specified.

---

## Self-review checklist

- [x] No new security surface introduced (no network calls, no user-controlled paths)
- [x] No new `unsafe` code
- [x] License obligation satisfied: OFL.txt committed alongside the font files
- [x] `include_bytes!` paths use `CARGO_MANIFEST_DIR` (not fragile relative paths)
- [x] Font keys are appended to Proportional family, not prepended (Latin perf preserved)
- [x] No changes outside the spec-declared file set (no other `src/` files touched)
- [x] Binary size delta documented in commit message
- [x] Placeholder cover origin documented in commit message (no third-party license)
