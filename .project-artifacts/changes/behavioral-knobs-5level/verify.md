# Verification — Behavioral knobs 5-level selectors

## Environment

Dev container, Linux aarch64. All scenarios in-process.

## Scenario results

| Scenario | Check | Result |
|----------|-------|--------|
| S-1 | Default values: closest_f64(1.0, 5-level) → 2; closest_f64(0.301, 3-level) → 1 | PASS |
| S-2 | Off-preset: closest_f64(0.8, [0.3,0.6,1.0,1.5,2.5]) → 2; (1.3) → 3 | PASS |
| S-3 | `cargo build` → 2 warnings (both pre-existing) | PASS |
| S-4 | `cargo test` → 16/16 | PASS |
| S-5 | No `knob_f64` call remains for converted fields | PASS |
| S-6 | `grep max_tracks_per_seed src/ui/settings.rs` → no match | PASS |
| S-7 | Numeric knobs (count, limits, thresholds, pool size) still use DragValue | PASS |
| S-8 | All `knob_level_f64` calls use `.then(|| any_changed = true)` | PASS |

## AC coverage

| AC | Result |
|----|--------|
| AC-1 | PASS |
| AC-2 | PASS |
| AC-3 | PASS |
| AC-4 | PASS |
| AC-5 | PASS |
| AC-6 | PASS |
| AC-7 | PASS |
| AC-8 | PASS |
| AC-9 | PASS |
| AC-10 | PASS |
| AC-11 | PASS |

## Notes

- `knob_f64` helper removed entirely — no longer needed after all f64 behavioral knobs were converted. This reduced warnings from 3 to 2.
