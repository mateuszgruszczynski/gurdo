# Vision — Gurdo (improve cycle)

*Phase: Vision · Mode: improve · Date: 2026-05-11*

Describes the desired end state of Gurdo after this improvement cycle. Frames the **delta** from the Analysis baseline — what should be added, changed, or removed. Algorithmic evolution (the [IDEAS.md](IDEAS.md) backlog) is **out of scope for this cycle**.

---

## 1. Product vision statement

Gurdo is a focused, single-user desktop player that surfaces personalised music recommendations driven by Last.fm history and explicit feedback, and plays them on Spotify Connect. After this cycle, Gurdo is *operable end-to-end from the UI* — no CLI, no terminal — with full algorithmic control exposed as discoverable, described, validated settings, and a Now Playing surface whose background visually responds to the current cover art without ever looking muddy.

---

## 2. Target user

A **single power user** running Gurdo on their own desktop machine. The current user (`grucha666`) is the only intended user; the app is not multi-tenant, not packaged for distribution, not authenticated beyond their own Last.fm + Spotify accounts. The user is technically literate (will edit a TOML file for one-time setup) but wants day-to-day operation through a UI.

---

## 3. App type and platform

- **Native desktop app** (single Rust binary, eframe/egui).
- **Primary target: macOS** (the dev machine), with Linux as a soft target since eframe/egui are cross-platform and the codebase contains no macOS-specific paths beyond a CJK font fallback. Windows is not a priority but not actively blocked.

---

## 4. Hard constraints

- **Language / framework locked** — Rust + eframe/egui 0.29 + rusqlite (bundled SQLite) + tokio + reqwest. Architecture and Backlog should not propose stack changes.
- **Spotify Connect is the only playback target.** Local audio playback is not in scope.
- **Last.fm is the only data source for history + similarity.** No MusicBrainz, no Discogs, no Spotify Audio Features.
- **Single-user, local-only.** No server, no sync between devices, no shared accounts.
- **Config file (`config.toml`) remains the source of truth** for first-time setup, credentials, and window dimensions. Algorithm tunables are also persisted there, but their primary edit surface is the UI.

---

## 5. Out of scope (explicit non-goals)

- **All [IDEAS.md](IDEAS.md) algorithm changes** — full artist history sync, loved-tracks→seeds, tag cleanup, depth-2 similar expansion, two-stage artist→track recommendation, playlist mix percentages, skip-handling. Each remains in the algorithm backlog and may run as a future change via `/agile-dev:change`.
- **Open questions from `plan`** — unknown→known→liked transition scoring, similar-artist score inflation. Deferred.
- **Removing/dropping the `similar_tracks` table** — Analysis flagged it as dead schema; left for a future cleanup pass to avoid scope creep here.
- **Multi-pane resizable layout** (queue list, library view, artist-score browser). The player stays a compact single-screen surface; the *settings* gets a bigger window. Building a queue/library browser is a future cycle.
- **Audio Features-driven recommendations** (BPM, key, energy from Spotify).
- **Test coverage uplift** beyond what is justified by the changes in this cycle. (We'll add tests where new logic warrants them, not retrofit historical code.)
- **Credential rotation / config-secret extraction** — flagged as tech debt for backlog but not a vision pillar.

---

## 6. Success criteria

A successful end of this cycle means **all** of the following are true:

1. **The CLI is gone.** `cargo run` launches the UI directly. No `clap` subcommand surface. The `clap` dependency is removed from `Cargo.toml`.
2. **Every operation that previously required a subcommand is reachable from the UI**, runs in-process (no `std::process::Command` spawning of `gurdo`), and shows live progress (which step, current artist, count done / total) and a clear final state (success / partial / failed with error).
3. **Every tunable in `config.toml` that affects recommendation/scoring/sync behaviour is editable from the UI**, grouped by config section, with a one-line description, validated min/max, and a per-field reset-to-default. Values persist back to `config.toml` on save.
4. **The Now Playing background is the current cover art, blurred, with a dark gradient overlay.** It is never muddy because no averaging step exists; the cover *is* the background. When no track is playing the background falls back to the static `[ui].background_color` from config.
5. **The player and settings live in separate OS windows.** Player window size is read from `config.toml`; settings window opens at a configurable larger size, also from config. The player remains close to today's compact layout.
6. **When nothing is playing, the player shows a cover-art placeholder** — a bundled image / icon (not an empty 400×400 hole). Used on first launch before any track has played, when Spotify reports no active playback, and during transient gaps between tracks.
7. **Non-Latin titles and artist names render correctly — including Chinese** (currently broken: CJK shows as tofu rectangles because the loaded `.ttc` collection isn't resolving to a usable glyph set in egui 0.29). The fix is to **embed a CJK-capable `.ttf` font in the binary** via `include_bytes!` so the app is self-contained and doesn't depend on the host machine having usable CJK system fonts. Cyrillic, Greek, and other Latin-extended scripts must also render. Architecture phase picks the specific font (Noto Sans CJK SC subset / Source Han Sans subset / similar — trade-off is binary size vs. glyph coverage).
8. **The app builds and runs without warnings** on the dev machine; the dead `extract_dominant_color` function and the commented-out bucketing code are removed (replaced by the blur-overlay approach).

Non-binary quality bar:
- The settings UI doesn't lose data on accidental scroll — values commit explicitly (Save button) or with clear visual feedback when changed.
- Sync progress is honest (no fake-progress spinners; real per-step counts).
- The app feels visually coherent — the player background, controls, and text remain legible on every cover, including pure-white, pure-black, and high-contrast ones.

---

## 7. Key user journeys

### Journey A — First launch on a new machine

1. User installs Rust, clones the repo, copies `config.toml.example` to `config.toml`, fills in `lastfm.api_key`, `lastfm.username`, `spotify.client_id`, saves.
2. User runs `cargo run --release` (or runs the prebuilt binary).
3. Gurdo opens the player window at the size configured in `[ui].player_window_size`. The window is empty (no track playing, no DB synced yet) — only the static background colour from `[ui].background_color` and a hint message.
4. User clicks ⚙ Settings. A separate, larger window opens at `[ui].settings_window_size`.
5. In Settings, user clicks **Sync Last.fm** in the "Data" section. A progress panel appears in-line: "Fetching loved tracks… 234/500", then "Syncing year charts… 2007/2024", etc. When done, the panel shows "Sync complete — 412 artists, 1 832 loved tracks."
6. User clicks **Expand similar artists** → progress runs (per-artist count). Then **Fetch top tracks** → same. Then **Recalculate scores** → near-instant.
7. User clicks **Spotify login** in the "Connect" section. A browser opens, user authorises, browser shows success; the Settings panel flips that field's status from "Not connected" to "Connected as <user>".
8. User closes Settings (or just switches focus back to the player window). User clicks **▶ / new queue** in the player. The first track resolves, plays on the active Spotify device, and the player's background blurs into that cover's colors with a dark overlay.

### Journey B — Daily use: vote and adjust

1. User opens Gurdo. The player window opens; if Spotify is already playing on another device, polling picks it up within ~5 s, the track info populates, and the background shifts to the current cover blurred.
2. A track plays the user dislikes. User clicks **👎 Dislike**. The track is recorded as disliked, the artist's score is recalculated, and Gurdo skips to the next track in the queue.
3. User feels recommendations are too safe today. Opens Settings → "Recommendations" section → bumps **artist_score_exponent** down from 1.0 to 0.6 (flattens the distribution = more diversity). Hovering the label shows the description; the field has a reset button. User clicks Save.
4. The next time the queue extends (after one or two tracks), the new exponent is in effect — the user notices more obscure picks surfacing.

### Journey C — Tuning the algorithm

1. User runs `recommend` (today's CLI command) — *in the new world*, user opens Settings → "Recommendations" section → clicks **Preview**.
2. A scrolling preview list of e.g. 20 (artist, track) pairs appears with each artist's `final_score` shown beside it.
3. User changes a knob (e.g. `similarity_multiplier`), clicks **Preview** again, and immediately sees how the candidate list shifts.
4. When the user is happy, they click **Save**, and the new values write back to `config.toml`.

*(Preview is a stretch goal — the success criterion is "every knob is exposed + persists", not "preview". Preview is a candidate iteration in the Backlog.)*

---

## 8. MVP definition

The minimum end-state that constitutes a successful close of this improvement cycle:

1. **CLI removed.** Binary launches into the UI. No subcommands. `clap` dropped.
2. **In-process operations from Settings:**
    - Sync Last.fm (loved tracks + tags + year-charts + external-artist scoring)
    - Expand similar artists
    - Fetch artist tracks
    - Recalculate scores
    - Spotify OAuth login
   …each with live progress (per-stage status text + per-step count where applicable) and clear pass/fail end-state.
3. **Full config-knob exposure in Settings.** Every numeric / boolean tunable in `[sync]`, `[engine]`, `[artist_scoring]`, `[recommendations]` is editable, with description + validation + reset-to-default + grouped layout. `[lastfm]`, `[spotify]`, `[app]`, `[ui]` are also visible but read-only or marked "restart required". Saves write back to `config.toml`.
4. **Two-window UI.** Player window opens at `[ui].player_window_size` (default same as today's 440×660). Settings opens as a separate window at `[ui].settings_window_size` (default e.g. 800×900). Both dimensions live in `config.toml` only — not user-editable from the UI.
5. **Cover-blur background.** The player's background is the current track's cover art, gaussian-blurred and overlaid with a dark vertical gradient for legibility. Falls back to `[ui].background_color` when no track is playing or no art is available. The legacy `extract_dominant_color` function and its commented bucketing/HSV code are deleted.
6. **Idle-state placeholder.** When no track is playing, the 400×400 art slot shows a bundled placeholder image (embedded in the binary via `include_bytes!`) rather than empty space.
7. **Unicode rendering fixed for CJK.** A CJK-capable font is embedded in the binary via `include_bytes!` and registered as a fallback in `egui::FontDefinitions` so Chinese / Japanese / Korean track and artist names render as glyphs rather than tofu rectangles. The existing OS-font-path probing in `app.rs` is removed in favor of the embedded font. Cyrillic, Greek, and other scripts covered by the default egui Proportional font continue to render.
8. **No regressions** in existing player functionality: play/pause/seek/next/prev/like/dislike/queue all behave as today.

Anything beyond this — preview, presets, queue list view, library browser, theming — is post-MVP and lives in the Backlog as separate epics.

---

## 9. What this cycle explicitly does *not* try to do

- It does **not** improve recommendation *quality*. It improves recommendation *controllability* (every knob exposed) and *visibility* (operations from UI with progress). Better recommendations are the next cycle.
- It does **not** refactor the entire UI. The player stays the player. The split is "ui/app.rs is 876 LOC and growing; carve out modules so the new settings can live somewhere reasonable" — but a wholesale rewrite is out of scope.
- It does **not** add tests retrospectively to the algorithm engine. Tests are added only where new logic warrants them (e.g., the background blur path, the new in-process sync orchestration with progress reporting).
- It does **not** address the committed-credentials issue or `.gitignore` hygiene beyond flagging it for the Backlog.
