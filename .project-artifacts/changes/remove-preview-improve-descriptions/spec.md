# Spec — Remove recommendation preview + improve settings descriptions

## Change summary

Two independent improvements to the Settings window:

1. **Remove the Preview feature** — the "Preview" button and the scrollable results panel that appears below it are deleted. The underlying dispatch command and state field are also removed.
2. **Improve all settings knob descriptions** — every hover-tooltip in the Settings window is rewritten in plain English so a non-technical user can understand what changing a value actually does to their recommendations.

---

## Before / After

### Preview removal

**Before:** Settings → Recommendations section has a "Preview" button. Clicking it runs the recommendation engine against the current (draft) config and shows a scrollable list of (artist — track, score) tuples below the knobs.

**After:** That button and list are gone. The three recommendation knobs remain; only the preview surface is removed.

### Knob descriptions

**Before:** descriptions are technical — "Exponent applied to artist score before weighted sampling. >1.0 = more top-heavy."

**After:** plain English focused on audible effect — "Raise this to hear mostly your top artists; lower it to give less-played artists more of a look-in."

---

## Scope

### In scope

- Remove `Preview` variant from `OperationCommand` enum (`src/ui/state.rs`)
- Remove `preview_results` field from `OperationsState` struct (`src/ui/state.rs`)
- Remove `OperationCommand::Preview` dispatch arm from `ops.rs`
- Remove `preview_results: None` from the two `OperationsState` initialisations in `ops.rs` (production + test helper)
- Remove `preview_results = None` from the Discard handler in `settings.rs`
- Remove the Preview button and results scroll panel from `settings.rs`
- Replace all 17 knob `desc` strings in `settings.rs` with the new plain-English versions

### Out of scope

- Changing knob labels, ranges, or defaults
- Adding or removing knobs
- Any changes to the recommendation algorithm itself
- Changing the `generate_recommendations` function (still used by "Play" and "Update everything")

---

## New knob descriptions

### Recommendations

| Knob | New description |
|------|----------------|
| Number of recommendations | How many tracks Gurdo prepares for you each time it runs. |
| Artist score exponent | Raise this to hear mostly your top artists; lower it to give less-played artists more of a look-in. |
| Track rank exponent | Raise this to stick to each artist's biggest hits; lower it to let deeper cuts and B-sides appear. |

### Engine

| Knob | New description |
|------|----------------|
| Similar artists per seed | How many "sounds like" artists Gurdo looks up per artist you've played — more means a wider discovery net. |
| Tracks per artist | How many top tracks Gurdo fetches per artist — higher gives more songs to choose from per artist. |
| Recommendation pool size | Raise this for more variety before your queue is finalised; lower it to keep the selection tightly focused. |
| Max tracks per seed artist | Stops any single artist from flooding your queue — lower this for a more balanced mix across artists. |
| Similarity multiplier | Raise this to hear more artists similar to ones you've played; lower it to stay closer to artists you've actually listened to. |
| Multi-source bonus | Raise this to give a bigger boost to artists recommended by several of your favourites at once. |
| Like bonus (flat) | Raise this to push artists with loved tracks higher up your queue. |
| Dislike penalty (%) | Raise this so a single disliked track drops an artist further down your recommendations. |

### Artist Scoring

| Knob | New description |
|------|----------------|
| Playcount score exponent | Raise this to widen the gap between artists you play constantly and ones you only occasionally revisit; lower it to treat all artists more equally. |
| Year active bonus (%) | Gives a small boost to artists you've kept coming back to across many years of listening. |
| Min playcount threshold | Artists you've played fewer times than this are ignored — raise it to filter out artists you've barely touched. |

### Sync

| Knob | New description |
|------|----------------|
| Loved tracks limit | How many of your Last.fm loved tracks are fetched during a sync. |
| Seed artists limit | Raise this to pull more of your listened-to artists into recommendations; lower it to focus only on your most-played ones. |
| Seed tracks limit | How many of your most-listened tracks are used as the starting point when building recommendations. |

---

## Acceptance criteria

| AC | Description |
|----|-------------|
| AC-1 | Settings window has no "Preview" button. |
| AC-2 | No `preview_results` state is stored anywhere. |
| AC-3 | `OperationCommand::Preview` variant does not exist. |
| AC-4 | All 17 knob hover descriptions match the new plain-English text above. |
| AC-5 | `cargo build` green (≤ 2 pre-existing warnings). |
| AC-6 | `cargo test` 16/16 pass. |

---

## Files affected

| File | Change |
|------|--------|
| `src/ui/state.rs` | Remove `preview_results` field; remove `Preview` command variant |
| `src/ui/ops.rs` | Remove Preview dispatch arm; update two `OperationsState` struct literals |
| `src/ui/settings.rs` | Remove button + panel; replace 17 description strings |
