# Iteration 14 Integration — Schema cleanup: similar_tracks drop (EP-13)

## Build status

`cargo build` — green, 47 warnings (down from 53).

## Env prep

None required.

## Start result

No new runnable artefact; existing app launches unchanged.

## Smoke outcome

`cargo test` — 16/16 pass. Structural grep checks all clean.

## Verification roll-up

All AC checks pass. 0 quarantined items.

## AC pass/fail table

| AC | Description | Result |
|----|-------------|--------|
| AC-1 | `CREATE TABLE similar_tracks` + index removed from schema.rs | PASS |
| AC-2 | `DROP TABLE IF EXISTS similar_tracks` in migrations batch | PASS |
| AC-3 | Three dead query functions removed from queries.rs | PASS |
| AC-4 | `track_similar` method + 3 model structs removed from lastfm layer | PASS |
| AC-5 | `similar_tracks_limit` removed from config.rs, config.toml, config.toml.example | PASS |
| AC-6 | Zero new warnings; build warning count improved from 53 → 47 | PASS |

## Integration-phase issues

None.

## Demo

No UI change. Demonstrable via `cargo build` warning count and `cargo test`.
