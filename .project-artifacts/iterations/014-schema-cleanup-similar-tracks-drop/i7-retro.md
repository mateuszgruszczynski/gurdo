# Iteration 14 Retrospective — Schema cleanup: similar_tracks drop (EP-13)

## What went well

- Scope was precisely bounded: `similar_tracks` only, `similar_artists` untouched. No scope creep.
- Warning count dropped from 53 → 47, improving the budget for EP-16.
- Migration follows the established `DROP TABLE IF EXISTS` batch pattern — no new migration mechanism needed.

## What was harder than expected

Nothing. Straightforward deletion with no surprises.

## Plan changes

None.

## Updated backlog

| # | Name | Priority | Status |
|---|------|----------|--------|
| EP-16 | Dead-code cleanup (orphaned API surface) | P3 | ready |
| EP-14 | Installer packaging | P3 | ready |
| EP-15 | Traditional Chinese font (on demand) | P3 | parked |

## Proposed next epic

**EP-16 — Dead-code cleanup (orphaned API surface)** — highest-priority remaining TODO (P3). Will drive warnings from 47 toward 0.
