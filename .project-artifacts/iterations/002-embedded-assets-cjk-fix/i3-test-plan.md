# Iteration 2 Test Plan — Embedded assets + CJK font fix (EP-3)

*Epic: EP-3 · Phase: Test Plan · Date: 2026-05-12*

---

## Scenario summary

| ID | Scenario | Covers AC | Level | Type | Owned by |
|---|---|---|---|---|---|
| T-01 | Font OTF files and OFL license are present and correctly formatted | AC-1 | Component | File-batch | Development |
| T-02 | Placeholder cover PNG exists with correct dimensions and size | AC-2 | Component | File-batch | Development |
| T-03 | `assets.rs` exposes exactly four typed byte-slice constants via `include_bytes!` | AC-3 | Component | File-batch | Development |
| T-04 | Cargo build succeeds — compile-time proof that all four asset paths resolve | AC-3, AC-6 | Component | CLI | Development |
| T-05 | OS-font-path probe is completely absent from the source tree | AC-4 | Component | File-batch | Development |
| T-06 | Embedded fonts are inserted and pushed in the declared fallback order | AC-5 | Component | File-batch | Development |
| T-07 | No new warnings attributable to `assets.rs` or `mod.rs` after the change | AC-6 | Component | CLI | Development |
| T-08 | Japanese track title renders as legible glyphs on the host | AC-7 | E2E | UI | Verification |
| T-09 | Korean track title renders as legible Hangul glyphs on the host | AC-8 | E2E | UI | Verification |
| T-10 | Cyrillic track title still renders correctly — no regression | AC-9 | E2E | UI | Verification |
| T-11 | Release binary grows 15–17 MB and the delta is documented in the commit | AC-10 | Component | CLI | Development |
| T-12 | Latin/ASCII track titles render correctly after font-setup change (regression) | AC-9 | E2E | UI | Verification |

---

## Scenarios

---

### T-01 — Font OTF files and OFL license are present and correctly formatted

**Covers AC:** AC-1  
**Level:** Component  
**Type:** File-batch  
**Owned by:** Development

```
Given the iteration has been applied
When I list the contents of assets/fonts/ and run `file` on each .otf
Then NotoSansJP-Regular.otf, NotoSansSC-Regular.otf, NotoSansKR-Regular.otf, and OFL.txt are all present
And each .otf file is reported by `file` as "OpenType font data" (not "TrueType font collection")
And each .otf file is larger than 4 MB
And OFL.txt is non-empty
```

**Notes:** The `file` output distinguishing "OpenType font data" from "TrueType font collection" is the definitive check that `.otf` single-language files were downloaded rather than the `.ttc` multi-language packages, which would silently fail at runtime.

---

### T-02 — Placeholder cover PNG exists with correct dimensions and size

**Covers AC:** AC-2  
**Level:** Component  
**Type:** File-batch  
**Owned by:** Development

```
Given the iteration has been applied
When I run `file assets/images/placeholder_cover.png` and `ls -lh assets/images/placeholder_cover.png`
Then `file` reports "PNG image data, 400 x 400"
And `ls -lh` shows the file is under 5 KB
And the commit message records the generation command used to produce the file
```

**Notes:** The commit message must document the generation command (e.g., `python3 -c "from PIL import Image; ..."`), confirming author-created origin and removing all third-party license ambiguity.

---

### T-03 — `assets.rs` exposes exactly four typed byte-slice constants via `include_bytes!`

**Covers AC:** AC-3  
**Level:** Component  
**Type:** File-batch  
**Owned by:** Development

```
Given the iteration has been applied
When I read the full contents of src/ui/assets.rs
Then the file contains exactly four pub const declarations:
  NOTO_SANS_JP, NOTO_SANS_SC, NOTO_SANS_KR, and PLACEHOLDER_COVER
And each is typed &[u8]
And each is initialised with include_bytes! using a concat!(env!("CARGO_MANIFEST_DIR"), "/assets/...") path
And the file contains no use statements, no helper functions, and no other items
```

**Notes:** Confirming `CARGO_MANIFEST_DIR`-anchored paths (not bare relative paths) guards against future source-file relocations silently breaking compile-time resolution.

---

### T-04 — Cargo build succeeds — compile-time proof that all four asset paths resolve

**Covers AC:** AC-3, AC-6  
**Level:** Component  
**Type:** CLI  
**Owned by:** Development

```
Given the iteration has been applied and T-01 and T-02 have passed
When I run `cargo build` in the project root
Then the command exits with status 0
And no error lines reference missing files in assets/fonts/ or assets/images/
```

**Notes:** `include_bytes!` is a compile-time macro. A missing file produces a hard error; a wrong path produces a hard error. Exit 0 is therefore direct proof all four asset paths resolve. This scenario cannot pass if T-01 or T-02 fail.

---

### T-05 — OS-font-path probe is completely absent from the source tree

**Covers AC:** AC-4  
**Level:** Component  
**Type:** File-batch  
**Owned by:** Development

```
Given the iteration has been applied
When I search src/ for the symbols and strings that belonged to the old probe block
  (cjk_paths, cjk_fallback, PingFang, msyh, msgothic, NotoSansCJK)
Then no matches are found in any file under src/
```

**Verification command:** `grep -r "cjk_paths\|cjk_fallback\|PingFang\|msyh\|msgothic\|NotoSansCJK" src/` returns no output.

---

### T-06 — Embedded fonts are inserted and pushed in the declared fallback order

**Covers AC:** AC-5  
**Level:** Component  
**Type:** File-batch  
**Owned by:** Development

```
Given the iteration has been applied
When I read src/ui/mod.rs
Then I see exactly three font_data.insert calls: noto_sans_jp, noto_sans_sc, noto_sans_kr
  sourced from the assets submodule constants
And I see exactly three proportional.push calls in the order:
  noto_sans_jp → noto_sans_sc → noto_sans_kr
And no other font manipulation code (no cjk_paths array, no for-loop, no std::fs::read for a font) exists in the file
And the CJK keys are appended after the default proportional family entries (not prepended)
```

**Notes:** Append-not-prepend matters for Latin/ASCII render performance: the default egui `NotoSans` is reached first for every common character, and the CJK fonts only when a codepoint is not found earlier.

---

### T-07 — No new warnings attributable to `assets.rs` or `mod.rs` after the change

**Covers AC:** AC-6  
**Level:** Component  
**Type:** CLI  
**Owned by:** Development

```
Given a clean cargo build completes after the iteration is applied
When I filter the build output for warning lines referencing assets.rs or mod.rs
Then no such lines appear
```

**Verification command:** `cargo build 2>&1 | grep "^warning:.*assets\|^warning:.*mod"` returns no output.

**Notes:** Pre-existing warnings from unrelated modules (db/queries.rs, lastfm/, spotify/) are out of scope for this iteration. This scenario asserts only that EP-3 introduces no new warnings.

---

### T-08 — Japanese track title renders as legible glyphs on the host

**Covers AC:** AC-7  
**Level:** E2E  
**Type:** UI  
**Owned by:** Verification

```
Given the release or debug binary has been built inside the container
And the binary has been launched on the host with `cargo run -- ui -c config.toml`
And Spotify is playing a track whose title contains Japanese characters (e.g. 夜に駆ける by YOASOBI)
When the player window updates with the current track information
Then the track name label displays recognisable Japanese kanji and kana glyphs
And no tofu rectangles (□) or question marks appear in the track name or artist name fields
```

**Notes:** Manual inspection — cannot be automated inside the dev container because egui requires a GPU/display context. Confirm with at least one track whose title contains kanji (CJK ideographs). Hiragana/katakana confirmation is a bonus.

---

### T-09 — Korean track title renders as legible Hangul glyphs on the host

**Covers AC:** AC-8  
**Level:** E2E  
**Type:** UI  
**Owned by:** Verification

```
Given the app is running on the host after the iteration is applied
And Spotify is playing a track whose title contains Korean Hangul characters (e.g. 강남스타일 by PSY)
When the player window shows the current track
Then the track name label displays recognisable Hangul syllable blocks
And no tofu rectangles are visible in the track name or artist name fields
```

---

### T-10 — Cyrillic track title still renders correctly — no regression

**Covers AC:** AC-9  
**Level:** E2E  
**Type:** UI  
**Owned by:** Verification

```
Given the app is running on the host after the iteration is applied
And Spotify is playing a track whose title contains Cyrillic characters (e.g. Розы by Maksim Fade)
When the player window shows the current track
Then the track name label displays the Cyrillic characters correctly
And the rendering is visually identical to the pre-iteration behaviour
```

**Notes:** Cyrillic was already working before EP-3 because it is covered by the default egui `NotoSans`. This regression check confirms the new CJK font insertions do not displace or interfere with the existing Proportional family coverage.

---

### T-11 — Release binary grows 15–17 MB and the delta is documented in the commit

**Covers AC:** AC-10  
**Level:** Component  
**Type:** CLI  
**Owned by:** Development

```
Given a release build has been produced before and after the iteration is applied
When I compare the sizes of the release binaries using `ls -lh target/release/gurdo`
Then the post-iteration binary is 15–17 MB larger than the pre-iteration binary
And the commit message states the before and after sizes explicitly
  (e.g. "Binary: 8.2 MB → 24.5 MB (+16.3 MB from three embedded Noto Sans OTF files)")
```

**Notes:** The expected delta (~16 MB) comes from embedding three ~5 MB OTF files (the placeholder PNG at <1 KB is negligible). A delta outside the 15–17 MB range warrants investigation — it may indicate the wrong font files were downloaded (e.g. `.ttc` instead of per-language `.otf`, or variable-font variants).

---

### T-12 — Latin/ASCII track titles render correctly after font-setup change (regression)

**Covers AC:** AC-9 (scope: general rendering regression)  
**Level:** E2E  
**Type:** UI  
**Owned by:** Verification

```
Given the app is running on the host after the iteration is applied
And Spotify is playing a track whose title contains only Latin/ASCII characters
When the player window shows the current track
Then the track name, artist name, and album name all display correctly
And no characters are missing or replaced with rectangles
```

**Notes:** Latin rendering relies on the default egui `NotoSans` being reached first in the Proportional family before the appended CJK fonts. This scenario detects any accidental prepend (which would force a CJK-font miss for every Latin glyph before falling back to the correct font). Can be checked simultaneously with T-10 if the Cyrillic track also has a Latin artist name.

---

## Coverage table

| AC | Covered by | Level | Phase |
|---|---|---|---|
| AC-1 | T-01 | Component | Development |
| AC-2 | T-02 | Component | Development |
| AC-3 | T-03, T-04 | Component | Development |
| AC-4 | T-05 | Component | Development |
| AC-5 | T-06 | Component | Development |
| AC-6 | T-04, T-07 | Component | Development |
| AC-7 | T-08 | E2E | Verification |
| AC-8 | T-09 | E2E | Verification |
| AC-9 | T-10, T-12 | E2E | Verification |
| AC-10 | T-11 | Component | Development |

All 10 ACs have at least one scenario. AC-7, AC-8, AC-9 (runtime rendering) are E2E/UI owned by Verification because they have no in-process observable — the only way to verify glyph rendering is to run the application on a display host.

---

## Notes on Verification automation

AC-7, AC-8, AC-9, and AC-12 are manual scenarios. The Gurdo player is a native egui/eframe desktop application that requires a GPU/display context. It cannot run headlessly inside the dev container. The Verification phase will document these as manual smoke tests performed on the host machine, with visual confirmation recorded in `i5-verify.md`.
