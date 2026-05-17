# Iteration 2 Spec — Embedded assets + CJK font fix (EP-3)

*Epic: EP-3 · Type: FIX · Priority: P1 · Size: S · Date: 2026-05-12*

---

## 1. Context and goal

CJK track titles (Japanese, Chinese, Korean) currently render as tofu rectangles in the Gurdo player window. The existing fix attempts to read a system font by probing a hard-coded list of OS-specific paths (`/System/Library/Fonts/PingFang.ttc`, `/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc`, `C:\Windows\Fonts\msyh.ttc`, etc.) at startup. This probe silently fails on any machine that doesn't have one of those exact paths — including the dev container and any fresh Linux install — leaving `egui` with no CJK coverage.

The fix embeds three Noto Sans `.otf` files (`NotoSansJP-Regular.otf`, `NotoSansSC-Regular.otf`, `NotoSansKR-Regular.otf`) directly into the compiled binary via `include_bytes!`, registers them as ordered fallbacks on the `Proportional` font family, and deletes the OS-path probe entirely. A programmatically-generated placeholder cover PNG is also embedded at this point (its bytes are needed by EP-5; displaying it is EP-5's concern, not this iteration's).

After this iteration the binary is fully self-contained: it carries its own CJK glyphs and requires no OS font installation on any supported platform.

The changed surface is small: one new module file (`src/ui/assets.rs`, currently a one-line placeholder) and a twelve-line replacement inside the `eframe::run_native` closure in `src/ui/mod.rs`. No other files require changes.

---

## 2. Scope

**In scope**

- Create the `assets/` directory tree with these committed files:
  - `assets/fonts/NotoSansJP-Regular.otf`
  - `assets/fonts/NotoSansSC-Regular.otf`
  - `assets/fonts/NotoSansKR-Regular.otf`
  - `assets/fonts/OFL.txt` (the Noto Sans Open Font License 1.1 text)
  - `assets/images/placeholder_cover.png` (programmatically generated 400×400 gray PNG, author-created, no external license required)
- Rewrite `src/ui/assets.rs` to expose four `pub const` byte slices: `NOTO_SANS_JP`, `NOTO_SANS_SC`, `NOTO_SANS_KR`, `PLACEHOLDER_COVER`.
- Replace the OS-font-path probe block in `src/ui/mod.rs` (lines 51–74) with the embedded-font registration block specified in Architecture §4.3.
- Confirm the font fallback order is JP → SC → KR (Japanese first, then Simplified Chinese, then Korean Hangul) appended to the existing `Proportional` family list.
- Document binary size growth in the commit message.

**Out of scope**

- Displaying the placeholder cover image in the player window — that is EP-5.
- Traditional Chinese (`NotoSansTC`) — that is EP-15, parked.
- Any changes to `Cargo.toml` — no new crate dependencies are needed.
- Any changes to `egui` widget code, `player.rs`, `poll.rs`, `state.rs`, or any file outside `src/ui/assets.rs` and `src/ui/mod.rs`.
- Font subsetting or size optimisation — embed the full `.otf` files.

---

## 3. Files to create / modify

| File | Action | Notes |
|---|---|---|
| `assets/fonts/NotoSansJP-Regular.otf` | **Create** | Download from Google Fonts / Noto project. ~5.8 MB. |
| `assets/fonts/NotoSansSC-Regular.otf` | **Create** | ~5.3 MB. |
| `assets/fonts/NotoSansKR-Regular.otf` | **Create** | ~5.2 MB. |
| `assets/fonts/OFL.txt` | **Create** | The full SIL Open Font License 1.1 text, as distributed with the Noto fonts. |
| `assets/images/placeholder_cover.png` | **Create** | 400×400, plain mid-gray (`#808080`) background. Author-created; no third-party license. Generate with any image tool or a short Rust/Python snippet. Keep under 5 KB (a flat PNG at this size compresses to ~1 KB). |
| `src/ui/assets.rs` | **Rewrite** | Replace the `// placeholder — EP-3` comment with four `pub const` declarations (see §6). |
| `src/ui/mod.rs` | **Modify** | Delete lines 51–74 (the `cjk_paths` array and `for` loop). Insert the embedded-font registration block in their place (see §6). |

No other files change.

---

## 4. Acceptance Criteria

**AC-1 — Font asset files are committed with their license**

After the change, the following files exist in the repository and are non-empty:
`assets/fonts/NotoSansJP-Regular.otf`, `assets/fonts/NotoSansSC-Regular.otf`, `assets/fonts/NotoSansKR-Regular.otf`, `assets/fonts/OFL.txt`.

*Verification:* `ls -lh assets/fonts/` shows all four files with sizes in the expected ranges (~5 MB each for the `.otf` files; several KB for `OFL.txt`).

*BDD scenario:*
```
Given the iteration has been applied
When I list the contents of assets/fonts/
Then I see NotoSansJP-Regular.otf, NotoSansSC-Regular.otf, NotoSansKR-Regular.otf, and OFL.txt
And each .otf file is larger than 4 MB
And OFL.txt is non-empty
```

---

**AC-2 — Placeholder cover asset is committed and license-clean**

`assets/images/placeholder_cover.png` exists, is a valid PNG, has dimensions of exactly 400×400 pixels, and is authored in-house (programmatically generated) with no third-party copyright claim.

*Verification:* `file assets/images/placeholder_cover.png` reports "PNG image data, 400 x 400". Commit message states "author-created via [tool]".

*BDD scenario:*
```
Given the iteration has been applied
When I inspect assets/images/placeholder_cover.png
Then it is a valid PNG file
And its dimensions are 400 × 400 pixels
And the commit message records its origin as programmatically generated
```

---

**AC-3 — `src/ui/assets.rs` exposes four public byte-slice constants**

The file `src/ui/assets.rs` declares:
- `pub const NOTO_SANS_JP: &[u8]`
- `pub const NOTO_SANS_SC: &[u8]`
- `pub const NOTO_SANS_KR: &[u8]`
- `pub const PLACEHOLDER_COVER: &[u8]`

Each is initialised with `include_bytes!` using an absolute-from-manifest path (`concat!(env!("CARGO_MANIFEST_DIR"), "/assets/…")`). The lengths of the slices at runtime equal the file sizes on disk.

*Verification:* `cargo build` succeeds; reading `src/ui/assets.rs` shows all four declarations. The build would fail at compile time if any referenced file were absent or the path were wrong.

*BDD scenario:*
```
Given the iteration has been applied
When I open src/ui/assets.rs
Then I see exactly four pub const declarations: NOTO_SANS_JP, NOTO_SANS_SC, NOTO_SANS_KR, PLACEHOLDER_COVER
And each is typed &[u8] and initialised with include_bytes!
And running cargo build succeeds, confirming the referenced files exist at the declared paths
```

---

**AC-4 — The OS-font-path probe block is deleted**

The `cjk_paths` array and the `for path in &cjk_paths` loop no longer appear anywhere in the codebase. No call to `std::fs::read` for a font path exists in any `src/ui/*.rs` file.

*Verification:* `grep -r "cjk_paths\|cjk_fallback\|PingFang\|msyh\|msgothic\|NotoSansCJK" src/` returns no output.

*BDD scenario:*
```
Given the iteration has been applied
When I search the source tree for cjk_paths, cjk_fallback, PingFang, msyh, msgothic, NotoSansCJK
Then no results are found in any file under src/
```

---

**AC-5 — Embedded fonts are registered in the correct fallback order**

In `src/ui/mod.rs`, the `FontDefinitions` setup inserts font data under the keys `"noto_sans_jp"`, `"noto_sans_sc"`, `"noto_sans_kr"` from the corresponding `assets` constants, and pushes them onto the `Proportional` family in the order JP, SC, KR — appended after the existing default fonts (i.e., Latin glyphs still resolve through the default `egui` font first).

*Verification:* Reading `src/ui/mod.rs` shows the three `fonts.font_data.insert` calls followed by three `proportional.push` calls in the stated order. No other font manipulation code exists in the file.

*BDD scenario:*
```
Given the iteration has been applied
When I read src/ui/mod.rs
Then I see font_data.insert calls for noto_sans_jp, noto_sans_sc, and noto_sans_kr sourced from ui::assets
And I see proportional.push calls in the order noto_sans_jp → noto_sans_sc → noto_sans_kr
And I see no cjk_paths array, no for-loop over paths, and no std::fs::read call
```

---

**AC-6 — Project builds with zero errors and zero new warnings**

`cargo build` (debug profile, default features) exits with status 0 after the change. No new `warning:` lines are introduced compared to the pre-iteration baseline. (Pre-existing warnings from other modules are not this iteration's responsibility, but this iteration must not add any.)

*Verification:* `cargo build 2>&1 | grep "^warning:"` shows no lines attributable to `src/ui/assets.rs` or `src/ui/mod.rs`.

*BDD scenario:*
```
Given the iteration has been applied
When I run cargo build
Then the build exits with status 0
And no warning lines reference assets.rs or mod.rs
```

---

**AC-7 — Japanese track titles render with visible glyphs at runtime**

When the app is launched on the host machine and a track with a Japanese title is playing on Spotify, the track name field in the player window displays recognisable Japanese characters — not tofu rectangles (□) or question marks.

*Verification:* Play a track with a Japanese title (e.g. "夜に駆ける" by YOASOBI). Observe the track name label in the Gurdo window. All CJK characters are legible.

*BDD scenario:*
```
Given the app has been built and launched on the host machine after the iteration is applied
And Spotify is playing a track whose title contains Japanese characters (e.g. 夜に駆ける)
When the player window updates with the current track info
Then the track name label shows recognisable Japanese glyphs
And no tofu rectangles or question marks appear in the track name or artist name
```

---

**AC-8 — Korean track titles render with visible Hangul glyphs**

When a track with a Korean title is playing, the track name displays Hangul characters, not placeholders.

*Verification:* Play a track with a Korean title (e.g. "강남스타일" by PSY). The player label shows Hangul glyphs.

*BDD scenario:*
```
Given the app is running and Spotify is playing a track with a Korean title (e.g. 강남스타일)
When the player window shows the current track
Then the track name label displays recognisable Hangul characters
And no tofu rectangles are visible
```

---

**AC-9 — Cyrillic track titles still render correctly (no regression)**

Cyrillic rendering was already working before this iteration. After the change, a track with a Cyrillic title continues to render correctly. This verifies that the new font registration does not displace or interfere with the existing Latin/Cyrillic coverage from the default `egui` fonts.

*Verification:* Play a track with a Cyrillic title (e.g. "Розы" by Maksim Fade). The player label shows the Cyrillic characters correctly — no regression.

*BDD scenario:*
```
Given the app is running and Spotify is playing a track with a Cyrillic title (e.g. Розы)
When the player window shows the current track
Then the track name label displays the Cyrillic characters correctly
And the rendering is identical to the pre-iteration behaviour
```

---

**AC-10 — Binary size grows by 15–17 MB and is documented**

The release binary size after `cargo build --release` is 15–17 MB larger than the pre-iteration baseline. The commit message records the before and after sizes (e.g. "Binary: 8.2 MB → 24.5 MB (+16.3 MB from three embedded Noto Sans OTF files)").

*Verification:* Compare `ls -lh target/release/gurdo` before and after. Check the commit message for the documented size delta.

*BDD scenario:*
```
Given a release build is produced before and after the iteration
When I compare the binary sizes
Then the post-iteration binary is 15–17 MB larger
And the commit message states the before/after sizes explicitly
```

---

## 5. Edge cases and failure modes

**`.ttc` vs `.otf` format.** The `egui` font stack uses `ab_glyph`, which does not reliably handle TrueType Collection (`.ttc`) files — font index 0 is selected silently and the wrong script may be loaded. The three Noto fonts must be downloaded as individual `.otf` files, not as `NotoSansCJK-Regular.ttc`. Confirm file type with `file assets/fonts/NotoSans*.otf` before committing — the output must say "OpenType font" not "TrueType font collection".

**Wrong Noto variant.** The Noto family includes `NotoSansCJKjp`, `NotoSansCJKsc`, `NotoSansCJKkr` (legacy multi-language packages) and the newer per-language `NotoSansJP`, `NotoSansSC`, `NotoSansKR`. Use the per-language `.otf` releases from the Noto GitHub (https://github.com/notofonts). The legacy `.ttc` packages will not work (see above).

**`include_bytes!` path resolution.** `include_bytes!` with a bare relative path resolves relative to the *source file's directory*, which is `src/ui/`. A path like `"../../assets/fonts/NotoSansJP-Regular.otf"` will work but is fragile if the module is moved. Use `concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/NotoSansJP-Regular.otf")` to resolve from the workspace root regardless of source file location. This is a compile-time constant and incurs no runtime cost.

**Compile-time missing file.** If an `assets/` file is absent when `cargo build` runs, the build fails with a clear error (`error: couldn't read …`). This is the desired behaviour — there is no silent fallback for a missing embedded asset.

**Font key collision.** If a future code path re-introduces a `"cjk_fallback"` key or attempts to insert `"noto_sans_jp"` a second time, `egui::FontDefinitions::font_data` is a `BTreeMap` and the second insert silently overwrites the first. Confirm there is exactly one `fonts.font_data.insert` per key in `mod.rs`.

**Proportional family ordering.** The three new font keys are *appended* to the `Proportional` family (after the default `egui` fonts). This means Latin and ASCII glyphs continue to resolve through the built-in `NotoSans` that `egui` ships — the CJK fonts are reached only when a codepoint is not found in an earlier font. Do not prepend the CJK fonts; that would slow down Latin rendering by forcing a miss on every common character.

**Placeholder cover size.** A 400×400 fully-saturated RGBA PNG can exceed 600 KB uncompressed. A flat single-color PNG at the same size compresses to under 1 KB. Use a flat gray (`#808080`) or a minimal geometric image. Verify `ls -lh assets/images/placeholder_cover.png` reports under 5 KB before committing.

**License of placeholder cover.** Any image sourced from a third party — even a "free" website — may carry a license incompatible with binary redistribution. Generate the placeholder programmatically (a short Rust or Python script using the `image` crate or Pillow) and document the generation command in the commit message. This eliminates all license ambiguity.

---

## 6. Notes for implementation

**Recommended order of edits**

1. Download the three `.otf` files and `OFL.txt`; place them under `assets/fonts/`. Verify file types with `file`.
2. Generate `assets/images/placeholder_cover.png` (script or image tool). Verify dimensions and size.
3. Rewrite `src/ui/assets.rs`.
4. Edit `src/ui/mod.rs`: delete the probe block, add `use crate::ui::assets;` if not already present (note: `assets` is a submodule of `ui`, so reference as `assets::NOTO_SANS_JP` directly), insert the replacement font setup block.
5. Run `cargo build`. Fix any compile errors before proceeding.
6. Launch `cargo run -- --config config.toml` on the host; play a CJK-titled track; confirm glyphs.
7. Run `cargo build --release`; record binary size in the commit message.

**`src/ui/assets.rs` replacement content**

```rust
pub const NOTO_SANS_JP: &[u8] = include_bytes!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/NotoSansJP-Regular.otf"));

pub const NOTO_SANS_SC: &[u8] = include_bytes!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/NotoSansSC-Regular.otf"));

pub const NOTO_SANS_KR: &[u8] = include_bytes!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/NotoSansKR-Regular.otf"));

pub const PLACEHOLDER_COVER: &[u8] = include_bytes!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/assets/images/placeholder_cover.png"));
```

No `use` statements, no helper functions, no `struct` or `impl` blocks. The file is constants only.

**`src/ui/mod.rs` replacement block (lines 51–75)**

Replace the entire `let mut fonts … cc.egui_ctx.set_fonts(fonts);` block with:

```rust
let mut fonts = egui::FontDefinitions::default();

fonts.font_data.insert("noto_sans_jp".into(),
    egui::FontData::from_static(assets::NOTO_SANS_JP));
fonts.font_data.insert("noto_sans_sc".into(),
    egui::FontData::from_static(assets::NOTO_SANS_SC));
fonts.font_data.insert("noto_sans_kr".into(),
    egui::FontData::from_static(assets::NOTO_SANS_KR));

let proportional = fonts.families
    .entry(egui::FontFamily::Proportional)
    .or_default();
proportional.push("noto_sans_jp".into());
proportional.push("noto_sans_sc".into());
proportional.push("noto_sans_kr".into());

cc.egui_ctx.set_fonts(fonts);
```

`assets` is already accessible as a submodule of `ui` (declared `mod assets;` in `mod.rs` line 1). No additional `use` import is needed.

**Generating the placeholder PNG**

A minimal Python one-liner using Pillow:
```
python3 -c "from PIL import Image; Image.new('RGB', (400,400), (128,128,128)).save('assets/images/placeholder_cover.png')"
```

Or using the `image` crate in a throwaway Rust snippet. Either approach produces a flat gray 400×400 PNG under 1 KB. Include the generation command in the commit message.

**Noto Sans download source**

Use the per-language releases from https://github.com/notofonts/noto-fonts/releases or the individual font package pages on Google Fonts. Verify the downloaded files:
- `file NotoSansJP-Regular.otf` → "OpenType font data"
- `otfinfo -i NotoSansJP-Regular.otf` (if `lcdf-typetools` available) → confirms the correct script is embedded

The OFL.txt file is included with every Noto download; commit the copy distributed alongside the fonts you download to ensure version consistency.

**Commit message structure**

```
EP-3: embed Noto Sans CJK fonts + placeholder cover

- Add assets/fonts/NotoSansJP/SC/KR-Regular.otf (OFL 1.1)
- Add assets/fonts/OFL.txt
- Add assets/images/placeholder_cover.png (author-created, 400×400 gray)
- Rewrite src/ui/assets.rs: NOTO_SANS_JP/SC/KR, PLACEHOLDER_COVER constants
- Replace OS-font-path probe in src/ui/mod.rs with embedded font registration
  (JP → SC → KR fallback order on Proportional family)

Fixes: CJK tofu rectangles on machines without system CJK fonts.
Binary: <before> MB → <after> MB (+~16 MB from three embedded OTF files)
Placeholder cover display: deferred to EP-5.
```
