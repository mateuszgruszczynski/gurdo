# Changelog

All notable changes to this project will be documented in this file.
Format based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [Iteration 017] — First-run setup screen + user-scoped config/secrets — 2026-05-17

### Added
- First-run setup wizard (`src/ui/setup.rs`): two-phase eframe window (440×400, not resizable) that gates app launch until credentials are present
  - Phase 1: three labeled text fields (Last.fm API Key, Last.fm Username, Spotify Client ID); Continue disabled until all non-empty
  - Phase 2: Spotify OAuth connect flow with inline status feedback; "Skip for now" bypasses OAuth
- `config::needs_setup(secrets_path)` — returns `true` when `~/.gurdo/secrets.toml` is absent, unparseable, or contains any empty/whitespace credential
- `config::gurdo_dir()` — returns `~/.gurdo/` via `dirs::home_dir()`
- `config::migrate_secrets_if_needed(gurdo_dir, cwd)` — one-time migration of `./secrets.toml` → `~/.gurdo/secrets.toml` for existing installations (copy-only, no-op if destination present)
- `README.md` — documents `~/.gurdo/` config and secrets paths, setup wizard flow
- `dirs = "5"` dependency for cross-platform home directory resolution

### Changed
- `Config::secrets_path()` — now always returns `~/.gurdo/secrets.toml` regardless of config path argument (invariant hardened)
- `Config::load()` refactored to `load_inner()` + `#[cfg(test)] load_with_secrets_at()` seam for test isolation
- `parse_config_arg()` default path changed from `./config.toml` to `~/.gurdo/config.toml`
- `src/main.rs` — runs `create_dir_all`, migration, and `needs_setup` check before loading config; setup wizard blocks player if credentials absent
- `config.toml.example` — header updated to reference `~/.gurdo/config.toml` as default

### Security
- `~/.gurdo/secrets.toml` receives `chmod 0o600` immediately after write (`#[cfg(unix)]`)
- No credential values (api_key, username, client_id, tokens) emitted in any `tracing::*` / `eprintln!` / `dbg!` call

Retro: iterations/017-first-run-setup-screen/i7-retro.md

---

## [Iteration 016] — Installer packaging — 2026-05-15

### Added
- `scripts/package.sh` — OS-detecting release packaging script; produces `dist/gurdo-<version>-linux-<arch>.tar.gz` on Linux and `dist/gurdo-<version>-macos-<arch>.zip` on macOS
- Archive contains the release binary and `assets/fonts/OFL.txt` only (no external runtime files needed; fonts are embedded)
- Script reads version from `Cargo.toml` at run time; exits non-zero if `cargo build --release` fails

### Changed
- `.gitignore` — added `/dist` to exclude generated archives from version control

Retro: iterations/016-installer-packaging/i7-retro.md

---

## [Iteration 015] — Dead-code cleanup (orphaned API surface) — 2026-05-15

### Removed
- 15 dead functions from `src/db/queries.rs` (top-artist/track upserts, tag reads, spotify URI read, chart period readers, clear_artists, etc.)
- `Image`, `TopArtist/Artists/Response`, `TopTrack/Tracks/Response`, `TagTopTrack/Tracks/Response` model structs from `src/lastfm/models.rs`
- Dead struct fields: `ArtistRef.mbid/url`, `SimilarArtist.mbid`, `WeeklyChartEntry.to`, `PageAttr.page/total`; dead methods `to_ts`, `total_u32`
- `user_top_artists`, `user_top_tracks`, `tag_top_tracks` from `src/lastfm/client.rs`
- 8 dead Spotify model structs (SavedTrack family, Playlist family) and 5 dead Device/AlbumImage fields
- `bearer`, `get_liked_songs`, `get_playlists`, `get_playlist_tracks`, `save_track`, `remove_saved_track` from `src/spotify/client.rs`

### Fixed
- Build warnings reduced from 47 → 1 (only `last_track_uri` value-assignment in poll.rs remains — deferred, requires logic change)

Retro: iterations/015-dead-code-cleanup/i7-retro.md

---

## [Iteration 014] — Schema cleanup: similar_tracks drop — 2026-05-15

### Removed
- `similar_tracks` table and `idx_similar_tracks_seed` index from schema
- `upsert_similar_track`, `get_similar_tracks_for_seed`, `is_track_synced_for_similar` from `src/db/queries.rs`
- `track_similar` method and `SimilarTracksResponse`/`SimilarTracks`/`SimilarTrack` structs from `src/lastfm/`
- `similar_tracks_limit` config field, default fn, and `config.toml`/`config.toml.example` entries

### Fixed
- `init_db()` migration now drops `similar_tracks` on existing databases
- Build warnings reduced from 53 → 47

Retro: iterations/014-schema-cleanup-similar-tracks-drop/i7-retro.md

---

## [Iteration 013] — Test scaffolding — 2026-05-15

### Added
- `RecordingReporter` in `src/progress.rs` (`#[cfg(test)]`) — captures stage/tick/message/finish events for unit tests
- `weighted_sample` unit tests (deterministic seed, single weight, equal weights) in `src/engine/recommend.rs`
- `generate_recommendations` component test with in-memory SQLite fixture (2 artists, 3 tracks each)
- `db::queries` round-trip tests: `upsert_artist_external`/`get_all_artists_ranked`, `upsert_artist_top_track`/`get_all_artist_top_tracks`, `get_scoreable_artists_with_tracks` filter, `recalculate_all_scores` formula

### Notes
- `upsert_artist_external` preserves case as received from the API; test fixtures use lowercase to match storage
- 16 tests total (7 pre-existing + 9 new); `cargo build` stays at 53 warnings

Retro: iterations/013-test-scaffolding/i7-retro.md

---

## [Iteration 012] — Secrets hardening & multi-user config — 2026-05-13

### Added
- `secrets.toml` sibling pattern: create a `secrets.toml` file alongside `config.toml`
  with your `[lastfm]` `api_key`/`username` and `[spotify]` `client_id`; `Config::load`
  overlays those three fields automatically.
- `src/config.rs`: `Config::secrets_path()`, private `load_secrets()`, `SecretsConfig`
  family of structs. Three new unit tests: `secrets_path_is_sibling`,
  `load_overlays_secrets_when_present`, `load_uses_config_values_when_secrets_absent`.
- `Cargo.toml`: `[dev-dependencies] tempfile = "3"` for temp-dir unit tests.

### Changed
- `config.toml`: real `api_key`, `username`, `client_id` replaced with `YOUR_*`
  placeholders — safe to commit.
- `config.toml.example`: same placeholder cleanup; added documentation block explaining
  `secrets.toml` format.
- `.gitignore`: `secrets.toml` added so the file is never accidentally committed.

**Backward compatible:** existing `config.toml` files with real secrets continue to work
unchanged. Create `secrets.toml` and clear `config.toml` when ready.

Retro: iterations/012-secrets-hardening/i7-retro.md

---

## [Iteration 011] — Recommendation preview-while-tuning — 2026-05-12

### Added
- `src/ui/settings.rs`: **Preview** button in the Recommendations section (disabled while
  any operation is active). Clicking sends `OperationCommand::Preview` using the current
  draft config (unsaved knob changes are reflected immediately).
- `src/ui/settings.rs`: Preview results panel — bounded-height (300 px) scroll area with
  one row per result: `"Artist — Track"` left, score right-aligned as `"0.00"`. Persists
  until re-run or Discard.
- `src/ui/state.rs`: `preview_results: Option<Vec<(String, String, f64)>>` field on
  `OperationsState`; `Preview` variant on `OperationCommand`.

### Changed
- `src/engine/recommend.rs`: `generate_recommendations` now returns
  `Vec<(String, String, f64)>` — third element is the sampled artist's `final_score`.
- `src/ui/poll.rs`: both `for (artist, track)` loops updated to `(artist, track, _score)`.
- `src/ui/ops.rs`: `ops_dispatcher_loop` gains `settings_draft` parameter; `Preview`
  branch reads draft-or-live config, runs `generate_recommendations` synchronously,
  stores results into `ops.preview_results` or falls back to `last_result = Failed`.
- `src/ui/mod.rs`: `settings_draft` Arc created before the background thread spawn so it
  can be shared between the dispatcher and `GurdoApp`.
- `src/ui/settings.rs`: Discard now also clears `preview_results`.

Retro: iterations/011-recommendation-preview/i7-retro.md

---

## [Iteration 010] — Combined "Update everything" action — 2026-05-12

### Added
- `src/ui/state.rs`: `OperationCommand::UpdateAll` variant; `step: Option<(u8, u8)>` field
  on `ActiveOperation` — `Some((current, total))` during multi-step sequences, `None` for
  single-op dispatch.
- `src/ui/settings.rs`: "Update everything" button in the Data section, above the four
  individual operation buttons; disabled while any operation is active.

### Changed
- `src/ui/ops.rs`: `ops_dispatcher_loop` restructured from `while let Some(Run(kind))`
  to `while let Some(cmd) { match cmd { Run … UpdateAll … } }`. `UpdateAll` iterates
  `[SyncLastfm, Expand, FetchTracks, Score]`; on any step failure, clears `active` and
  sets `last_result = Failed("Step N/4 (name) failed: …")` then stops the chain; on full
  success sets `last_result = Ok("Update complete (4 steps)")`. `Run` branch sets
  `step: None` (unchanged behaviour).
- `src/ui/settings.rs`: progress label now prefixes `Step n/t: ` when `active.step` is
  `Some`, falling back to the existing `kind: stage` display for single ops.

Retro: iterations/010-update-everything/i7-retro.md

---

## [Iteration 009] — CLI removal & entry-point collapse — 2026-05-12

### Changed
- `src/main.rs`: rewritten from 217 lines to 46. Sync `fn main`; manual `-c`/`--config`
  flag parsing via `parse_config_arg()`. All 8 CLI subcommands removed
  (SyncLastfm, Expand, Score, Recommend, FetchTracks, Login, Devices, Play).
- `Cargo.toml`: `clap` dependency removed. `cargo tree` confirms it is no longer a
  transitive dependency.
- `gurdo` now launches the UI directly with no subcommand. Optional `-c <path>` selects
  a non-default config file.

**MVP close** — all P1 epics (EP-1 through EP-8, EP-17) are now DONE.

Retro: iterations/009-cli-removal/i7-retro.md

---

## [Iteration 008] — Full config-knob exposure — 2026-05-12

### Added
- `src/config.rs`: `impl Default for SyncConfig` and `impl Default for EngineConfig`
  (needed for per-field Reset-to-default in Settings).
- `src/ui/knobs.rs`: `KnobSpec` struct + four static metadata slices (`SYNC_KNOBS`,
  `ENGINE_KNOBS`, `ARTIST_SCORING_KNOBS`, `RECOMMEND_KNOBS`) for future EP-10 use.

### Changed
- `src/ui/settings.rs`: Recommendations, Engine, Artist Scoring, Sync sections filled with
  `DragValue` knobs (drag-to-edit, tooltip, per-field ↺ Reset). Appearance section shows
  read-only paths and identifiers. `• Save` / `Discard changes` block appears when dirty;
  Save writes to `config.toml` and updates `shared_config` in-process.
- `src/ui/player.rs`: `settings_draft: Arc<Mutex<Option<Config>>>` field; passes
  `shared_config`, `settings_draft`, `config_path` into settings viewport closure.
- `src/ui/mod.rs`: `settings_draft` wired into `GurdoApp` init.
- `src/sync/expand.rs`: `const TRACKS_PER_ARTIST` deleted; `fetch_artist_tracks` gains
  `config: &Config` parameter and reads `config.engine.artist_top_tracks_limit`.
- `src/ui/ops.rs`, `src/main.rs`: updated `fetch_artist_tracks` callsites.
- Removed `#[allow(dead_code)]` from `Config::save` and `GurdoApp.config_path`
  (both now actively used).

Retro: iterations/008-config-knob-exposure/i7-retro.md

---

## [Iteration 007] — In-process operations + progress — 2026-05-12

### Added
- `src/progress.rs`: `ProgressReporter` trait (`stage`, `tick`, `message`, `finish`) + `NullProgress`
  no-op; top-level module shared by sync, engine, and UI layers.
- `src/ui/state.rs`: `OperationsState`, `ActiveOperation`, `OperationKind`, `OperationResult`,
  `OperationCommand` — operation lifecycle types.
- `src/ui/ops.rs`: `StateReporter` (writes directly to `Arc<Mutex<OperationsState>>`);
  `ops_dispatcher_loop` tokio task; `token_exists` helper; 3 unit tests.

### Changed
- `src/ui/mod.rs`: creates ops channel + `OperationsState`; runs `ops_dispatcher_loop`
  alongside `polling_loop` via `tokio::join!` in the background thread.
- `src/ui/player.rs`: added `ops_state` + `ops_cmd_tx` fields to `GurdoApp`; forwards to
  `settings::render` via deferred viewport closure.
- `src/ui/settings.rs`: Data section — 4 operation buttons, live progress panel, last-result
  line. Spotify section — Login button + connected/not-connected status. Live repaint every
  100 ms while op active.
- `src/sync/mod.rs`: `sync_lastfm` gains `progress: &dyn ProgressReporter`; emits stage/tick
  per loved track and tag; delegates year ticks to `sync_artist_history`.
- `src/sync/artists.rs`: `sync_artist_history` gains `progress`; ticks per year.
- `src/sync/expand.rs`: `expand_artists` and `fetch_artist_tracks` gain `progress`; emit
  stage + tick per source artist.
- `src/engine/artist_scores.rs`: `score_artists` gains `progress`; emits stage + finish.
- `src/main.rs`: CLI callers pass `&NullProgress` to modified sync/engine functions.

Retro: iterations/007-in-process-operations/i7-retro.md

---

## [Iteration 006] — Spotify API error suppression + status indicator — 2026-05-12

### Added
- `PlayerState.api_error_snooze_until: Option<Instant>` — snooze deadline shared
  between poll thread and UI thread.
- "Snooze 10 min" button in the error modal; suppresses background poll error modals
  for 10 minutes from click time.
- `⚠ Spotify API unavailable` warning replaces the progress time label while snoozed
  (no layout shift).

### Changed
- `poll.rs`: background error paths (`do_poll`, `extend_queue_if_needed`) use
  `set_background_error` helper that skips writing `state.error` while snoozed.
  Successful poll clears the snooze immediately. Explicit user-action errors
  (`handle_cmd`) unchanged — always surface the modal.

Retro: iterations/006-spotify-error-suppression/i7-retro.md

---

## [Iteration 005] — Settings viewport window — 2026-05-12

### Added
- `src/ui/settings.rs`: `pub(super) fn render` — scrollable Settings OS window with 7
  placeholder sections (Data, Spotify, Recommendations, Engine, Artist Scoring, Sync,
  Appearance) and an in-window Close button.
- `src/config.rs`: `UiConfig.player_window_size` (default `[440, 660]`) and
  `settings_window_size` (default `[800, 900]`); serde-defaulted for back-compat.

### Changed
- `src/ui/player.rs`: replaced inline `egui::Window` Settings modal with
  `ctx.show_viewport_deferred`; `settings_open: bool` → `Arc<AtomicBool>`;
  `settings_initial_pos` stores center position at open time.
- `src/ui/mod.rs`: player window size driven by `config.ui.player_window_size`.
- Removed `SettingsDraft` and `spawn_cli` (knob controls return in EP-8).

### Fixed
- In-window Close button now closes instantly via `ctx.request_repaint_of(ROOT)`.

Retro: iterations/005-settings-viewport-window/i7-retro.md

---

## [Iteration 004] — Idle-state placeholder cover — 2026-05-12

### Added
- `src/ui/player.rs`: `placeholder_texture: Option<egui::TextureHandle>` field on
  `GurdoApp`; lazy-decoded from `assets::PLACEHOLDER_COVER` on first `update()` frame.
- Placeholder renders in the 400×400 cover slot (rounding 10.0) whenever no track is
  playing — replaces the previous invisible `allocate_space` gap.

### Changed
- `src/ui/assets.rs`: removed `#[allow(dead_code)]` suppressor from `PLACEHOLDER_COVER`
  (constant is now actively used).

Retro: iterations/004-idle-state-placeholder-cover/i7-retro.md

---

## [Iteration 003] — Cover-blur background painter — 2026-05-12

### Added
- `src/ui/background.rs`: `BackgroundPainter` — off-thread blur pipeline
  (decode → `resize_exact(256×256, Triangle)` → `fast_blur(sigma=30)`) delivered
  via `Arc<Mutex<Option<ColorImage>>>` slot; `ctx.request_repaint()` on completion
  for near-instant response.
- Full-window blurred cover art as player background with dark vertical gradient
  overlay (`rgba(0,0,0,60)` → `rgba(0,0,0,200)`) for text/control legibility.
- Solid `background_color` fallback when no track is playing.

### Changed
- `src/ui/player.rs`: wired `BackgroundPainter`; removed static `panel_fill`
  assignment (now handled by `BackgroundPainter::paint`).

Retro: iterations/003-cover-blur-background/i7-retro.md

---

## [Iteration 002] — Embedded assets + CJK font fix — 2026-05-12

### Fixed
- CJK track titles (Japanese, Korean, Simplified Chinese) now render with correct glyphs
  instead of tofu rectangles on machines without system CJK fonts.
- Deleted OS-font-path probe loop (`cjk_paths` array + `for` loop in `src/ui/mod.rs`);
  replaced with embedded font registration (JP → SC → KR fallback on Proportional family).

### Added
- `assets/fonts/NotoSansJP-Regular.otf` (4.4 MB, OpenType-CFF, OFL 1.1)
- `assets/fonts/NotoSansSC-Regular.otf` (8.0 MB, OpenType-CFF, OFL 1.1)
- `assets/fonts/NotoSansKR-Regular.otf` (4.5 MB, OpenType-CFF, OFL 1.1)
- `assets/fonts/OFL.txt` (SIL Open Font License 1.1)
- `assets/images/placeholder_cover.png` (400×400 gray, 1 KB, author-created; displayed in EP-5)
- `src/ui/assets.rs`: `NOTO_SANS_JP`, `NOTO_SANS_SC`, `NOTO_SANS_KR`, `PLACEHOLDER_COVER`
  constants via `include_bytes!`.

Binary: 24 MB → 41 MB (+17 MB from three embedded Noto Sans OTF files).

Retro: iterations/002-embedded-assets-cjk-fix/i7-retro.md

---

## [Iteration 001] — UI module split — 2026-05-12

### Changed
- Split `src/ui/app.rs` (876 LOC) into nine focused modules under `src/ui/`:
  `state.rs` (PlayerState, PlayerCommand), `poll.rs` (polling loop + command handlers),
  `player.rs` (egui rendering, SettingsDraft), `mod.rs` (entry point + font setup),
  plus five skeleton stubs for upcoming epics (EP-3/4/6/7/8).
- `src/ui/app.rs` deleted; `pub fn run` re-exported unchanged from `mod.rs`.
- Removed dead code: `extract_dominant_color` function and its commented call site.

Retro: iterations/001-ui-module-split/i7-retro.md
