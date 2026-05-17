# Iteration 15 Retrospective — Dead-code cleanup (EP-16)

## What went well

- Warning count dropped from 47 → 1 (target was ≤1). Only the pre-existing `last_track_uri` assignment remains.
- Systematic approach (by file) worked well.
- Compile errors immediately caught over-removed fields (`LovedTrack.mbid`, `WeeklyArtistEntry.mbid`).

## What was harder than expected

Field-level dead-code warnings are ambiguous without checking callers first. `ArtistRef.mbid` and `WeeklyArtistEntry.mbid` look identical from the warning list (both named `mbid`), but only `ArtistRef.mbid` was actually dead. Fix: always grep for field usage before removing struct fields, not just after the build fails.

## Plan changes

None. Backlog is now EP-14 (Installer packaging) and EP-15 (TC font, parked).

## Updated backlog

| # | Name | Priority | Status |
|---|------|----------|--------|
| EP-14 | Installer packaging | P3 | ready |
| EP-15 | Traditional Chinese font (on demand) | P3 | parked |

## Proposed next epic

**EP-14 — Installer packaging** — only remaining non-parked TODO epic. Produces a distributable artifact for macOS/Linux.
