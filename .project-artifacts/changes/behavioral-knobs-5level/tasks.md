# Tasks — Behavioral knobs 5-level selectors

## DEV tasks

- [ ] T-1 Add `closest_f64` and `closest_u32` level-detection helpers to `src/ui/settings.rs`
- [ ] T-2 Add `knob_level_f64` helper (renders label + horizontal selectable_label row) to `src/ui/settings.rs`
- [ ] T-3 Replace 7 `knob_f64` / `knob_u32` call sites with `knob_level_f64` calls for the converted knobs (artist_score_exponent, track_rank_exponent, similarity_multiplier, multi_source_bonus_pct, like_bonus_flat, dislike_modifier_pct, year_bonus_pct)
- [ ] T-4 Replace `score_exponent` knob with 3-level `knob_level_f64` (Playcount factor)
- [ ] T-5 Remove `max_tracks_per_seed` knob call from settings.rs (AC-9)

## Cross-cutting

- [ ] T-6 `cargo build` ≤ 2 warnings; `cargo test` 16/16 (AC-11)
