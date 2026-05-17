# Iteration 7 Spec — In-process operations + progress (EP-7)

*Epic: EP-7 · Phase: Refinement · Date: 2026-05-12*

---

## Problem

The four data-pipeline operations (Sync Last.fm, Expand similar artists, Fetch top
tracks, Recalculate scores) and Spotify login are currently unreachable from the UI.
The old Settings modal had `spawn_cli` subprocess buttons (deleted in EP-6). EP-7
replaces them with in-process calls running on a dedicated tokio task, with live
progress visible in the Settings window.

---

## Scope

### In scope

**Infrastructure (new types in `src/ui/state.rs`)**
- `OperationsState { active: Option<ActiveOperation>, last_result: Option<OperationResult> }`
- `ActiveOperation { kind, stage, current, total: Option<u64>, message }`
- `OperationKind` enum: `SyncLastfm | Expand | FetchTracks | Score | SpotifyLogin`
- `OperationResult::Ok(String) | Failed(String)`
- `OperationCommand::Run(OperationKind)`

**`src/ui/ops.rs`**
- `ProgressReporter` trait: `stage(&str)`, `tick(u64, Option<u64>)`, `message(&str)`, `finish(bool, &str)`
- `StateReporter` — impl that writes directly to `Arc<Mutex<OperationsState>>`; no channel needed
- `ops_dispatcher_loop(cmd_rx, ops, shared_config)` — tokio task; one operation at a time; sets `active` on start, clears on finish and writes `last_result`

**Sync/engine functions — add `progress: &dyn ProgressReporter` parameter**
- `sync::sync_lastfm` — emit: stage("Loved tracks"), tick per track; stage("Top tags"); stage("Artist history"), tick per year; stage("Scoring external artists"); finish
- `sync::expand_artists` — emit: stage("Fetching similar artists"), tick per artist; finish
- `sync::fetch_artist_tracks` — emit: stage("Fetching top tracks"), tick per artist; finish
- `engine::artist_scores::score_artists` — emit: stage("Recalculating scores"); finish
- `spotify::auth::run_oauth_flow` is **not** modified (no progress parameter); the dispatcher wraps it with manual stage/finish calls

**`src/ui/settings.rs`**
- `render` gains two new parameters: `ops: &Arc<Mutex<OperationsState>>`, `ops_cmd_tx: &UnboundedSender<OperationCommand>`
- Data section: 4 operation buttons (disabled when `active.is_some()`); live progress panel when `active.is_some()`; last-result line when `active.is_none() && last_result.is_some()`
- Spotify section: Login button (disabled when active); connection status line
- While `active.is_some()`, settings viewport calls `ctx.request_repaint_after(100ms)` for live updates

**`src/ui/mod.rs`**
- Create `Arc<Mutex<OperationsState>>` and `(ops_cmd_tx, ops_cmd_rx)` channel
- Run `ops_dispatcher_loop` alongside `polling_loop` via `tokio::join!` in the background thread
- Pass `ops_state` and `ops_cmd_tx` to `GurdoApp`; forward to `settings::render`

### Out of scope

- Combining operations into a single "Update everything" action (EP-9)
- Cancelling in-flight operations
- `TRACKS_PER_ARTIST` constant replacement (EP-8)

---

## Acceptance criteria

| # | Criterion |
|---|---|
| AC-1 | Clicking a Data button starts the corresponding operation in-process; progress (stage + count) updates live in the Settings window. |
| AC-2 | While any operation is running, all 5 buttons (4 Data + Login) are disabled. |
| AC-3 | On completion, `last_result` is shown: "✓ <summary>" or "✗ <error>". |
| AC-4 | On failure, the error is shown in the Settings window; the modal flood suppression from EP-17 is unaffected (ops errors appear in Settings, not as player modals). |
| AC-5 | Spotify Login button runs `run_oauth_flow`; on success the status line shows "Connected as \<username\>" read from the token; on failure shows the error. |
| AC-6 | Playback polling continues normally while an operation runs (separate tokio tasks). |
| AC-7 | `cargo build` produces zero new warnings beyond the 53 pre-existing baseline. |

---

## Implementation notes

### Progress emission points (per function)

`sync_lastfm`:
```
stage("Syncing loved tracks")  → tick(i, total) per track
stage("Syncing top tags")      → tick(i, total) per tag
stage("Syncing artist history") → tick(year_idx, total_years) per year batch
stage("Scoring external artists")
finish(ok, "N artists, M loved tracks")
```

`expand_artists`:
```
stage("Fetching similar artists") → tick(i, total) per source artist
finish(ok, "N similar artists fetched, M cached")
```

`fetch_artist_tracks`:
```
stage("Fetching top tracks") → tick(i, total) per artist
finish(ok, "N tracks stored")
```

`score_artists`:
```
stage("Recalculating scores")
finish(ok, "N artists scored")
```

### `StateReporter`

```rust
pub struct StateReporter {
    ops: Arc<Mutex<OperationsState>>,
}

impl ProgressReporter for StateReporter {
    fn stage(&self, name: &str) {
        if let Some(a) = &mut self.ops.lock().unwrap().active {
            a.stage = name.to_string();
            a.current = 0;
            a.total = None;
        }
    }
    fn tick(&self, current: u64, total: Option<u64>) {
        if let Some(a) = &mut self.ops.lock().unwrap().active {
            a.current = current;
            a.total = total;
        }
    }
    fn message(&self, msg: &str) {
        if let Some(a) = &mut self.ops.lock().unwrap().active {
            a.message = msg.to_string();
        }
    }
    fn finish(&self, _ok: bool, _summary: &str) {} // handled by dispatcher
}
```

### `ops_dispatcher_loop`

```rust
pub async fn ops_dispatcher_loop(
    mut cmd_rx: UnboundedReceiver<OperationCommand>,
    ops: Arc<Mutex<OperationsState>>,
    shared_config: Arc<Mutex<Config>>,
) {
    while let Some(OperationCommand::Run(kind)) = cmd_rx.recv().await {
        ops.lock().unwrap().active = Some(ActiveOperation { kind, ... });
        let reporter = StateReporter { ops: Arc::clone(&ops) };
        let config = shared_config.lock().unwrap().clone();
        let result = run_operation(kind, &config, &reporter).await;
        let mut o = ops.lock().unwrap();
        o.active = None;
        o.last_result = Some(match result {
            Ok(summary) => OperationResult::Ok(summary),
            Err(e)      => OperationResult::Failed(e.to_string()),
        });
    }
}
```

### Settings Data section rendering

```rust
let ops = ops_state.lock().unwrap().clone();
let busy = ops.active.is_some();

ui.add_enabled_ui(!busy, |ui| {
    if ui.button("Sync Last.fm").clicked()     { let _ = ops_cmd_tx.send(OperationCommand::Run(OperationKind::SyncLastfm)); }
    if ui.button("Expand").clicked()           { let _ = ops_cmd_tx.send(OperationCommand::Run(OperationKind::Expand)); }
    if ui.button("Fetch Tracks").clicked()     { let _ = ops_cmd_tx.send(OperationCommand::Run(OperationKind::FetchTracks)); }
    if ui.button("Recalculate Scores").clicked() { let _ = ops_cmd_tx.send(OperationCommand::Run(OperationKind::Score)); }
});

if let Some(active) = &ops.active {
    ui.label(format!("{}: {}", active.kind.label(), active.stage));
    if let Some(total) = active.total {
        ui.label(format!("{}/{}", active.current, total));
    } else if active.current > 0 {
        ui.label(format!("{}", active.current));
    }
}

if let Some(result) = &ops.last_result {
    match result {
        OperationResult::Ok(s)     => ui.label(format!("✓ {}", s)),
        OperationResult::Failed(s) => ui.label(egui::RichText::new(format!("✗ {}", s)).color(egui::Color32::RED)),
    };
}

if busy { ctx.request_repaint_after(Duration::from_millis(100)); }
```

### Spotify Login status

After `run_oauth_flow` succeeds, read the stored token to get the username:
```rust
OperationKind::SpotifyLogin => {
    reporter.stage("Waiting for browser authorization");
    spotify::auth::run_oauth_flow(&config).await
        .map(|_| {
            // read username from token file if available, else generic success
            "Authenticated".to_string()
        })
}
```

The Spotify section shows a "Login" button + a status line. On first launch (no token),
status is "Not connected". After login, status is "Connected" (token exists on disk).
A `token_exists` helper in `ops` checks `config.token_path().exists()`.

---

## Files changed (expected)

| File | Change |
|---|---|
| `src/ui/state.rs` | Add `OperationsState`, `ActiveOperation`, `OperationKind`, `OperationResult`, `OperationCommand` |
| `src/ui/ops.rs` | Full implementation: trait, `StateReporter`, `ops_dispatcher_loop`, `run_operation` |
| `src/ui/mod.rs` | Create ops channel + state; run dispatcher; pass to `GurdoApp` |
| `src/ui/player.rs` | Add `ops_state` + `ops_cmd_tx` fields; forward to `settings::render` |
| `src/ui/settings.rs` | Add `ops` + `ops_cmd_tx` params; fill Data + Spotify sections |
| `src/sync/mod.rs` | Add `progress: &dyn ProgressReporter` to `sync_lastfm`; emit events |
| `src/sync/expand.rs` | Add `progress` to `expand_artists` + `fetch_artist_tracks`; emit events |
| `src/engine/artist_scores.rs` | Add `progress` to `score_artists`; emit events |
