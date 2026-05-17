# Test Plan — Behavioral knobs 5-level selectors

## Scenarios

### S-1 Default values select correct level (Component / UI — code inspection)
**Given** a freshly loaded config with all default values
**When** each of the 8 selectors is inspected
**Then** `closest_f64` / `closest_u32` returns the index of the default preset for each knob:
- Playcount factor → index 1 (0.301 closest to 0.3)
- Artist variety → index 2 (1.0)
- Track variety → index 2 (1.0)
- Similar artist influence → index 2 (0.5)
- Multi-source boost → index 2 (0.05)
- Loved-track bonus → index 2 (5.0)
- Dislike penalty → index 2 (0.10)
- Years-active bonus → index 2 (5.0)

Covers: AC-4

---

### S-2 Closest-preset logic for off-preset values (Unit)
**Given** `closest_f64` called with a value between two presets
**When** the value is closer to one preset than another
**Then** the nearer index is returned (e.g. 0.8 with presets [0.3, 0.6, 1.0] → index 2)

Covers: AC-5

---

### S-3 `cargo build` green (Component / CLI)
**Given** all changes applied
**When** `cargo build` is run
**Then** exit 0, ≤ 2 warnings (both pre-existing)

Covers: AC-11

---

### S-4 `cargo test` green (Component / CLI)
**Given** all changes applied
**When** `cargo test` is run
**Then** 16/16 pass

Covers: AC-11

---

### S-5 No DragValue or reset button for converted knobs (Component / UI — code inspection)
**Given** `src/ui/settings.rs`
**When** the 8 converted knob names are searched
**Then** none of them appear as arguments to `knob_f64` or `knob_u32`; all appear in `knob_level_f64` calls

Covers: AC-8

---

### S-6 `max_tracks_per_seed` absent from settings (Component / UI — code inspection)
**Given** `src/ui/settings.rs`
**When** `grep max_tracks_per_seed src/ui/settings.rs` is run
**Then** no matches

Covers: AC-9

---

### S-7 Remaining numeric knobs unaffected (Component / UI — code inspection)
**Given** `src/ui/settings.rs`
**When** the still-numeric knob fields are searched (count, similar_artists_limit, artist_top_tracks_limit, recommendation_pool_size, min_playcount_threshold, loved_tracks_limit, seed_artists_limit, seed_tracks_limit)
**Then** each still appears as an argument to `knob_f64` or `knob_u32`

Covers: AC-10

---

## Regression scenarios

### S-8 `any_changed` flag still set on level click (Component / UI — code inspection)
**Given** `knob_level_f64` implementation
**When** it returns `true`
**Then** the call site does `.then(|| any_changed = true)` — same pattern as existing knobs

Covers: AC-7

---

## Level / type assignments

| Scenario | Level | Type | Phase |
|----------|-------|------|-------|
| S-1 | Unit | UI (inspection) | Development |
| S-2 | Unit | UI | Development |
| S-3 | Component | CLI | Development |
| S-4 | Component | CLI | Development |
| S-5 | Component | UI (inspection) | Development |
| S-6 | Component | UI (inspection) | Development |
| S-7 | Component | UI (inspection) | Development |
| S-8 | Component | UI (inspection) | Development |

All scenarios are in-process. No out-of-process scenarios — change is pure UI widget replacement.

## AC coverage

| AC | Scenario(s) |
|----|-------------|
| AC-1 | S-5 (label above row), S-3 |
| AC-2 | S-1, S-2 |
| AC-3 | S-8, S-3 |
| AC-4 | S-1 |
| AC-5 | S-2 |
| AC-6 | S-5 (on_hover_text present in call) |
| AC-7 | S-8 |
| AC-8 | S-5 |
| AC-9 | S-6 |
| AC-10 | S-7 |
| AC-11 | S-3, S-4 |
