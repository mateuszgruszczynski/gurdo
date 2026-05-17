# Iteration 8 Spec — Full config-knob exposure (EP-8)

*Epic: EP-8 · Phase: Refinement · Date: 2026-05-12*

---

## Problem

The Settings window (EP-6) has five placeholder sections: Recommendations, Engine, Artist
Scoring, Sync, and Appearance. The 21 tunable config fields (across `[sync]`, `[engine]`,
`[artist_scoring]`, `[recommendations]`) are invisible from the UI — users must hand-edit
`config.toml`. Additionally, `TRACKS_PER_ARTIST = 50` in `sync/expand.rs` ignores the
`config.engine.artist_top_tracks_limit` field it was meant to reflect. This epic fills all
five placeholder sections and fixes the constant.

---

## Scope

### In scope

**Draft config management**
- `GurdoApp` gains `settings_draft: Arc<Mutex<Option<Config>>>` field (initialized to `None`).
- When Settings opens, the closure also captures `Arc::clone(&shared_config)` and
  `Arc::clone(&settings_draft)` and the `config_path: PathBuf`.
- Settings render reads display values from `draft` (if `Some`) else `shared_config`.
- Any knob change: if draft is `None`, clone `shared_config` into draft, then apply the change.
- `dirty = settings_draft.lock().unwrap().is_some()`
- **Save:** write draft to `config_path` via `Config::save`; replace `shared_config` with draft;
  set draft to `None`. Remove `#[allow(dead_code)]` on `Config::save` and `config_path`.
- **Discard changes:** set draft to `None`.
- `spotify_connected` computed from `shared_config` / `config.token_path().exists()` (unchanged).

**`src/ui/knobs.rs`**
- `KnobSpec` struct: `{ field: &'static str, label: &'static str, description: &'static str }`
- Four static slices: `SYNC_KNOBS`, `ENGINE_KNOBS`, `ARTIST_SCORING_KNOBS`, `RECOMMEND_KNOBS`.
- Each entry covers one config field; description shown as a tooltip on hover.
- Widget type derived from the Rust type: `f64` → `DragValue` with `speed(0.001)` + range;
  `u32`/`u64`/`usize` → `DragValue` with `speed(1.0)` + integer range.

**Knob fields and metadata**

*Sync* (`[sync]`):
| Field | Label | Range | Default |
|---|---|---|---|
| `sync.loved_tracks_limit` | Loved tracks limit | 50–5000 | 500 |
| `sync.seed_artists_limit` | Seed artists limit | 10–500 | 50 |
| `sync.seed_tracks_limit` | Seed tracks limit | 10–500 | 50 |

*Engine* (`[engine]`):
| Field | Label | Range | Default |
|---|---|---|---|
| `engine.similar_artists_limit` | Similar artists per seed | 5–100 | 20 |
| `engine.artist_top_tracks_limit` | Tracks per artist | 5–200 | 10 |
| `engine.recommendation_pool_size` | Recommendation pool size | 50–2000 | 200 |
| `engine.max_tracks_per_seed` | Max tracks per seed artist | 1–100 | 20 |
| `engine.similarity_multiplier` | Similarity multiplier | 0.01–2.0 | 0.5 |
| `engine.multi_source_bonus_pct` | Multi-source bonus | 0.0–0.5 | 0.05 |
| `engine.like_bonus_flat` | Like bonus (flat) | 0.0–50.0 | 5.0 |
| `engine.dislike_modifier_pct` | Dislike penalty (%) | 0.0–1.0 | 0.10 |

*Artist Scoring* (`[artist_scoring]`):
| Field | Label | Range | Default |
|---|---|---|---|
| `artist_scoring.score_exponent` | Playcount score exponent | 0.1–2.0 | 0.301 |
| `artist_scoring.year_bonus_pct` | Year active bonus (%) | 0.0–50.0 | 5.0 |
| `artist_scoring.min_playcount_threshold` | Min playcount threshold | 1–500 | 40 |

*Recommendations* (`[recommendations]`):
| Field | Label | Range | Default |
|---|---|---|---|
| `recommendations.count` | Number of recommendations | 5–500 | 50 |
| `recommendations.artist_score_exponent` | Artist score exponent | 0.1–5.0 | 1.0 |
| `recommendations.track_rank_exponent` | Track rank exponent | 0.1–5.0 | 1.0 |

**`src/ui/settings.rs`** — render changes
- `render` gains `shared_config: &Arc<Mutex<Config>>`, `settings_draft: &Arc<Mutex<Option<Config>>>`,
  `config_path: &std::path::Path` parameters (removing the `spotify_connected` bool — compute inline).
- Each section calls a helper `knob_section(ui, title, knobs, draft_config, default_config)`.
- `knob_section` renders: heading + separator; for each knob a horizontal row with
  `DragValue` widget + tooltip + Reset button (resets that field to default); Save / Discard
  buttons below the last section when dirty.
- The Save button is `add_enabled_ui(dirty)`.
- A dirty indicator: a `•` before the Save button label.

**Appearance section** (read-only display)
- Shows key paths and read-only fields: `data_dir`, `db_path`, `config_path`, `token_path`,
  `lastfm.username`, `spotify.client_id` (truncated), `ui.player_window_size`,
  `ui.settings_window_size`. No edit controls.

**`src/sync/expand.rs`**
- Delete `const TRACKS_PER_ARTIST: u32 = 50;`.
- Replace `TRACKS_PER_ARTIST` usage with a `tracks_per_artist` parameter. The function
  already receives `config`... wait, `fetch_artist_tracks` does **not** receive `Config` —
  only `conn`, `client`, `sample`. Add `config: &Config` parameter; read
  `config.engine.artist_top_tracks_limit`.
- Update callsite in `ops.rs` and in `main.rs`.

**`src/config.rs`**
- `impl Default for EngineConfig` — needed for knob Reset defaults.
- `impl Default for SyncConfig` — same.
- Both already have individual `default_*` fns; `Default` just calls them.

### Out of scope

- Recommendation preview panel (EP-10)
- Secrets hardening / removing keys from config.toml (EP-11)
- `like_modifier_pct` and `similar_tracks_limit` / `tag_top_tracks_limit` fields — these are
  present in config but not consumed by any active code path; leave as read-only in
  Appearance or omit entirely to avoid exposing dead knobs (decision: omit).

---

## Acceptance criteria

| # | Criterion |
|---|---|
| AC-1 | All 16 exposed knob fields are editable from Settings; changes apply to the next operation that reads them. |
| AC-2 | Save writes modified values to `config.toml` and updates `shared_config` in-process. |
| AC-3 | Discard returns all knobs to the last-saved values without writing to disk. |
| AC-4 | Reset button next to each knob restores that field to its compiled default. |
| AC-5 | Save button is disabled (greyed) when no knobs have been changed. |
| AC-6 | Appearance section shows data_dir, db_path, config_path, token_path, lastfm.username, spotify.client_id (first 8 chars + …). |
| AC-7 | `TRACKS_PER_ARTIST` constant is deleted; `fetch_artist_tracks` reads `config.engine.artist_top_tracks_limit`. |
| AC-8 | `cargo build` produces zero new warnings beyond the 53 pre-existing baseline (remove `#[allow(dead_code)]` from `Config::save` and `config_path`). |

---

## Implementation notes

### Draft wiring in `mod.rs` and `player.rs`

```rust
// mod.rs — GurdoApp init:
settings_draft: Arc::new(Mutex::new(None)),
config_path,   // move config_path into GurdoApp (currently stored but dead)
```

```rust
// player.rs — settings viewport closure:
let shared_config  = Arc::clone(&self.shared_config);
let settings_draft = Arc::clone(&self.settings_draft);
let config_path    = self.config_path.clone();
ctx.show_viewport_deferred(..., move |ctx, _class| {
    ...
    super::settings::render(ctx, &settings_open, &ops_state, &ops_cmd_tx,
                            &shared_config, &settings_draft, &config_path);
});
```

### Knob row layout

```rust
fn knob_row(ui: &mut egui::Ui, label: &str, desc: &str,
            value: &mut f64, min: f64, max: f64, default: f64) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label(label).on_hover_text(desc);
        if ui.add(egui::DragValue::new(value).speed(0.001).range(min..=max)).changed() {
            changed = true;
        }
        if ui.small_button("↺").on_hover_text("Reset to default").clicked() {
            *value = default;
            changed = true;
        }
    });
    changed
}
```
(Variant for `u32`/`u64`/`usize` uses `speed(1.0)` and integer cast.)

### Save / Discard button block

```rust
if dirty {
    ui.add_space(8.0);
    ui.separator();
    ui.horizontal(|ui| {
        if ui.button("• Save").clicked() {
            let draft = settings_draft.lock().unwrap().take().unwrap();
            draft.save(&config_path).unwrap_or_else(|e| tracing::error!("Save failed: {}", e));
            *shared_config.lock().unwrap() = draft;
        }
        if ui.button("Discard changes").clicked() {
            *settings_draft.lock().unwrap() = None;
        }
    });
}
```

### `fetch_artist_tracks` signature change

```rust
pub async fn fetch_artist_tracks(
    conn: &Connection,
    client: &LastfmClient,
    sample: Option<usize>,
    config: &Config,
    progress: &dyn ProgressReporter,
) -> Result<()>
```

Callsites: `ops.rs` (add `&config`), `main.rs` (add `&config`).

---

## Files changed (expected)

| File | Change |
|---|---|
| `src/config.rs` | `impl Default for EngineConfig`, `impl Default for SyncConfig`; remove `#[allow(dead_code)]` from `Config::save` |
| `src/ui/state.rs` | No change |
| `src/ui/knobs.rs` | `KnobSpec` struct; four static slices |
| `src/ui/mod.rs` | Add `settings_draft` to `GurdoApp` init |
| `src/ui/player.rs` | Add `settings_draft` field; update settings closure; remove `#[allow(dead_code)]` from `config_path` |
| `src/ui/settings.rs` | Revised `render` signature; fill all five sections; knob/Save/Discard/Reset helpers |
| `src/sync/expand.rs` | Delete `TRACKS_PER_ARTIST`; add `config: &Config` param to `fetch_artist_tracks` |
| `src/ui/ops.rs` | Pass `&config` to `fetch_artist_tracks` call |
| `src/main.rs` | Pass `&config` to `fetch_artist_tracks` call |
