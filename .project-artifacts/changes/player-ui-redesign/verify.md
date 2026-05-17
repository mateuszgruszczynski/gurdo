# Verification — Player UI polish: consistent ghost-style controls

## Environment

Dev container, Linux aarch64. In-process scenarios verified from source and build output.
E2E/UI scenarios require the host machine with a display and a running Spotify session — noted
where applicable.

## In-process scenario results

| Scenario | Check | Result |
|----------|-------|--------|
| S-1 | `inactive.weak_bg_fill` = TRANSPARENT; `inactive.bg_fill` = TRANSPARENT; `hovered.weak_bg_fill` = `rgba(255,255,255,20)`; `active.weak_bg_fill` = `rgba(255,255,255,40)` | PASS (source) |
| S-2 | `extreme_bg_color` = `rgba(255,255,255,25)`; bar fill = `rgba(255,255,255,160)` | PASS (source) |
| S-3 | `cargo build` → 1 warning (pre-existing `last_track_uri`), no new warnings | PASS |
| S-4 | `cargo test` → 16/16 | PASS |

## E2E/UI scenario results

The dev container has no display. These scenarios are deferred to manual smoke in Integration.

| Scenario | Status |
|----------|--------|
| S-5 Transport buttons consistent | deferred → Integration smoke |
| S-6 Feedback row separate | deferred → Integration smoke |
| S-7 Liked state colour | deferred → Integration smoke |
| S-8 Rounding uniformity | deferred → Integration smoke |
| S-9 Progress bar on light cover | deferred → Integration smoke |
| S-10 Transport commands still fire | deferred → Integration smoke |
| S-11 Settings viewport still opens | deferred → Integration smoke |

## AC coverage

| AC | Covered by | Result |
|----|-----------|--------|
| AC-1 | S-1 (source), S-5 (deferred) | PASS / deferred |
| AC-2 | S-1 (source), S-5 (deferred) | PASS / deferred |
| AC-3 | S-2 (source), S-9 (deferred) | PASS / deferred |
| AC-4 | S-6 (deferred) | deferred |
| AC-5 | S-6 (deferred) | deferred |
| AC-6 | S-6 (deferred) | deferred |
| AC-7 | S-8 (deferred) | deferred |
| AC-8 | S-7 (deferred) | deferred |
| AC-9 | S-3 | PASS |
| AC-10 | S-4 | PASS |

## Notes

- No `.fill()` calls remain on any Button widget in `player.rs` — all button fills are now
  governed by the ghost visuals block.
- The only `.fill()` in the file is on `ProgressBar::new(progress)` (line 104), which is
  intentional and correct (`rgba(255,255,255,160)`).
