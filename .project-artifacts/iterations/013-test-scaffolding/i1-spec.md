# Iteration 13 Spec — Test scaffolding (EP-12)

*Epic: EP-12 · Phase: Refinement · Date: 2026-05-15*

---

## Problem

Seven tests exist (3 StateReporter, 3 config overlay, 1 arg-parse). Core logic —
scoring formula, recommendation sampling, DB query round-trips — is untested. Several
scenarios were deferred from earlier iterations (EP-9 SC-1/4, EP-10 SC-1/2) pending
this scaffolding epic.

---

## Scope

### In scope

**`src/engine/recommend.rs`** — `weighted_sample` unit tests (within module, private fn visible):
- Deterministic test with seeded `StdRng`.
- Edge cases: single weight, all-equal weights.

**`src/db/queries.rs`** — round-trip tests with `Connection::open_in_memory()`:
- `upsert_artist_external` → `get_all_artists_ranked` round-trip.
- `upsert_artist_top_track` → `get_all_artist_top_tracks` round-trip.
- `get_scoreable_artists_with_tracks` — requires artist with `final_score > 0` and a track.
- `recalculate_all_scores` formula: insert known artist values, call function, assert `final_score`.

**`src/engine/recommend.rs`** — `generate_recommendations` integration test (deferred SC-2 from EP-10):
- In-memory SQLite seeded with 2 artists + 3 tracks each + scores set directly.
- Assert result is non-empty and each tuple has 3 elements with score > 0.

**`src/progress.rs`** — `RecordingReporter` under `#[cfg(test)]`:
- Struct that collects `stage`, `tick`, `finish` calls into a `Vec<String>`.
- One test: sequence of calls matches expected order.

### Out of scope

- Dispatcher integration test (needs async runtime + temp file I/O — deferred to EP-12b or inline).
- Sync function `ProgressReporter` sequence tests (require real DB fixtures with network data).
- Snapshot / golden-file testing.
- CI workflow setup.

---

## Acceptance criteria

| # | Criterion |
|---|---|
| AC-1 | `weighted_sample` returns expected index for a seeded RNG (deterministic). |
| AC-2 | `recalculate_all_scores` produces `final_score` matching the documented formula for known inputs. |
| AC-3 | `upsert_artist_external` + `get_all_artists_ranked` round-trips correctly via in-memory SQLite. |
| AC-4 | `upsert_artist_top_track` + `get_all_artist_top_tracks` round-trips correctly. |
| AC-5 | `get_scoreable_artists_with_tracks` returns only artists with `final_score > 0` and a track. |
| AC-6 | `generate_recommendations` on a seeded in-memory DB returns non-empty results with scores > 0 (deferred SC-2). |
| AC-7 | `RecordingReporter` captures events in order. |
| AC-8 | `cargo build` and `cargo test` produce zero new warnings beyond the 53 baseline. |

---

## Implementation notes

### In-memory DB helper

```rust
fn open_mem() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    crate::db::schema::init_db(&conn).unwrap();
    conn
}
```

### `weighted_sample` test approach

The function is private. Tests live inside `#[cfg(test)] mod tests` within
`recommend.rs`, where private items are visible:

```rust
use rand::SeedableRng;
use rand::rngs::StdRng;

#[test]
fn weighted_sample_deterministic() {
    let weights = vec![1.0, 3.0, 1.0];          // middle weight is 3×
    let mut rng  = StdRng::seed_from_u64(42);
    let idx = weighted_sample(&weights, &mut rng);
    // With seed 42 the result is deterministic — just record and lock it in.
    assert_eq!(idx, /* value from first run */ 1);
}
```

Run once, record the actual index, then lock it in the assertion.

### `recalculate_all_scores` formula

Insert artist with: `playcount_score=10.0`, `year_bonus=1.05`, `similarity_score=5.0`,
`similarity_appearances=2`, `likes=1`, `dislikes=0`.

Call `recalculate_all_scores(conn, 3.0 /*like_bonus*/, 0.10 /*dislike_pct*/, 0.05 /*multi*/)`.

Expected:
```
base = 10.0 * 1.05 + 5.0 * (1 + 0.05 * max(0, 2-1)) = 10.5 + 5.0 * 1.05 = 10.5 + 5.25 = 15.75
feedback_bonus   = 1 * 3.0 = 3.0
feedback_penalty = max(0, 1.0 - 0 * 0.10) = 1.0
final_score = ROUND((15.75 + 3.0) * 1.0, 2) = 18.75
```

### `RecordingReporter`

```rust
pub struct RecordingReporter(pub std::sync::Mutex<Vec<String>>);

impl RecordingReporter {
    pub fn new() -> Self { Self(std::sync::Mutex::new(vec![])) }
    pub fn events(&self) -> Vec<String> { self.0.lock().unwrap().clone() }
}

impl ProgressReporter for RecordingReporter {
    fn stage(&self, name: &str) { self.0.lock().unwrap().push(format!("stage:{}", name)); }
    fn tick(&self, cur: u64, total: Option<u64>) {
        self.0.lock().unwrap().push(format!("tick:{}/{}", cur, total.unwrap_or(0)));
    }
    fn message(&self, msg: &str) { self.0.lock().unwrap().push(format!("msg:{}", msg)); }
    fn finish(&self, ok: bool, summary: &str) {
        self.0.lock().unwrap().push(format!("finish:{}/{}", ok, summary));
    }
}
```

Placed in `src/progress.rs` under `#[cfg(test)]` so it's available across all test modules.

---

## Files changed (expected)

| File | Change |
|------|--------|
| `src/engine/recommend.rs` | `#[cfg(test)]` module with 3 `weighted_sample` tests + 1 `generate_recommendations` test |
| `src/db/queries.rs` | `#[cfg(test)]` module with 5 round-trip / formula tests |
| `src/progress.rs` | `RecordingReporter` + 1 ordering test under `#[cfg(test)]` |
