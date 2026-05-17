# Backlog — Gurdo (improve cycle, delta mode)

*Phase: Backlog · Mode: improve · Date: 2026-05-11*

Seeded from the gap between Analysis (current state) and Vision (desired end state). Each P1 epic maps to one or more success criteria in [Vision §6](.project-artifacts/f1-vision.md). Tech debt from Analysis §12 appears as explicit epics, not hidden inside feature work.

---

## Epic table

| # | Name | Type | Priority | Size | Depends on | Status |
|---|---|---|---|---|---|---|
| EP-1 | UI module split | REFACTOR | P1 | M | — | ready |
| EP-2 | CLI removal & entry-point collapse | MIGRATION | P1 | S | — | ready |
| EP-3 | Embedded assets + CJK font fix | FIX | P1 | S | — | DONE |
| EP-4 | Cover-blur background painter | FEATURE | P1 | M | EP-1 | DONE |
| EP-5 | Idle-state placeholder cover | FEATURE | P1 | S | EP-1, EP-3 | DONE |
| EP-6 | Settings viewport window | FEATURE | P1 | M | EP-1 | DONE |
| EP-7 | In-process operations + progress | FEATURE | P1 | L | EP-1, EP-6 | ready |
| EP-8 | Full config-knob exposure | FEATURE | P1 | L | EP-6 | ready |
| EP-9 | Combined "Update everything" action | FEATURE | P2 | S | EP-7 | ready |
| EP-10 | Recommendation preview-while-tuning | FEATURE | P2 | M | EP-7, EP-8 | ready |
| EP-11 | Secrets hardening & multi-user config | SECURITY | P2 | M | — | DONE |
| EP-12 | Test scaffolding | QA | P2 | M | — | ready |
| EP-13 | Schema cleanup (similar_tracks drop) | TECH_DEBT | P3 | S | — | ready |
| EP-14 | Installer packaging | INFRA | P3 | M | — | ready |
| EP-15 | Traditional Chinese font (on demand) | REFINEMENT | P3 | XS | EP-3 | parked |
| EP-16 | Dead-code cleanup (orphaned API surface) | TECH_DEBT | P3 | S | EP-2 | ready |
| EP-17 | Spotify API error suppression + status indicator | FIX | P1 | S | — | DONE |
| EP-18 | Remove recommendation preview + plain-English settings descriptions | REFINEMENT | P2 | S | EP-10 | DONE |
| EP-19 | Replace behavioral knobs with contextual level selectors | REFINEMENT | P2 | M | EP-18 | DONE |
| EP-20 | First-run setup screen + user-scoped config/secrets | FEATURE | P2 | M | EP-6, EP-11 | ready |

Legend: **P1** must land for MVP close · **P2** important, follow-up cycle · **P3** opportunistic / on-demand · **XS/S/M/L/XL** intuitive sizing.

---

## EP-1 — UI module split

**Type:** REFACTOR · **Priority:** P1 · **Size:** M · **Roles:** DEV, QA

[src/ui/app.rs](src/ui/app.rs) is 876 LOC mixing widget rendering, async polling, command handling, image decode, font loading, settings draft, and color extraction. Everything downstream (multi-window UI, blur background, in-process ops, knob exposure) needs a place to live. This epic carves the file into the module layout decided in Architecture §4.7 — with **no user-visible behaviour change**.

**Scenarios:**
- As the developer, I can find the player rendering code by opening `src/ui/player.rs` instead of scrolling a thousand-line file.
- As the developer, I can add a new sub-module under `src/ui/` without re-reading the polling loop.
- As the user, the app behaves identically to before — same window, same controls, same actions.

**High-level acceptance criteria:**
- `src/ui/` contains: `mod.rs`, `state.rs`, `player.rs`, `settings.rs` (skeleton, empty), `background.rs` (skeleton), `poll.rs`, `ops.rs` (skeleton), `assets.rs` (skeleton), `knobs.rs` (skeleton).
- `src/ui/app.rs` is deleted; `mod.rs` exposes `pub fn run(config, path)`.
- The app builds clean (no warnings), launches, and the existing player + settings modal behave as before.
- No behaviour regressions in playback, like/dislike, queue, polling, error modal.

**Out of scope:** any new functionality (settings window, blur, in-process ops, knob exposure) — those land in their own epics.

**Risks / unknowns:** the polling loop uses `Arc<Mutex<...>>` and message channels in ways that span the old file; splitting carelessly can introduce subtle deadlocks. Mitigation: keep `ui::state` as the single source of truth for shared types, and let `mod.rs` own the channel wiring.

---

## EP-2 — CLI removal & entry-point collapse

**Type:** MIGRATION · **Priority:** P1 · **Size:** S · **Roles:** DEV

Delete every `clap` subcommand except the implicit UI launch. `main.rs` becomes ~30 lines: parse a single optional `-c <path>` for config location (by hand, no `clap`), load config, open DB, run UI. Drop `clap` from `Cargo.toml`.

**Scenarios:**
- As the user, I run `gurdo` (or `cargo run --release`) and the UI opens immediately.
- As the user, I run `gurdo -c /path/to/other-config.toml` and the alternate config is used.
- As the developer, `Cargo.toml` no longer lists `clap` and `cargo tree` confirms it's not a transitive dependency of anything I still need.

**High-level acceptance criteria:**
- `main.rs` contains no `clap` derives, no `Cli`/`Command` structs.
- `Cargo.toml` `[dependencies]` does not include `clap`.
- The seven previous subcommands (`sync-lastfm`, `expand`, `score`, `fetch-tracks`, `recommend`, `login`, `devices`, `play`) are unreachable; their wiring in `main.rs` is removed.
- `gurdo --help` is not provided (or shows just the `-c` flag).
- The binary builds in release mode and launches into the UI.

**Out of scope:** in-process replacements for the removed subcommand actions — that's EP-7. This epic just deletes; EP-7 fills the gap.

**Risks / unknowns:** until EP-7 lands, there is *no way* to trigger sync/expand/fetch/score/login. Order matters — see §Sequencing.

---

## EP-3 — Embedded assets + CJK font fix

**Type:** FIX · **Priority:** P1 · **Size:** S · **Roles:** DEV, DESIGN, SECURITY (license check)

Embed the three CJK fonts (`NotoSansSC/JP/KR-Regular.otf`) and the placeholder cover image via `include_bytes!` into the binary. Register the fonts in `egui::FontDefinitions` as ordered fallbacks (JP → SC → KR) on the Proportional family. Remove the OS-font-path probing loop. This fixes the current tofu-rectangles bug for Chinese/Japanese/Korean track titles.

**Scenarios:**
- As the user, the track title "夜に駆ける" renders with real glyphs, not rectangles.
- As the user, "강남스타일" renders with Hangul glyphs.
- As the user, "Розы" renders correctly (Cyrillic already worked).
- As the developer, the binary is self-contained — no OS font files required.

**High-level acceptance criteria:**
- `assets/fonts/NotoSansSC-Regular.otf`, `assets/fonts/NotoSansJP-Regular.otf`, `assets/fonts/NotoSansKR-Regular.otf`, OFL.txt all committed.
- `assets/images/placeholder_cover.png` committed (small, CC0 or author-created).
- `src/ui/assets.rs` exposes `NOTO_SANS_SC`, `NOTO_SANS_JP`, `NOTO_SANS_KR`, `PLACEHOLDER_COVER` as static byte slices.
- The old OS-font-path probe block in `app.rs` is deleted.
- Manual test: spin up the app, play a track with a CJK title — glyphs render.
- Binary size growth ~15–17 MB; documented in commit message.

**Out of scope:** displaying the placeholder image — that's EP-5. This epic just ships the bytes.

**Risks / unknowns:** placeholder image needs to be license-clean. Either commissioned/drawn locally or sourced from CC0. Confirm license in commit.

---

## EP-4 — Cover-blur background painter

**Type:** FEATURE + REFACTOR · **Priority:** P1 · **Size:** M · **Roles:** DEV, DESIGN

Replace the static `panel_fill` background with the current track's cover art, downscaled and blurred, with a dark vertical gradient overlay for legibility. Falls back to `[ui].background_color` when no track is playing. Delete the dead `extract_dominant_color` function and the commented-out HSV/bucketing code.

**Scenarios:**
- As the user, when a vibrant red album plays, the player background becomes a soft, blurred red wash — never muddy.
- As the user, when a near-monochrome cover plays, the background reflects that mood (gray/black) — readable, never "every cover ends up gray-brown".
- As the user, when nothing is playing, the background is the static color from config.
- As the developer, [src/ui/background.rs](src/ui/background.rs) owns the blur pipeline and is testable in isolation.

**High-level acceptance criteria:**
- `ui::background::BackgroundPainter` accepts cover bytes, produces a blurred `TextureHandle`, and paints it as a full-window image with a `(0,0,0,60)→(0,0,0,200)` vertical gradient on top.
- Blur uses `image::imageops::fast_blur` at ~256×256 working size with sigma ≈ 30. No new dependency.
- The blurred texture is rebuilt only when the cover URL changes (cached otherwise).
- Blur generation runs off the UI thread (on the polling runtime) and is delivered via channel so frames never stall on a blur.
- `extract_dominant_color` and its commented-out HSV code in the old `app.rs` are deleted (will be naturally gone after EP-1, but explicitly confirmed here).
- Text/controls remain legible on a pure-white cover and on a pure-black cover.

**Out of scope:** any per-cover *color* extraction (vibrant palette, etc.) — the architecture chose blur-overlay for a reason. Don't reintroduce extraction here.

**Risks / unknowns:**
- `fast_blur` quality at sigma 30 — if it looks blocky, swap to a two-pass box blur or a real gaussian. Verify on a few real covers before committing.
- Performance: blur ~256×256 on a low-spec CPU should be <30 ms; verify on the dev machine.

---

## EP-5 — Idle-state placeholder cover

**Type:** FEATURE · **Priority:** P1 · **Size:** S · **Roles:** DEV, DESIGN

Show a bundled placeholder image in the 400×400 cover slot whenever no track is playing (first launch, gaps between tracks, paused with no track loaded). Today's UI allocates empty space; this epic fills it.

**Scenarios:**
- As the user, on first launch, instead of an empty square I see a small "no track playing" placeholder image.
- As the user, when Spotify reports no active playback, the placeholder appears.
- As the user, when a track is loaded, the real cover replaces the placeholder seamlessly.

**High-level acceptance criteria:**
- `ui::player` decodes `PLACEHOLDER_COVER` once into a long-lived `TextureHandle` at app start.
- The slot shows the placeholder texture exactly when `PlayerState.album_art_bytes.is_none()`.
- The placeholder uses the same 400×400 layout and rounding as the real cover (no layout jump).
- Background remains the static color from config (no blur applied to the placeholder — confirmed visually clean).

**Out of scope:** animated placeholders, multiple placeholder variations, "you've heard X tracks today" stat displays. Just one static image.

**Risks / unknowns:** the chosen image must look intentional next to the static background color — preview before committing.

---

## EP-6 — Settings viewport window

**Type:** FEATURE · **Priority:** P1 · **Size:** M · **Roles:** DEV, DESIGN

Settings becomes a separate OS window opened via `egui::Context::show_viewport_deferred`, centered on the player window at the moment of opening. Window sizes for both player and settings are read from new `config.toml` fields `[ui].player_window_size` and `[ui].settings_window_size`. The new window hosts (initially) empty placeholder sections that EP-7 and EP-8 will fill: **Data**, **Spotify**, **Recommendations**, **Engine**, **Artist Scoring**, **Sync**, **Appearance** (read-only).

**Scenarios:**
- As the user, clicking ⚙ in the player opens a new OS-level window centered over the player.
- As the user, dragging the player and then opening Settings places Settings centered over the player's new position.
- As the user, I can resize the player window by editing `[ui].player_window_size` in `config.toml` and relaunching.
- As the user, the player window stays interactive (play/pause/seek work) while Settings is open.

**High-level acceptance criteria:**
- `ui::settings::render` exposes a `pub fn` invoked by the player viewport on each frame when `settings_open == true`.
- Player calls `ctx.show_viewport_deferred("settings", ViewportBuilder::with_inner_size(...).with_position(centered), …)`.
- `[ui].player_window_size` and `[ui].settings_window_size` exist in `Config` with defaults `[440, 660]` and `[800, 900]`; old `config.toml` files still load (via serde defaults).
- Closing the Settings window via the OS close button sets `settings_open = false`.
- The Settings window contains visually-distinct empty section placeholders ("Data", "Spotify", "Recommendations", "Engine", "Artist Scoring", "Sync", "Appearance"). Each placeholder shows "Coming in EP-7/EP-8" — or, post-EP-7/8, real content.
- Both windows share `Arc<Mutex<PlayerState>>` / `Arc<Mutex<OperationsState>>`; no duplicated state.

**Out of scope:** populating the Data section (EP-7) or the knob sections (EP-8).

**Risks / unknowns:**
- egui's deferred viewport API in 0.29 — confirm `with_position` actually positions the window on macOS (some platforms ignore initial position hints). Mitigation: if unreliable, accept off-center positioning as a known limitation and document.
- Resizing the window at runtime — out of scope; we don't write back size on close.

---

## EP-7 — In-process operations + progress reporting

**Type:** FEATURE · **Priority:** P1 · **Size:** L · **Roles:** DEV, QA

The Data section of Settings exposes **Sync Last.fm**, **Expand similar artists**, **Fetch top tracks**, **Recalculate scores**, and (in the Spotify section) **Login**. Each runs in-process on a dedicated tokio task, with live progress (stage name, current/total count, last message) flowing back into `OperationsState` via the `ProgressReporter` trait defined in Architecture §4.5. Only one operation runs at a time; other buttons disable while one is active. Spawn-CLI subprocess code is deleted.

**Scenarios:**
- As the user, I click **Sync Last.fm** in Settings; a progress panel appears showing "Fetching loved tracks… 234/500" and updates live; the final state is "Sync complete — 412 artists, 1 832 loved tracks."
- As the user, while a sync runs in Settings, the player still polls Spotify and plays tracks without stutter.
- As the user, if a sync fails (network error, invalid token), the panel shows a clear error string and the button re-enables.
- As the user, while sync is running, the other Data buttons (Expand, Fetch, Score) are visibly disabled.

**High-level acceptance criteria:**
- `ProgressReporter` trait + `ChannelReporter` implementation in `ui::ops`.
- `sync::sync_lastfm`, `sync::expand_artists`, `sync::fetch_artist_tracks`, `engine::artist_scores::score_artists` accept a `progress: &dyn ProgressReporter` and emit at least: stage transitions, per-step tick counts, final summary.
- A new `OperationCommand` enum and a dispatcher tokio task (separate from the playback polling loop).
- `OperationsState` contains `active: Option<ActiveOperation>` and `last_result: Option<OperationOutcome>`.
- The Data section in Settings shows live state from `OperationsState`; buttons disabled when an operation is active.
- Spotify Login is also wired through the dispatcher (runs `spotify::auth::run_oauth_flow` with a thin reporter); on success, status flips to "Connected as <user>".
- The `spawn_cli` helper and "Last.fm" subprocess buttons from the old Settings modal are deleted.
- Errors propagate as `OperationOutcome::Failed(String)` and display in the UI; no silent `let _ = ...spawn()` calls remain.

**Out of scope:** combining operations into one action (EP-9), previewing recommendations (EP-10).

**Risks / unknowns:**
- The existing sync functions don't have clean "total count" signals at the start of each stage (e.g. number of artists to expand isn't known until the previous query runs). `total: Option<u64>` is `None` for indeterminate stages — UI shows a spinner with current count only.
- OAuth login currently expects a browser; on a headless dev container this won't work — but the app runs on the host (hybrid mode), so non-issue. Note in commit.
- Cancellation: not in this epic. If a sync takes 5 minutes, the user waits or quits the app. Adding cancel = future epic.

---

## EP-8 — Full config-knob exposure in Settings

**Type:** FEATURE · **Priority:** P1 · **Size:** L · **Roles:** DEV, DESIGN

Every numeric / boolean tunable in `[sync]`, `[engine]`, `[artist_scoring]`, `[recommendations]` is exposed in the Settings window as a labeled, described, range-validated control with a per-field reset-to-default button. Knob metadata lives in `src/ui/knobs.rs` as static `KnobSpec` slices grouped by config section (Architecture §4.6). `[lastfm]`, `[spotify]`, `[app]`, `[ui]` are shown as read-only (or marked "restart required") in an Appearance/About section. Saves write back to `config.toml`. The `TRACKS_PER_ARTIST = 50` hardcoded constant in `sync/expand.rs:12` is replaced by the config field it currently shadows.

**Scenarios:**
- As the user, I open Settings, scroll to "Engine — Similar artists per seed", read the description "How many similar artists to fetch from Last.fm per seed artist", change the value from 20 to 30, click Save.
- As the user, I hit "Reset" next to a field I've messed with and it returns to the configured default.
- As the user, after Save, the change is written to `config.toml` and applied to the running app (next operation honors it).
- As the developer, adding a new knob requires one line in the `KnobSpec` array plus the corresponding field on the Config struct.

**High-level acceptance criteria:**
- Every numeric/bool field in `[sync]`, `[engine]`, `[artist_scoring]`, `[recommendations]` is reachable from the Settings window — exhaustively (currently ~22 fields; today's modal exposes 9).
- Each knob has: group, label, one-line description (visible on hover or below the label), min/max range, default-for-reset, type-appropriate widget (`DragValue` for numeric, `Checkbox` for bool).
- A Save action persists changes to `config.toml` via `Config::save`. Live-edit (save-on-change) is removed in favor of explicit Save — protects against accidental scroll-wheel mutations.
- Changed fields are visually marked (e.g. dot indicator) and a "Save" button is only enabled when something is dirty.
- `sync/expand.rs::TRACKS_PER_ARTIST` is deleted; the function reads `config.engine.artist_top_tracks_limit`.
- Read-only sections show `[lastfm]`, `[spotify]`, `[app]`, `[ui]` with a "Edit in config.toml" hint.

**Out of scope:** previewing recommendation output as knobs change (EP-10). Reorderable sections, custom presets, import/export.

**Risks / unknowns:**
- Some knobs are only consumed by specific operations — changing them mid-run is a no-op until the next sync/expand/score. Document on the relevant knob description ("Applies to next Sync").
- Validation: what if the user types 0 for `similar_artists_limit`? Min should be 1; enforced by widget range.

---

## EP-9 — Combined "Update everything" action

**Type:** FEATURE · **Priority:** P2 · **Size:** S · **Roles:** DEV

A single button in the Data section that chains Sync → Expand → Fetch → Score as one sequential operation, with a multi-stage progress display ("Step 2 of 4: Expand similar artists — 45/120"). Built as a thin wrapper around EP-7's dispatcher; no new infrastructure.

**Scenarios:**
- As the user, after I've validated each individual step works, I click **Update everything** and the four stages run in sequence with a unified progress display.
- As the user, if any step fails, subsequent steps are skipped and the result panel shows which step failed.

**High-level acceptance criteria:**
- `OperationCommand::UpdateAll` variant; dispatcher runs the four operations in order using the same `ChannelReporter` with a stage prefix.
- UI shows "Step N of 4" in the active operation panel.
- Failure stops the chain; partial-completion state is honestly reported.

**Out of scope:** parallel execution (the user explicitly wants sequential).

**Risks / unknowns:** none significant; this is plumbing.

---

## EP-10 — Recommendation preview-while-tuning

**Type:** FEATURE · **Priority:** P2 · **Size:** M · **Roles:** DEV, DESIGN

In the Recommendations / Engine sections, add a **Preview** button that runs `engine::recommend::generate_recommendations` with current draft knob values (not yet saved) and shows the resulting top-N (artist, track) pairs with each artist's `final_score` beside it. Re-running Preview after tweaking a knob shows how the candidate list shifts.

**Scenarios:**
- As the user, I open Recommendations, click Preview, see 20 (artist, track) candidate pairs.
- As the user, I bump `artist_score_exponent` from 1.0 to 0.6, click Preview again, immediately see more obscure picks surface.
- As the user, Preview does not save my changes — I have to click Save separately.

**High-level acceptance criteria:**
- Preview reads draft knob values (not committed) and runs `generate_recommendations` against the current DB state.
- Result is rendered as a scrollable list in a panel beside or below the Recommendations knobs.
- Preview is a read-only operation — no writes to DB or config.

**Out of scope:** previewing playback (no audio), persisting a preview as a saved playlist.

**Risks / unknowns:** `generate_recommendations` is fast (<100 ms typical) and can be called inline without spawning a task; verify on a large DB.

---

## EP-11 — Secrets hardening & multi-user config

**Type:** SECURITY · **Priority:** P2 · **Size:** M · **Roles:** DEV, SECURITY

Today `config.toml` is committed with live Last.fm api_key + Spotify client_id + the username `grucha666`. To ship to other users (a stated future goal — Architecture §12), secrets need to move out of the committed file. Options to evaluate during Refinement:
- A separate `~/.gurdo/secrets.toml` (gitignored).
- Environment variables (`GURDO_LASTFM_API_KEY`, `GURDO_SPOTIFY_CLIENT_ID`).
- A first-launch wizard inside the UI that asks for keys and writes them to a user-only config.

**Scenarios:**
- As a new user, on first launch I'm prompted for my Last.fm API key + Spotify client ID, the values are stored in a user-local file, and `config.toml` in the repo contains only non-sensitive defaults.
- As the existing user, my current keys are migrated automatically on first run of the new version.
- As a contributor, cloning the repo no longer exposes credentials.

**High-level acceptance criteria:**
- `config.toml` in git contains no real keys, no usernames.
- A documented path (env var / user-config file / wizard) exists for users to supply their own keys.
- `.gitignore` excludes the user's config from accidental re-commit.
- A migration path for the current user (one-time) preserves their existing keys.

**Out of scope:** multi-account support (one Last.fm + one Spotify per install). Encryption at rest of the credentials file.

**Risks / unknowns:** committing the *removal* of keys is a git history concern — the keys remain in history. Decision in Refinement: rotate the keys (issue new ones from Last.fm/Spotify dev consoles) after the cleanup commit, or leave history alone since the API key is rate-limited and the Spotify client_id is not a secret in the OAuth-PKCE sense.

---

## EP-12 — Test scaffolding

**Type:** QA · **Priority:** P2 · **Size:** M · **Roles:** DEV, QA

Today there are **zero** tests. This epic introduces a baseline harness:
- Unit tests for `engine::recommend::weighted_sample` (deterministic RNG).
- Unit tests for `engine::artist_scores` formula behaviour (set known inputs, verify outputs).
- Integration tests for `db::queries` round-trips against an in-memory SQLite.
- Unit tests for the `ProgressReporter` channel: mock reporter records events; sync functions emit expected sequence.
- Integration test for the operations dispatcher: submitting `OperationCommand::Score` results in expected `OperationsState` transitions.
- All tests headless; pass inside the dev container and in CI (CI itself is not added here — that's a future epic).

**Scenarios:**
- As the developer, `cargo test` passes on a freshly cloned repo without any external service or display.
- As the developer, adding a new sync stage with a progress reporter is verifiable in a unit test.

**High-level acceptance criteria:**
- `tests/` directory or `#[cfg(test)]` modules exist for the components above.
- `cargo test` is green on the dev machine.
- No real Last.fm or Spotify network calls in tests — fixtures stored under `tests/fixtures/`.
- At least one test per: weighted_sample determinism, score formula behaviour, progress channel ordering.

**Out of scope:** GUI/E2E tests, contract tests against live Last.fm/Spotify, CI workflow setup (`.github/workflows/`), coverage tooling.

**Risks / unknowns:** none significant — this is groundwork.

---

## EP-13 — Schema cleanup (drop `similar_tracks`)

**Type:** TECH_DEBT · **Priority:** P3 · **Size:** S · **Roles:** DEV

The `similar_tracks` table is populated by old code paths but **never read** by the recommender. Dead schema. Remove the table, the `upsert_similar_track` query, and any sync code that writes to it. Add a migration that drops the table on next launch.

**Scenarios:**
- As the developer, the schema has one fewer table; no dead writes.
- As the user, the DB shrinks slightly after the migration runs.

**High-level acceptance criteria:**
- `CREATE TABLE similar_tracks` removed from schema.
- `DROP TABLE IF EXISTS similar_tracks` added to migration block.
- All callers / query helpers referencing `similar_tracks` are removed.
- The remaining algorithm produces identical recommendations (no behavioural regression).

**Out of scope:** broader DB optimization, index review, vacuum.

**Risks / unknowns:** confirm via grep that no live code path reads `similar_tracks`. Already verified in Analysis but worth re-checking on landing.

---

## EP-14 — Installer packaging

**Type:** INFRA · **Priority:** P3 · **Size:** M · **Roles:** DEV, DEVOPS (one-shot)

Ship a real installer for macOS (`.dmg` or `.app` bundle) and Linux (`.AppImage` or `.deb`). Today users run `cargo build --release` and copy the binary by hand. This epic adds a build recipe — likely `cargo-bundle` or a hand-rolled script — that produces a distributable artifact per platform.

**Scenarios:**
- As a new user, I download a `.dmg` from a Releases page, drag the app to Applications, double-click, it runs.
- As the maintainer, `cargo bundle --release` (or `make release`) produces the artifact in one command.

**High-level acceptance criteria:**
- A documented build recipe produces a distributable artifact for at least macOS.
- The artifact contains the binary plus a license bundle (incl. OFL for the embedded fonts).
- Manual install on a clean machine works without `cargo` installed.

**Out of scope:** code signing / notarization (separate epic), auto-update, App Store / Homebrew publishing.

**Risks / unknowns:**
- macOS unsigned binaries are blocked by Gatekeeper — users will need to right-click → Open the first time. Document this; signing is its own future epic.
- `eframe` apps need careful bundling — verify the resulting `.app` launches correctly.

---

## EP-15 — Traditional Chinese font (on demand)

**Type:** REFINEMENT · **Priority:** P3 · **Size:** XS · **Roles:** DEV · **Status:** parked

Embed `NotoSansTC-Regular.otf` (~5 MB) as a fourth fallback font *if and only if* the user reports glyphs that don't render correctly because of TC-specific forms. JP often covers them; this epic is on-demand only.

**Scenarios:**
- As a user listening to Cantonese / Taiwanese pop, "你好" renders with the expected TC glyph forms (not Simplified-style fallbacks).

**High-level acceptance criteria:**
- `assets/fonts/NotoSansTC-Regular.otf` and a constant in `ui::assets` exist.
- Font registered as a fourth fallback (after JP, SC, KR or wherever appropriate).
- Binary grows by ~5 MB.

**Out of scope:** language preference switching at runtime.

**Risks / unknowns:** none — trivial change, only blocked by "is this actually a problem in practice."

---

## EP-16 — Dead-code cleanup (orphaned API surface)

**Type:** TECH_DEBT · **Priority:** P3 · **Size:** S · **Roles:** DEV · **Depends on:** EP-2

Remove the unused `pub fn` and `pub struct` items in `src/db/queries.rs`, `src/lastfm/client.rs`, `src/lastfm/models.rs`, `src/spotify/client.rs`, and `src/spotify/models.rs` that were originally called from CLI subcommands now removed by EP-2. These generate 52 `dead_code` and `unused` compiler warnings. EP-13 (similar_tracks drop) handles the `upsert_similar_track` family; this epic handles the rest.

**Scenarios:**
- As the developer, `cargo build` produces only warnings about items I actively want to keep (or zero warnings).
- As the developer, the dead API surface is gone so I can add new code without noise drowning out real issues.

**High-level acceptance criteria:**
- `cargo build` warning count drops from 53 to 1 or fewer (the `last_track_uri` unused_assignment is in existing logic and benign; may be deferred further).
- No behaviour regressions — deleted items were not called at runtime.
- Deleted functions confirmed unreferenced by `cargo check` and grep.

**Out of scope:** Fixing the `last_track_uri` unused_assignment warning (in poll.rs — logic change, separate decision). Replacing deleted APIs with new ones.

**Risks / unknowns:** Some "dead" functions might be called by tests added in EP-12. Run after EP-12 to avoid conflicts, or audit against EP-12 test fixtures.

---

## EP-17 — Spotify API error suppression + status indicator

**Type:** FIX · **Priority:** P1 · **Size:** S · **Roles:** DEV

During Spotify API downtime the polling loop (every ~5 s) produces a new error on every
cycle. Each error surfaces as a blocking modal that the user must dismiss, so sustained
downtime creates an uninterruptible modal flood. The fix has two parts:

1. **Suppress the modal after the first failure.** Once the user has acknowledged an
   error (OK or Snooze), stop raising new modals for the same class of error until the
   API recovers or the snooze expires.
2. **Show a passive status indicator.** While in degraded/snoozed state, show a small
   non-blocking warning in the player UI so the user knows why playback polling is quiet.

**Proposed mechanism:**
- Add `api_error_count: u32` and `error_snoozed_until: Option<std::time::Instant>` to
  `PlayerState`.
- In `poll.rs`, before writing to `state.error`: if `error_snoozed_until` is `Some` and
  still in the future, skip the modal (don't set `state.error`).
- In the error modal, replace the plain "Ignore" button with **"Snooze 10 min"** which
  sets `error_snoozed_until = Instant::now() + 10 min`. Keep "OK" as a dismiss-once.
- In `player.rs`, when `error_snoozed_until` is active, render a small inline warning
  label (e.g., `"⚠ Spotify API unavailable"`) near the track info — not a modal, just
  a text line that disappears when the snooze expires or the API recovers.

**Scenarios:**
- As the user, during a Spotify outage I see one error modal. I click "Snooze 10 min".
  For the next 10 minutes the modal does not reappear; instead a small "⚠ Spotify API
  unavailable" label shows in the player. When the API recovers the label disappears.
- As the user, if I click "OK" I see the error once more on the next failed poll — same
  as today. The change only applies to "Snooze".

**High-level acceptance criteria:**
- After "Snooze 10 min", no new error modals appear for 10 minutes from the snooze click.
- A non-blocking status label (⚠ + short message) is visible in the player while snoozed.
- The label disappears automatically when the snooze expires or a successful poll occurs.
- "OK" dismiss behaviour is unchanged from today.
- `cargo build` produces zero new warnings.

**Out of scope:** per-endpoint error categorisation, retry back-off, circuit breaker.
Cancellation of in-flight polls during downtime is EP-7's concern.

**Risks / unknowns:** `std::time::Instant` is not `Send` on some older targets; use
`tokio::time::Instant` or a simple `Arc<AtomicU64>` Unix-timestamp if cross-thread
sharing of the snooze deadline is needed. Check in Refinement.

---

## EP-20 — First-run setup screen + user-scoped config/secrets

**Type:** FEATURE · **Priority:** P2 · **Size:** M · **Roles:** DEV, DESIGN · **Depends on:** EP-6, EP-11

On first launch (no `~/.gurdo/secrets.toml` found), show a setup screen instead of the player. The user enters their Last.fm API key, Last.fm username, and Spotify client_id. The app writes them to `~/.gurdo/secrets.toml` (chmod 600) and a default `~/.gurdo/config.toml`, then transitions to the player. Subsequent launches skip the screen. This also moves the default config/secrets location from the working directory into `~/.gurdo/`, making the binary distribution self-contained — no manual file copying required.

**Scenarios:**
- As a new user who downloaded the binary, on first launch I see a simple setup screen asking for my Last.fm API key, username, and Spotify client_id. I fill them in, click Continue, and the player opens.
- As an existing user after this change ships, my credentials are migrated from `./secrets.toml` to `~/.gurdo/secrets.toml` automatically on first run, with no re-entry.
- As the user, running `gurdo` from any directory finds my config at `~/.gurdo/config.toml` — I don't need a `config.toml` next to the binary.
- As the user, the `-c <path>` flag still overrides the default location for power users.

**High-level acceptance criteria:**
- On launch, if `~/.gurdo/secrets.toml` does not exist, the setup screen is shown instead of the player.
- The setup screen collects Last.fm API key, Last.fm username, Spotify client_id; validates they are non-empty before enabling Continue.
- On Continue, writes `~/.gurdo/secrets.toml` (chmod 600) and `~/.gurdo/config.toml` (default values) then proceeds to the player.
- If `~/.gurdo/secrets.toml` already exists, the setup screen is skipped.
- `Config::load()` default path is `~/.gurdo/config.toml`; `-c <path>` overrides it.
- `Config::secrets_path()` always resolves to `~/.gurdo/secrets.toml` (not sibling of config.toml) so secrets stay in one predictable location regardless of the `-c` flag.
- One-time migration: if `./secrets.toml` exists next to the binary but `~/.gurdo/secrets.toml` does not, copy and inform the user.

**Out of scope:** multi-account support, credential validation against the live API during setup (check happens on first sync), UI polish beyond functional (full design pass is a follow-on), password-manager / keychain integration.

**Risks / unknowns:** The `~/.gurdo/` directory must exist before writing; create it with `std::fs::create_dir_all` on first run. The migration heuristic (copy from `./secrets.toml`) should be conservative — only run once and log what it did.

**Known blocker — Spotify OAuth on first run:** The legacy CLI exposed a `gurdo login` subcommand that ran the Spotify PKCE OAuth flow and wrote `~/.gurdo/spotify_token.json`. EP-2 (CLI removal) deleted that subcommand; EP-7 moves login in-process via the Settings → Spotify section. This means a brand-new user who goes through the EP-20 setup screen still cannot use the app until they open Settings and click **Login** (EP-7's Spotify Login button). EP-20's setup screen should make this explicit — either by including a "Connect Spotify" step as the final setup step (triggering EP-7's OAuth flow inline), or by showing a prominent post-setup prompt directing the user to Settings → Spotify. Refinement must decide which. Without this, a new user will have credentials set but no Spotify token, and the player will show a Spotify auth error on every poll cycle.

---

## Sequencing

Recommended iteration order (each numbered iteration is a self-contained refinement → development → verification → integration → retrospective cycle):

1. **EP-1 UI module split** — pure refactor, unblocks everything else.
2. **EP-3 Embedded assets + CJK fix** — independent; visible quick win, fixes a current user-facing bug.
3. **EP-2 CLI removal** — small; can come *after* EP-7 (otherwise the user temporarily loses access to syncs). Alternative: run after EP-7 lands.
4. **EP-4 Cover-blur background** — visible quality win, scoped change.
5. **EP-5 Placeholder cover** — small, builds on EP-3 assets + EP-1 split.
6. **EP-6 Settings viewport shell** — empty multi-window scaffold.
7. **EP-7 In-process operations** — the largest single epic; fills the Data section of EP-6.
8. **EP-8 Full knob exposure** — fills the algorithm sections of EP-6; absorbs the `TRACKS_PER_ARTIST` bug fix.
   ... at this point the MVP from Vision §8 is closed.
9. **EP-2** (if not already done) — drop `clap` once all in-process replacements are live.
10. **EP-9 Combined Update** — small follow-on.
11. **EP-12 Test scaffolding** — can interleave with the above iterations, but worth a dedicated pass once core code stabilizes.
12. **EP-10 Recommendation preview** — once knobs + ops are stable.
13. **EP-11 Secrets hardening** — when readying for multi-user.
14. **EP-13 Schema cleanup** — opportunistic.
15. **EP-14 Installer packaging** — when ready to share.
16. **EP-15 TC font** — on demand only.

**P1 → MVP close = EP-1, 2, 3, 4, 5, 6, 7, 8** (8 iterations).
