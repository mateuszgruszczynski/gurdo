# Spec: Replace behavioral knobs with contextual level selectors

## 1. Goal

Replace eight `DragValue` knobs in the Settings window with horizontal contextual level selectors that map plain-English labels to fixed numeric presets. Remove the `max_tracks_per_seed` knob entirely (the field is defined in config but never read by the engine). The underlying `Config` struct fields and `config.toml` format are unchanged; the selectors are purely a presentation layer.

---

## 2. Before / After

### Before — DragValue knob
```
Artist score exponent  [1.00 ↕]  [↺]
```

### After — level selector
```
Artist variety
[ Max variety ] [ More variety ] [ ● Balanced ] [ Favour favs ] [ Top artists ]
```

A label above a horizontal row of `selectable_label` buttons. The active preset is highlighted. No reset button — the default is always one of the levels.

---

## 3. Scope

### In scope
- Replace the 8 knobs listed in section 4 with level selectors.
- Remove the `max_tracks_per_seed` knob from the UI (field kept in `config.rs` and `config.toml`; not read by engine).
- Add `knob_level_f64` and `knob_level_u32` helpers in `src/ui/settings.rs`.
- Keep existing `knob_f64` / `knob_u32` helpers for all other knobs.
- No changes to `Config`, `config.toml` schema, or any engine/scoring code.

### Out of scope
- Implementing `max_tracks_per_seed` in the engine (separate future epic).
- Changing remaining numeric knobs (counts, limits, thresholds, pool size).
- Persisting the selected level index — always derived from the stored numeric value.

---

## 4. The 8 selector specs

### 4.1 Playcount factor
**Config field:** `artist_scoring.score_exponent` | **Section:** Artist Scoring
**Hover text:** "Controls how much your most-played artists pull ahead of ones you only occasionally revisit."
**Levels: 3**

| # | Label | Preset | Default |
|---|-------|--------|---------|
| 1 | Minimal | 0.1 | |
| 2 | Slightly favour top played | 0.3 | D |
| 3 | Favour top played | 0.5 | |

---

### 4.2 Artist variety
**Config field:** `recommendations.artist_score_exponent` | **Section:** Recommendations
**Hover text:** "Controls whether your queue is spread across many artists or dominated by the ones you play most."
**Levels: 5**

| # | Label | Preset | Default |
|---|-------|--------|---------|
| 1 | Max variety | 0.3 | |
| 2 | More variety | 0.6 | |
| 3 | Balanced | 1.0 | D |
| 4 | Favour favourites | 1.5 | |
| 5 | Top artists only | 2.5 | |

---

### 4.3 Track variety
**Config field:** `recommendations.track_rank_exponent` | **Section:** Recommendations
**Hover text:** "Controls whether the queue sticks to each artist's biggest hits or explores deeper cuts and B-sides."
**Levels: 5**

| # | Label | Preset | Default |
|---|-------|--------|---------|
| 1 | Deep cuts welcome | 0.3 | |
| 2 | More B-sides | 0.6 | |
| 3 | Balanced | 1.0 | D |
| 4 | Mostly big hits | 1.5 | |
| 5 | Hits only | 2.5 | |

---

### 4.4 Similar artist influence
**Config field:** `engine.similarity_multiplier` | **Section:** Engine
**Hover text:** "Controls how strongly artists similar to your favourites are pulled into recommendations."
**Levels: 5**

| # | Label | Preset | Default |
|---|-------|--------|---------|
| 1 | Stick to listened | 0.1 | |
| 2 | Slight exploration | 0.25 | |
| 3 | Balanced | 0.5 | D |
| 4 | More similar artists | 1.0 | |
| 5 | Explore widely | 1.5 | |

---

### 4.5 Multi-source boost
**Config field:** `engine.multi_source_bonus_pct` | **Section:** Engine
**Hover text:** "Rewards artists that appear as a recommendation from several of your favourites at once."
**Levels: 5**

| # | Label | Preset | Default |
|---|-------|--------|---------|
| 1 | No consensus boost | 0.0 | |
| 2 | Subtle boost | 0.03 | |
| 3 | Moderate boost | 0.05 | D |
| 4 | Strong boost | 0.10 | |
| 5 | Heavy consensus bias | 0.20 | |

---

### 4.6 Loved-track bonus
**Config field:** `engine.like_bonus_flat` | **Section:** Engine
**Hover text:** "How much a Last.fm loved track pushes that artist higher in your recommendations."
**Levels: 5**

| # | Label | Preset | Default |
|---|-------|--------|---------|
| 1 | Loves ignored | 0.0 | |
| 2 | Gentle nudge | 2.0 | |
| 3 | Noticeable boost | 5.0 | D |
| 4 | Strong preference | 15.0 | |
| 5 | Loved artists first | 30.0 | |

---

### 4.7 Dislike penalty
**Config field:** `engine.dislike_modifier_pct` | **Section:** Engine
**Hover text:** "How hard a single disliked track drops an artist in your recommendations."
**Levels: 5**

| # | Label | Preset | Default |
|---|-------|--------|---------|
| 1 | Dislikes ignored | 0.0 | |
| 2 | Mild penalty | 0.05 | |
| 3 | Moderate penalty | 0.10 | D |
| 4 | Heavy penalty | 0.20 | |
| 5 | Near-excluded | 0.50 | |

---

### 4.8 Years-active bonus
**Config field:** `artist_scoring.year_bonus_pct` | **Section:** Artist Scoring
**Hover text:** "Rewards artists you have kept coming back to across many years of listening history."
**Levels: 5**

| # | Label | Preset | Default |
|---|-------|--------|---------|
| 1 | No loyalty bonus | 0.0 | |
| 2 | Small loyalty bonus | 2.0 | |
| 3 | Moderate bonus | 5.0 | D |
| 4 | Strong loyalty bonus | 10.0 | |
| 5 | Longevity first | 20.0 | |

---

## 5. Acceptance criteria

| # | Criterion |
|---|-----------|
| AC-1 | Each of the 8 converted knobs renders as a label line above a horizontal row of buttons (3 for Playcount factor, 5 for all others). |
| AC-2 | The button corresponding to the closest preset to the current config value is highlighted as selected. |
| AC-3 | Clicking a button writes the exact preset value to the config field and marks the settings draft as dirty. |
| AC-4 | On initial load, the default level is pre-selected for each selector. |
| AC-5 | If `config.toml` contains a value not in the preset list, the closest preset is selected without error. |
| AC-6 | Hovering the knob name shows the hover-text description. |
| AC-7 | Save / Discard flow is unchanged — selector changes participate identically to the existing `any_changed` mechanism. |
| AC-8 | No `DragValue` or reset (↺) button is shown for the 8 converted knobs. |
| AC-9 | `max_tracks_per_seed` knob is absent from the Settings window. |
| AC-10 | All other knobs continue to use `DragValue` and are unaffected. |
| AC-11 | `cargo build` ≤ 2 pre-existing warnings; `cargo test` 16/16 green. |

---

## 6. Implementation notes

### 6.1 Helper signatures

```rust
fn knob_level_f64(ui: &mut egui::Ui, label: &str, desc: &str,
                  value: &mut f64, labels: &[&str], presets: &[f64]) -> bool {
    let active = closest_f64(*value, presets);
    let mut changed = false;
    ui.label(label).on_hover_text(desc);
    ui.horizontal(|ui| {
        for (i, btn_label) in labels.iter().enumerate() {
            if ui.selectable_label(active == i, *btn_label).clicked() && active != i {
                *value = presets[i];
                changed = true;
            }
        }
    });
    changed
}
// equivalent knob_level_u32 replacing f64 with u32 throughout
```

Slices (not fixed-size arrays) allow the same helper to serve both 3-level and 5-level selectors.

### 6.2 Level-detection

```rust
fn closest_f64(value: f64, presets: &[f64]) -> usize {
    presets.iter().enumerate()
        .min_by(|(_, a), (_, b)|
            (value - *a).abs().partial_cmp(&(value - *b).abs()).unwrap())
        .map(|(i, _)| i)
        .unwrap_or(0)
}

fn closest_u32(value: u32, presets: &[u32]) -> usize {
    presets.iter().enumerate()
        .min_by_key(|(_, &p)| (value as i64 - p as i64).unsigned_abs())
        .map(|(i, _)| i)
        .unwrap_or(0)
}
```

### 6.3 egui layout
- Label on its own line: `ui.label(...).on_hover_text(...)`.
- `ui.horizontal(|ui| { ... })` for the button row.
- `ui.selectable_label(selected, text)` — built-in highlight, no custom styling needed.

---

## 7. Files affected

| File | Change |
|------|--------|
| `src/ui/settings.rs` | Add `knob_level_f64` / `closest_f64` / `closest_u32` helpers; replace 8 call sites; remove `max_tracks_per_seed` knob |
