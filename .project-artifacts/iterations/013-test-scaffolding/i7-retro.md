# Iteration 13 Retrospective — Test scaffolding (EP-12)

## What went well

- All 9 new tests written in a single session; 16/16 pass with zero production code changed.
- `RecordingReporter` pattern is clean and reusable — any test needing progress event assertions can import it from `crate::progress`.
- In-memory SQLite fixture approach (`open_mem()` + `init_db`) makes query round-trips self-contained and fast (<30 ms total suite).

## What was harder than expected

- `upsert_artist_external` does not lowercase names (unlike `upsert_artist_top_track`). This caused 3 test failures — tests used mixed-case names in `upsert_artist_external` but lowercase literals in the SQL UPDATE/assert. Fix: always pass lowercase to `upsert_artist_external` in tests. Decision: production code unchanged (it's a deliberate feature — external names preserve casing as received from the API).

## Plan changes

None. EP-12 delivered exactly what was scoped.

## Updated backlog

| # | Name | Priority | Status |
|---|------|----------|--------|
| EP-12 | Test scaffolding | P2 | DONE |
| EP-13 | Schema cleanup (similar_tracks drop) | P3 | ready |
| EP-16 | Dead-code cleanup (orphaned API surface) | P3 | ready |
| EP-14 | Installer packaging | P3 | ready |
| EP-15 | Traditional Chinese font (on demand) | P3 | parked |

## Proposed next epic

**EP-13 — Schema cleanup (similar_tracks drop)** — highest-priority remaining TODO (P3). Removes a dead table + its write path; improves schema hygiene before any packaging work.
