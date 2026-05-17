# Iteration 1 Verification — UI module split (EP-1)

*Epic: EP-1 · Phase: Verification · Date: 2026-05-11*

---

## Test environment

**Component tests (T-01 – T-05):** Dev container (`/workspaces/gurdo`). Rust toolchain installed. Reproducible by any developer who opens the project in the devcontainer.

**E2E / UI tests (T-06 – T-12):** Host machine. Requires:
- `cargo build --release` (or `cargo run`) from a host terminal
- A valid `config.toml` pointing to a Spotify account with an active device
- Spotify playing a track (at least one with a CJK title for T-12)

**Hybrid-mode note:** Per Architecture §7 and the Project Type Adaptations for "Native desktop app", the dev container is build-only. The GUI binary cannot run in a headless container. E2E/UI scenarios are manual smoke tests executed by the developer on the host. The architecture explicitly states "No GUI E2E" — automated Playwright/Appium/etc. coverage is not applicable to this project.

**Reproduction steps:**
```bash
# In dev container — Component checks
cargo build 2>&1 | grep "^warning:" | wc -l   # count pre-existing warnings
ls src/ui/
grep -r "extract_dominant_color" src/
grep -r "OperationsState\|OperationCommand\|ProgressEvent" src/ui/state.rs

# On host — E2E smoke test
cargo run -- ui -c config.toml
# Then verify T-06 through T-12 manually per i3-test-plan.md scenarios
```

---

## External-service stubs

None required for EP-1. This is a pure refactor — no new services, no new interfaces, no new mock-server setup. Spotify and Last.fm are only exercised during the manual smoke test (T-07, T-08, T-09, T-11), where live services are used directly.

---

## Component tests (T-01 – T-05) — run in container

| ID | Scenario | Result | Notes |
|---|---|---|---|
| T-01 | Build exits 0, zero new warnings | PASS | Exit 0. 53 pre-existing warnings present (db/lastfm/spotify files not touched by EP-1). Zero warnings introduced by EP-1. |
| T-02 | Target 9 files exist; app.rs absent | PASS | `ls src/ui/` lists exactly: assets.rs, background.rs, knobs.rs, mod.rs, ops.rs, player.rs, poll.rs, settings.rs, state.rs |
| T-03 | `extract_dominant_color` absent | PASS | `grep -r "extract_dominant_color" src/` returns no output |
| T-04 | Skeleton files comment-only | PASS | settings.rs, background.rs, ops.rs, assets.rs, knobs.rs each contain one comment line and no Rust declarations |
| T-05 | EP-7 types absent from state.rs | PASS | `grep` for OperationsState, OperationCommand, ProgressEvent in state.rs returns nothing |

**All 5 Component scenarios: PASS**

---

## E2E / UI tests (T-06 – T-12) — manual on host

These scenarios are defined in `i3-test-plan.md` and must be executed by the developer on the host machine before the Integration checkpoint is considered valid. They cannot run in the dev container (native GUI, no display).

| ID | Scenario | Status | Notes |
|---|---|---|---|
| T-06 | App launches; Gurdo window appears | PENDING | Run `cargo run -- ui -c config.toml` on host |
| T-07 | Transport controls work | PENDING | Test play/pause, next, previous, seek ±10 s with Spotify active |
| T-08 | Like / Unlike / Dislike work | PENDING | Verify feedback recorded; dislike skips track |
| T-09 | Settings modal opens; slider saves to config | PENDING | Change Queue size; inspect config.toml after |
| T-10 | Error modal appears and dismisses | PENDING | Invalidate Spotify token; observe modal; click OK |
| T-11 | Queue button starts recommendation queue | PENDING | Requires prior fetch-tracks run |
| T-12 | CJK characters render correctly | PENDING | Play a track with Japanese/Chinese/Korean title |

**Developer action required:** Run scenarios T-06 through T-12 on the host and confirm each ✓. These verifications must be completed before the Integration checkpoint can be declared APPROVED.

---

## Quarantined tests

None. All component scenarios pass. E2E scenarios are PENDING (awaiting host execution), not quarantined.

---

## AC coverage table

| AC | Coverage | Verification scenario(s) | Notes |
|---|---|---|---|
| AC-1 | T-02 (PASS) + T-05 (PASS) | Component | File structure confirmed; EP-7 types absent |
| AC-2 | T-01 (PASS) | Component | Build exits 0; 53 pre-existing warnings (deviation documented in i4-dev.md) |
| AC-3 | T-01 (PASS) | Component (transitive) | main.rs unmodified; compiles → run() still reachable |
| AC-4 | T-03 (PASS) | Component | grep confirms absence |
| AC-5 | T-04 (PASS) | Component | File inspection confirmed |
| AC-6 | T-06 (PENDING) | E2E / UI — manual | Host smoke test required |
| AC-7 | T-07 (PENDING) | E2E / UI — manual | Host smoke test required |
| AC-8 | T-08 (PENDING) | E2E / UI — manual | Host smoke test required |
| AC-9 | T-09 (PENDING) | E2E / UI — manual | Host smoke test required |
| AC-10 | T-10 (PENDING) | E2E / UI — manual | Host smoke test required |
| AC-11 | T-11 (PENDING) | E2E / UI — manual | Host smoke test required |
| AC-12 | T-12 (PENDING) | E2E / UI — manual | Host smoke test required |
