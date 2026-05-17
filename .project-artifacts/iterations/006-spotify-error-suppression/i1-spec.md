# Iteration 6 Spec — Spotify API error suppression + status indicator (EP-17)

*Epic: EP-17 · Phase: Refinement · Date: 2026-05-12*

---

## Problem

The polling loop runs every 5 seconds. During Spotify API downtime every cycle
sets `state.error`, surfacing a blocking modal. The user dismisses it, 5 seconds
pass, it reappears — indefinitely. The app becomes unusable during outages.

---

## Scope

### In scope

1. **`src/ui/state.rs`** — add `api_error_snooze_until: Option<std::time::Instant>`
   to `PlayerState`. `Instant` is `Clone + Send + Sync` so no wrapper needed.

2. **`src/ui/poll.rs`** — background error paths (`do_poll`,
   `extend_queue_if_needed`) check the snooze before writing to `state.error`.
   On any **successful** `do_poll` response, clear `api_error_snooze_until` so
   the indicator disappears immediately when the API recovers.
   Explicit user-action errors (`handle_cmd`) are **not** suppressed — the user
   just asked for something and deserves to know it failed.

3. **`src/ui/player.rs`** — two changes:
   - Error modal: replace the single `"OK" | "Ignore"` row with `"OK"` (dismiss
     once, same as today) and `"Snooze 10 min"` (sets snooze + clears error).
   - Status indicator: when `api_error_snooze_until` is `Some` and still in the
     future, render a small amber `⚠ Spotify API unavailable` label below the
     artist name (non-blocking, no modal).

### Out of scope

- Per-endpoint error categorisation or retry back-off.
- Suppressing `handle_cmd` errors (explicit user actions always show the modal).
- Configurable snooze duration.

---

## Acceptance criteria

| # | Criterion |
|---|---|
| AC-1 | After clicking "Snooze 10 min", no new error modals appear for 10 minutes from the click time. |
| AC-2 | A small `⚠ Spotify API unavailable` label appears in the player while snoozed. |
| AC-3 | The label disappears immediately when a successful `do_poll` response is received (API recovered). |
| AC-4 | "OK" dismiss behaviour is unchanged — next poll failure shows the modal again. |
| AC-5 | Explicit user-action errors (play/pause, like, queue start) still surface the modal regardless of snooze. |
| AC-6 | `cargo build` produces zero new warnings beyond the 53 pre-existing baseline. |

---

## Implementation notes

### `src/ui/state.rs`

```rust
#[derive(Clone, Default)]
pub struct PlayerState {
    // ... existing fields ...
    pub api_error_snooze_until: Option<std::time::Instant>,
}
```

`Option<Instant>` implements `Default` as `None`. ✓

### `src/ui/poll.rs`

Helper used in `do_poll` and `extend_queue_if_needed` error paths:
```rust
fn set_background_error(state: &Arc<Mutex<PlayerState>>, msg: String) {
    let mut s = state.lock().unwrap();
    let snoozed = s.api_error_snooze_until
        .map(|t| t > std::time::Instant::now())
        .unwrap_or(false);
    if !snoozed {
        s.error = Some(msg);
    }
}
```

In `do_poll` success paths (both `None` and `Some(playing)` branches), clear the
snooze so the indicator goes away on recovery:
```rust
s.api_error_snooze_until = None;
```

Replace the two direct `state.lock().unwrap().error = Some(e.to_string())` calls
in `do_poll` and `extend_queue_if_needed` with `set_background_error(&state, e.to_string())`.
`handle_cmd`'s error path is left unchanged.

### `src/ui/player.rs`

Error modal button row (currently one `if clicked()` with `||`):
```rust
if ui.button("OK").clicked() {
    self.state.lock().unwrap().error = None;
}
if ui.button("Snooze 10 min").clicked() {
    let mut s = self.state.lock().unwrap();
    s.api_error_snooze_until = Some(
        std::time::Instant::now() + std::time::Duration::from_secs(600)
    );
    s.error = None;
}
```

Status indicator (after artist name label, before progress bar):
```rust
if state.api_error_snooze_until
    .map(|t| t > std::time::Instant::now())
    .unwrap_or(false)
{
    ui.label(egui::RichText::new("⚠ Spotify API unavailable")
        .color(egui::Color32::from_rgb(255, 180, 0))
        .size(11.0));
}
```

---

## Files changed (expected)

| File | Change |
|---|---|
| `src/ui/state.rs` | Add `api_error_snooze_until` field |
| `src/ui/poll.rs` | Add `set_background_error` helper; use it in background error paths; clear snooze on success |
| `src/ui/player.rs` | Split OK/Snooze buttons; add status indicator label |

No new Cargo dependencies.
