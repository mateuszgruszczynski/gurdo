# Iteration 2 Tasks — Embedded assets + CJK font fix (EP-3)

*Epic: EP-3 · Phase: Decomposition · Date: 2026-05-12*

---

## Task list

| # | Title | Role | Depends on | Maps to AC |
|---|---|---|---|---|
| T-1 | Download Noto Sans OTF files + OFL.txt | DEV | — | AC-1 |
| T-2 | Generate placeholder_cover.png | DEV | — | AC-2 |
| T-3 | Rewrite `src/ui/assets.rs` | DEV | T-1, T-2 | AC-3 |
| T-4 | Replace OS-probe loop in `src/ui/mod.rs` | DEV | T-3 | AC-4, AC-5 |
| T-5 | Verify clean build | DEV | T-4 | AC-6 |
| T-6 | Manual smoke test on host | DEV | T-5 | AC-7, AC-8, AC-9, AC-10 |

T-1 and T-2 are independent and can run in parallel.

---

## Task details

### T-1 — Download Noto Sans OTF files + OFL.txt
**Role:** DEV (SECURITY: verify OFL license)
**Depends on:** —

Download from the Noto fonts GitHub (https://github.com/notofonts/noto-fonts) or Google Fonts:
- `NotoSansJP-Regular.otf`
- `NotoSansSC-Regular.otf`
- `NotoSansKR-Regular.otf`
- `OFL.txt` (the SIL Open Font License 1.1 text shipped with the fonts)

Place all four files under `assets/fonts/`. Verify each `.otf` with `file` — output must say "OpenType font data", not "TrueType font collection". Confirm `OFL.txt` is non-empty.

License check: OFL 1.1 permits binary redistribution (embedding in compiled binaries). No additional conditions apply beyond the license notice already satisfied by committing `OFL.txt`.

**Done when:** `ls -lh assets/fonts/` shows 4 files, each OTF > 4 MB, `file` confirms OpenType format.

---

### T-2 — Generate `assets/images/placeholder_cover.png`
**Role:** DEV (DESIGN: confirm adequate appearance for EP-5 preview)
**Depends on:** —

Generate a 400×400 plain gray (`#808080`) PNG programmatically. The file must be:
- Author-created (no third-party copyright)
- Under 5 KB (flat gray compresses to ~1 KB)
- Exactly 400×400 pixels

Recommended one-liner (Python + Pillow — already available in the dev container):
```
python3 -c "from PIL import Image; Image.new('RGB', (400,400), (128,128,128)).save('assets/images/placeholder_cover.png')"
```

If Pillow is unavailable, use the `image` crate in a throwaway Rust snippet, or any image tool that produces a valid PNG.

**Done when:** `file assets/images/placeholder_cover.png` reports "PNG image data, 400 x 400"; `ls -lh` shows under 5 KB.

---

### T-3 — Rewrite `src/ui/assets.rs`
**Role:** DEV
**Depends on:** T-1, T-2

Replace the placeholder comment with four `pub const` declarations using `include_bytes!` anchored to `CARGO_MANIFEST_DIR`:

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

No `use` statements, no helper functions, no other items.

**Done when:** `cargo build` succeeds (compile-time proof that all four paths resolve).

---

### T-4 — Replace OS-probe loop in `src/ui/mod.rs`
**Role:** DEV
**Depends on:** T-3

Delete the entire block from `let mut fonts = egui::FontDefinitions::default();` through `cc.egui_ctx.set_fonts(fonts);` (the `cjk_paths` array and `for` loop), and replace with:

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

`assets` is already a declared submodule of `mod.rs` — no new `use` import needed.

**Done when:** `grep -r "cjk_paths\|cjk_fallback\|PingFang\|msyh\|msgothic\|NotoSansCJK" src/` returns no output.

---

### T-5 — Verify clean build
**Role:** DEV
**Depends on:** T-4

Run `cargo build` (debug profile). Confirm exit code 0 and no new warnings in `assets.rs` or `mod.rs`.

**Done when:** `cargo build 2>&1 | grep "^warning:.*assets\|^warning:.*mod"` returns no output.

---

### T-6 — Manual smoke test + release build
**Role:** DEV
**Depends on:** T-5

1. Run `cargo build --release`. Record pre/post binary size delta in the commit message.
2. Launch `cargo run -- ui -c config.toml` on the host.
3. Play a track with a Japanese title — confirm glyphs render (AC-7).
4. Play a track with a Korean title — confirm Hangul glyphs render (AC-8).
5. Play a track with a Cyrillic title — confirm no regression (AC-9).

**Done when:** All three script checks pass; binary size delta ~15–17 MB documented in commit.

---

## Auto-continue assessment

All tasks map to ACs. DESIGN concern (placeholder appearance) and SECURITY concern (license verification) are absorbed into T-2 and T-1 respectively — no separate role tasks needed for a S-size epic. No scope added beyond spec. **Auto-continue condition met → proceeding to Test Plan.**
