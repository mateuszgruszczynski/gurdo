# Iteration 15 Integration — Dead-code cleanup (EP-16)

## Build status

`cargo build` — green, **1 warning** (down from 47 at iteration start, from 53 at project baseline).

## Smoke outcome

`cargo test` — 16/16 pass.

## AC pass/fail table

| AC | Description | Result |
|----|-------------|--------|
| AC-1 | 15 dead query functions removed from queries.rs | PASS |
| AC-2 | Dead lastfm model structs, fields, methods removed | PASS |
| AC-3 | 3 dead lastfm client methods removed | PASS |
| AC-4 | 8 dead spotify structs + 5 dead fields removed from models.rs | PASS |
| AC-5 | 6 dead spotify client methods removed | PASS |
| AC-6 | `cargo build` = 1 warning; `cargo test` green | PASS |

## Integration-phase issues

Intermediate compile errors from over-aggressive field removal (`LovedTrack.mbid`, `WeeklyArtistEntry.mbid`) were caught and corrected before this phase. The fields ARE read in sync code; the dead-code warnings were for structurally similar fields on other structs (`ArtistRef.mbid`, `SimilarArtist.mbid`).

## Demo

No UI change. Demonstrable via `cargo build` warning count.
