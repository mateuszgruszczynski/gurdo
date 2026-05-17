# Iteration 1 Integration — UI module split (EP-1)

*Epic: EP-1 · Phase: Integration · Date: 2026-05-11*

---

## Build status

**Command:** `cargo build --release`
**Result:** Success. Exit code 0.
**Binary:** `target/release/gurdo` — 24 MB (includes Rust stdlib, egui, SQLite, TLS, image libs).
**Warnings:** 53 pre-existing (see i4-dev.md deviation note). Zero new warnings introduced by EP-1.

---

## Environment preparation

This project uses `config.toml` for all configuration (no `.env` / `.env.example`). No pre-start env setup is required — `config.toml` is already present from the existing user setup.

**Variables classified:**
- `[lastfm].api_key`, `[lastfm].username` — user's own credentials, already in `config.toml`
- `[spotify].client_id`, `[spotify].client_secret` — user's own Spotify app credentials, already in `config.toml`
- `[db].path`, `[ui].*` — local paths and display settings, already configured

Security note: `config.toml` contains live credentials and is **not committed** (listed in `.gitignore`). Tech debt to move secrets to a separate file is tracked as EP-11.

---

## Application start

**Hybrid-mode app:** Binary built in dev container; must be executed on the host (no display available in container).

To start: run `cargo run -- ui -c config.toml` (or `./target/release/gurdo ui -c config.toml`) from a host terminal in the project directory. The Gurdo player window should appear.

*Container-based start verification was not performed — per Architecture §7, native GUI apps are run on the host only.*

---

## Manual smoke

**Scope:** EP-1 delivers no new features. The smoke confirms that the module split did not break existing behaviour. This covers AC-6 through AC-12 from the spec.

**Checklist (to be confirmed by developer on host):**

- [ ] T-06 — Player window opens with title "Gurdo", correct layout (~440×660)
- [ ] T-07 — Transport buttons (⏮ ⏪ ▶/⏸ ⏩ ⏭) work correctly with Spotify
- [ ] T-08 — Like (♥) / Unlike / Dislike (👎) record feedback; dislike skips track
- [ ] T-09 — Settings gear opens modal; slider change writes to config.toml
- [ ] T-10 — Error modal appears on token failure; OK dismisses it
- [ ] T-11 — Queue button (☰) starts a recommendation queue
- [ ] T-12 — CJK track/artist names render without tofu blocks

---

## Verification roll-up

See `i5-verify.md` for full detail.

- **Component tests (T-01–T-05):** All 5 PASS.
- **E2E / UI tests (T-06–T-12):** 7 PENDING — manual execution on host required before APPROVE.

---

## AC pass/fail table

| AC | Status | Proved by |
|---|---|---|
| AC-1 | PASS | T-02 + T-05 (Component, i5-verify.md) |
| AC-2 | PASS* | T-01 (Component); *53 pre-existing warnings, 0 new |
| AC-3 | PASS | T-01 transitive (main.rs unmodified, compiles) |
| AC-4 | PASS | T-03 (Component, grep confirms absence) |
| AC-5 | PASS | T-04 (Component, file inspection) |
| AC-6 | PENDING | T-06 manual smoke — host run required |
| AC-7 | PENDING | T-07 manual smoke — host run required |
| AC-8 | PENDING | T-08 manual smoke — host run required |
| AC-9 | PENDING | T-09 manual smoke — host run required |
| AC-10 | PENDING | T-10 manual smoke — host run required |
| AC-11 | PENDING | T-11 manual smoke — host run required |
| AC-12 | PENDING | T-12 manual smoke — host run required |

---

## Integration-phase issues

None identified. All Development and Component Verification phases completed cleanly.

---

## Demo

**Applicable?** Limited — EP-1 delivers no new visible features. The only demo is launching the app and confirming it looks and works identically to before.

**Demo script (host):**
1. `cargo run -- ui -c config.toml` — window opens
2. With Spotify playing: play/pause, next, like a track
3. Open settings gear — sliders present
4. Confirm CJK track title renders if a Japanese/Chinese/Korean track is playing

This is a refactor demo: "same as before, but the codebase is now maintainable."
