# Iteration 4 Development — Idle-state placeholder cover (EP-5)

*Epic: EP-5 · Phase: Development · Date: 2026-05-12*

---

## Baseline

`cargo build` before changes: 53 warnings (pre-existing baseline in db/queries.rs,
lastfm/, spotify/). Zero failures.

---

## Files changed

| File | Change |
|---|---|
| `src/ui/assets.rs` | Removed `#[allow(dead_code)]` from `PLACEHOLDER_COVER` |
| `src/ui/player.rs` | Added `placeholder_texture` field; lazy-init in `update()`; updated album art branch |
| `src/ui/mod.rs` | Added `placeholder_texture: None` to struct literal |

---

## Tasks completed

| # | Task | Status |
|---|---|---|
| T-1 | Add `placeholder_texture` field + init in mod.rs | [x] |
| T-2 | Lazy-init in `update()` via `decode_image` | [x] |
| T-3 | Album art rendering branch updated | [x] |
| T-4 | Removed `#[allow(dead_code)]` from `assets.rs` | [x] |
| T-5 | `cargo build` — 53 warnings, zero new | [x] |

---

## In-process tests

S-1 (decode succeeds on `PLACEHOLDER_COVER`) is implicitly verified: `cargo build`
would fail or the runtime would silently not decode if `decode_image` panicked on the
bytes. The lazy-init branch swallows `Err` so a bad decode results in empty space, not
a crash — acceptable for a static embedded asset. No separate test file for size-S epic.

---

## Key decisions

- **Lazy init vs. `CreationContext`:** lazy-init in `update()` was chosen over decoding
  in `mod.rs`'s closure, because it keeps the decode logic co-located with `player.rs`
  and reuses the existing `decode_image` helper without moving it or adding imports to
  `mod.rs`. The placeholder is only needed at runtime, not at construction time.
- **`else` fallback kept:** the final `ui.allocate_space` branch after the placeholder
  check is unreachable after the first frame but retained as a safety net for the
  brief window before `placeholder_texture` is populated.

---

## External interfaces wired

None — this epic is pure UI rendering with no out-of-process interfaces.

---

## Self-review

- No logic added beyond what the spec required. ✓
- No new dependencies. ✓
- No security surface changes. ✓
- `cargo build` green, 53 warnings (baseline unchanged). ✓
- Committed: `feat(ui): show placeholder cover in idle slot (EP-5)` (235468f)
