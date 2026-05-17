# Iteration 6 Development — Spotify API error suppression + status indicator (EP-17)

*Epic: EP-17 · Phase: Development · Date: 2026-05-12*

---

## Baseline

`cargo build` before changes: 53 warnings. Zero failures.

---

## Files changed

| File | Change |
|---|---|
| `src/ui/state.rs` | Added `api_error_snooze_until: Option<std::time::Instant>` |
| `src/ui/poll.rs` | Added `set_background_error` helper; used in `do_poll` + `extend_queue_if_needed`; cleared snooze on success in both branches |
| `src/ui/player.rs` | Split OK/Snooze buttons; added amber status indicator label |

---

## Tasks

| # | Status |
|---|---|
| T-1 | [x] |
| T-2 | [x] |
| T-3 | [x] |
| T-4 | [x] |
| T-5 | [x] |
| T-6 | [x] — 53 warnings |

---

## Key decisions

- `handle_cmd` error path left unchanged — user-triggered actions always surface modals.
- Snooze cleared on successful `do_poll` (both `None` and `Some` playing branches) so the indicator vanishes immediately when API recovers, not after the full 10-minute expiry.
- `std::time::Instant` works directly in `PlayerState` (`Clone + Send + Sync`); no wrapper needed.

## Commit: a9b3026
