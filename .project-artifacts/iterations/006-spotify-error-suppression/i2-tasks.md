# Iteration 6 Tasks — Spotify API error suppression + status indicator (EP-17)

*Epic: EP-17 · Phase: Decomposition · Date: 2026-05-12*

| # | Task | AC(s) | Status |
|---|---|---|---|
| T-1 | `state.rs`: add `api_error_snooze_until: Option<std::time::Instant>` to `PlayerState` | AC-1, AC-2 | [ ] |
| T-2 | `poll.rs`: add `set_background_error` helper with snooze check; replace direct error writes in `do_poll` and `extend_queue_if_needed` | AC-1, AC-4, AC-5 | [ ] |
| T-3 | `poll.rs`: clear `api_error_snooze_until` in both success branches of `do_poll` | AC-3 | [ ] |
| T-4 | `player.rs`: split error modal buttons into "OK" + "Snooze 10 min" | AC-1, AC-4 | [ ] |
| T-5 | `player.rs`: add amber status indicator label below artist name | AC-2, AC-3 | [ ] |
| T-6 | `cargo build` — confirm zero new warnings (baseline: 53) | AC-6 | [ ] |
