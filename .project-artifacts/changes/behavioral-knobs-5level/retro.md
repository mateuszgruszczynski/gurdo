# Retrospective — Behavioral knobs 5-level selectors

## What went well

- The discussion-first approach produced a much better spec than auto-generating all 9 at once — the user caught that `max_tracks_per_seed` was dead code, which would have shipped a selector for a non-functional setting.
- Using slices instead of fixed-size arrays for the helper made the 3-level and 5-level cases share one implementation with no extra complexity.
- Removing `knob_f64` entirely was a side effect of the change — fewer helpers, cleaner file.

## What was harder than expected

Nothing significant technically. The design discussion was the right place to spend the time.

## Follow-up items

- `max_tracks_per_seed` is a dead config field — the engine never reads it. A future epic should either implement the per-artist cap in `generate_recommendations` or remove the field from `EngineConfig` and `config.toml` entirely.
