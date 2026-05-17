# Iteration 11 Spec — Recommendation preview-while-tuning (EP-10)

*Epic: EP-10 · Phase: Refinement · Date: 2026-05-12*

---

## Problem

Tuning `Recommendations` and `Engine` knobs is guess-work: there is no way to see how
changing `artist_score_exponent` from 1.0 to 0.6 shifts the candidate list without
saving the config, restarting Spotify, and queuing tracks. EP-10 adds a **Preview**
button that runs the recommender with the current draft knob values and shows the
resulting (artist, track, score) list inline in the Settings window.

---

## Scope

### In scope

**`src/engine/recommend.rs`**
- Change `generate_recommendations` return type from `Vec<(String, String)>` to
  `Vec<(String, String, f64)>` — the f64 is the sampled artist's `final_score`.
- Update internal construction: when pushing to `results`, include
  `artists[artist_idx].1` as the score.

**`src/ui/poll.rs`**
- Update the two `generate_recommendations` call sites to destructure
  `(artist, track, _score)` instead of `(artist, track)`.

**`src/ui/state.rs`**
- Add `preview_results: Option<Vec<(String, String, f64)>>` field to
  `OperationsState` (default `None`).
- Add `Preview` variant to `OperationCommand`.

**`src/ui/ops.rs`**
- Handle `OperationCommand::Preview` in the dispatcher loop:
  - Does **not** set `active` (no progress indicator; operation is <100 ms).
  - Opens DB, reads draft-or-live config, calls `generate_recommendations`.
  - On `Ok(results)`: stores into `ops.preview_results = Some(results)`.
  - On `Err(e)`: stores `ops.last_result = Some(OperationResult::Failed(…))`
    and leaves `preview_results` unchanged.

**`src/ui/settings.rs`**
- In the Recommendations section, after the three existing knobs, add a
  **"Preview"** button (disabled when `busy`).
- When clicked: send `OperationCommand::Preview` through `ops_cmd_tx`.
  Pass the current draft config (if dirty) so the preview reflects unsaved
  knob changes.
- Preview panel: rendered below the button when `ops.preview_results` is
  `Some`. Bounded-height scroll area (≤ 300 px) with one row per result:
  `"{artist} — {track}"` left-aligned, score right-aligned as `"{score:.2}"`.
- The panel persists across frames until Preview is run again.
- Clearing the draft (Discard) also clears `preview_results` — see Notes.

### Out of scope

- Running Preview while another operation is active (button is disabled when `busy`).
- Persisting the preview as a saved playlist.
- Audio playback from the preview list.
- Cancellation of Preview mid-run.
- Clearing `preview_results` on Save (the saved config may differ from what was
  previewed; leave the stale results visible with a "(draft)" label until re-run).

---

## Acceptance criteria

| # | Criterion |
|---|---|
| AC-1 | "Preview" button appears in the Recommendations section; disabled while any operation is active. |
| AC-2 | Clicking Preview runs `generate_recommendations` with the current draft config (or live config if no draft). |
| AC-3 | Results render as a bounded-height scrollable list with artist, track, and score. |
| AC-4 | If the DB is empty or the engine returns `[]`, last_result shows an informative message ("No results — run Fetch Tracks first"). |
| AC-5 | Re-clicking Preview refreshes the list with the current knob values. |
| AC-6 | Discard clears preview_results (panel disappears). |
| AC-7 | `poll.rs` call sites compile and behave identically to before. |
| AC-8 | `cargo build` produces zero new warnings beyond the 53 baseline. |

---

## Implementation notes

### Return type change

```rust
// Before
pub fn generate_recommendations(conn: &Connection, config: &Config)
    -> Result<Vec<(String, String)>>

// After
pub fn generate_recommendations(conn: &Connection, config: &Config)
    -> Result<Vec<(String, String, f64)>>
```

Add the score when pushing to `results`:

```rust
results.push((artist_name.clone(), track_name.clone(), artists[artist_idx].1));
```

### `poll.rs` call site update

```rust
// Before
for (artist, track) in recs.iter().take(QUEUE_CHUNK_SIZE) { … }

// After
for (artist, track, _score) in recs.iter().take(QUEUE_CHUNK_SIZE) { … }
```

### `OperationsState`

```rust
#[derive(Clone, Default)]
pub struct OperationsState {
    pub active:          Option<ActiveOperation>,
    pub last_result:     Option<OperationResult>,
    pub preview_results: Option<Vec<(String, String, f64)>>,
}
```

### Dispatcher — Preview branch

```rust
OperationCommand::Preview => {
    let config = {
        let draft = shared_config_draft.lock().unwrap();   // ← see §Passing draft
        draft.clone().unwrap_or_else(|| shared_config.lock().unwrap().clone())
    };
    match db::open(&config.db_path())
        .and_then(|conn| generate_recommendations(&conn, &config))
    {
        Ok(results) if results.is_empty() => {
            ops.lock().unwrap().last_result = Some(OperationResult::Failed(
                "No results — run Fetch Tracks first".to_string(),
            ));
        }
        Ok(results) => {
            ops.lock().unwrap().preview_results = Some(results);
        }
        Err(e) => {
            ops.lock().unwrap().last_result = Some(OperationResult::Failed(
                format!("Preview failed: {}", e),
            ));
        }
    }
}
```

### Passing the draft config to the dispatcher

`OperationCommand::Preview` needs access to the draft config so the preview
reflects unsaved knob changes. The dispatcher already receives `shared_config:
Arc<Mutex<Config>>`. Add a second Arc for the draft:

```rust
pub async fn ops_dispatcher_loop(
    mut cmd_rx: UnboundedReceiver<OperationCommand>,
    ops: Arc<Mutex<OperationsState>>,
    shared_config: Arc<Mutex<Config>>,
    settings_draft: Arc<Mutex<Option<Config>>>,   // ← new
)
```

Call sites in `mod.rs` already have `settings_draft`; pass it through.

### Clearing preview on Discard

In `settings.rs`, the Discard button currently does:
```rust
*settings_draft.lock().unwrap() = None;
```

Add:
```rust
ops_state.lock().unwrap().preview_results = None;
```

### Preview panel layout

```
[Preview]

Artist — Track                                    0.84
Another Artist — Another Track                    0.51
...                                               (bounded 300px scroll)
```

---

## Files changed (expected)

| File | Change |
|------|--------|
| `src/engine/recommend.rs` | Return `Vec<(String, String, f64)>`; include score in push |
| `src/ui/poll.rs` | Destructure `(artist, track, _score)` at both call sites |
| `src/ui/state.rs` | Add `preview_results` to `OperationsState`; add `Preview` to `OperationCommand` |
| `src/ui/ops.rs` | Handle `Preview` in dispatcher; add `settings_draft` param; update loop call in `mod.rs` |
| `src/ui/settings.rs` | Preview button + results panel; clear preview on Discard |
| `src/ui/mod.rs` | Pass `settings_draft` to `ops_dispatcher_loop` |
