# Iteration 1 Spec — UI module split (EP-1)

*Epic: EP-1 · Type: REFACTOR · Priority: P1 · Size: M · Date: 2026-05-11*

---

## 1. Context and goal

`src/ui/app.rs` is a 876-line file that mixes at least seven distinct concerns: shared player state, command definitions, settings form data, cover-art decoding, font loading, playback polling, and the egui widget tree. Every planned downstream feature — multi-window UI, blur background, in-process background operations, knob exposure — needs a dedicated home that does not yet exist. This iteration carves the existing code into the module structure decided in Architecture §4.7. The split is a pure refactor: no logic changes, no new features, no user-visible behaviour change. Its sole deliverable is a `src/ui/` directory whose contents match the target layout, with the project building cleanly and all existing app behaviour preserved.

---

## 2. Scope

**In scope**
- Delete `src/ui/app.rs`.
- Rewrite `src/ui/mod.rs` to declare all new sub-modules and re-export `pub fn run`.
- Create `src/ui/state.rs` containing `PlayerState` and `PlayerCommand` (moved verbatim).
- Create `src/ui/poll.rs` containing `do_poll`, `handle_cmd`, `extend_queue_if_needed`, `polling_loop`, and `QUEUE_CHUNK_SIZE` (moved verbatim).
- Create `src/ui/player.rs` containing `GurdoApp`, `impl eframe::App for GurdoApp`, `SettingsDraft`, `decode_image`, `fmt_ms`, and `spawn_cli` (moved verbatim).
- Move the `pub fn run` entry point into `src/ui/mod.rs`.
- Delete `extract_dominant_color` and its commented-out call site — dead code, never called.
- Create skeleton files that compile without warnings: `src/ui/settings.rs`, `src/ui/background.rs`, `src/ui/ops.rs`, `src/ui/assets.rs`, `src/ui/knobs.rs`.

**Out of scope**
- Any new functionality (settings window redesign, blur pipeline, in-process background ops, knob widgets).
- Future types `OperationsState`, `OperationCommand`, `ProgressEvent` — these are EP-7 concerns and must not appear in this iteration.
- Any changes to logic in `do_poll`, `handle_cmd`, `extend_queue_if_needed`, or `polling_loop`.
- Any changes to the egui widget code in `GurdoApp::update`.
- Adding tests — EP-1 is verified by build cleanliness and manual smoke-test.

---

## 3. Module allocation

| File | Contents | Notes |
|---|---|---|
| `src/ui/mod.rs` | `pub fn run(config, config_path)` entry point; `mod` declarations for all sub-modules | Rewritten from scratch; owns channel creation, `Arc<Mutex>` wiring, font setup, `eframe::run_native` call |
| `src/ui/state.rs` | `PlayerState` struct, `PlayerCommand` enum | Moved verbatim from `app.rs`; both items must be `pub` |
| `src/ui/player.rs` | `GurdoApp` struct, `impl eframe::App for GurdoApp`, `SettingsDraft` struct + impl, `decode_image`, `fmt_ms`, `spawn_cli` | Entire egui render tree, error modal, and settings window stay here until EP-6 |
| `src/ui/poll.rs` | `polling_loop`, `do_poll`, `handle_cmd`, `extend_queue_if_needed`, `QUEUE_CHUNK_SIZE` | Async; imports `PlayerState`/`PlayerCommand` from `super::state` |
| `src/ui/settings.rs` | Placeholder comment only (`// placeholder — EP-6`) | Must be a valid Rust module file with no dead-code warnings |
| `src/ui/background.rs` | Placeholder comment only (`// placeholder — EP-4`) | Same requirement |
| `src/ui/ops.rs` | Placeholder comment only (`// placeholder — EP-7`) | Same requirement |
| `src/ui/assets.rs` | Placeholder comment only (`// placeholder — EP-3`) | Same requirement |
| `src/ui/knobs.rs` | Placeholder comment only (`// placeholder — EP-8`) | Same requirement |

`extract_dominant_color` does not appear in any file — it is deleted entirely.

---

## 4. Acceptance Criteria

**AC-1 — Target files exist**

After the change, the directory `src/ui/` contains exactly these files: `mod.rs`, `state.rs`, `player.rs`, `poll.rs`, `settings.rs`, `background.rs`, `ops.rs`, `assets.rs`, `knobs.rs`. The file `app.rs` is absent.

*Verification:* `ls src/ui/` lists the nine files above and nothing else.

*BDD scenario:*
```
Given the refactor has been applied
When I list the contents of the src/ui/ directory
Then I see exactly mod.rs, state.rs, player.rs, poll.rs, settings.rs, background.rs, ops.rs, assets.rs, and knobs.rs
And I do not see app.rs
```

---

**AC-2 — Project builds with zero errors and zero warnings**

Running `cargo build` (debug profile, default features) exits with status 0 and emits no `warning:` lines to stderr.

*Verification:* `cargo build 2>&1 | grep -c "^warning:"` returns `0`; `echo $?` after the build returns `0`.

*BDD scenario:*
```
Given the refactor has been applied
When I run the build command for the project
Then the build completes successfully
And no warning messages are printed to the output
```

---

**AC-3 — The run entry point is accessible from outside the ui module**

The symbol `ui::run` is callable from `src/main.rs` with the same signature it had before: it accepts a `Config` value and a `PathBuf` and returns `anyhow::Result<()>`. No import path changes are needed in any file outside `src/ui/`.

*Verification:* `src/main.rs` compiles unchanged (no edits to the call site); confirming AC-2 is sufficient if main.rs is not modified.

*BDD scenario:*
```
Given the refactor has been applied
And src/main.rs has not been edited
When I build the project
Then the build succeeds, confirming that ui::run is still reachable with its original signature
```

---

**AC-4 — Dead code is removed, not moved**

The function `extract_dominant_color` does not appear anywhere in the codebase. The commented-out line `// self.bg_color = extract_dominant_color(bytes);` is also absent.

*Verification:* `grep -r "extract_dominant_color" src/` returns no output.

*BDD scenario:*
```
Given the refactor has been applied
When I search the entire source tree for the text "extract_dominant_color"
Then no results are found
```

---

**AC-5 — Skeleton files contain no compilable Rust items**

Each of `settings.rs`, `background.rs`, `ops.rs`, `assets.rs`, and `knobs.rs` contains only comments. None of them declares any `struct`, `enum`, `fn`, `trait`, `impl`, `const`, `static`, or `use` item. This guarantees that unused-import or dead-code warnings cannot be triggered by stub code.

*Verification:* Reading each file shows comment-only content; AC-2 (zero warnings) is a runtime confirmation.

*BDD scenario:*
```
Given the refactor has been applied
When I open each skeleton file (settings.rs, background.rs, ops.rs, assets.rs, knobs.rs)
Then each file contains only comment lines and no Rust declarations
```

---

**AC-6 — App launches and displays the player window**

Running the compiled binary opens a single window titled "Gurdo" with dimensions approximately 440×660 pixels. The window contains album art area, track name label, artist name label, progress bar, five playback buttons, and the like/dislike/queue/settings buttons — identical in appearance to the pre-refactor build.

*Verification:* Launch `cargo run -- --config config.toml` (or equivalent invocation), observe the window. A side-by-side screenshot comparison with a pre-refactor build is acceptable evidence.

*BDD scenario:*
```
Given the app has been built after the refactor
When I launch the app with a valid config pointing to an active Spotify account
Then a single window titled "Gurdo" appears
And the window shows an album art area, track name, artist name, a progress bar, and playback controls
And the layout and window size are unchanged from before the refactor
```

---

**AC-7 — Playback controls function correctly**

Each of the five transport buttons (previous, seek back 10 s, play/pause, seek forward 10 s, next) sends the correct command to Spotify and the UI reflects the result — without any regression compared to the pre-refactor build.

*Verification:* With Spotify playing, click each button and observe: play/pause toggles the icon and pauses/resumes playback; next/previous change the track; seek buttons shift the progress bar position by approximately 10 seconds.

*BDD scenario:*
```
Given the app is running and Spotify is playing a track
When I click the play/pause button
Then Spotify pauses and the button icon changes to the play symbol
When I click play/pause again
Then Spotify resumes and the button icon changes to the pause symbol
When I click the next track button
Then Spotify advances to the next track and the track name updates within a few seconds
```

---

**AC-8 — Like and dislike record feedback and advance the queue**

Clicking "Like" on a playing track marks it as liked in the database and the button label changes to "Unlike". Clicking "Dislike" marks it as disliked, skips to the next track, and the dislike button turns red. Clicking "Unlike" removes the like. These are identical behaviours to the pre-refactor build.

*Verification:* Manual test with a known track; verify database row via `sqlite3`.

*BDD scenario:*
```
Given the app is running and a track is playing
When I click the Like button
Then the button label changes to "Unlike" and the button text turns green
When I click Unlike
Then the button label returns to "Like" and the green colour is gone

Given the app is running and a different track is playing
When I click the Dislike button
Then the button turns red, the track is skipped, and the next track begins playing
```

---

**AC-9 — Settings modal opens and saves config changes**

Clicking the settings gear button opens the Settings window. Changing any numeric slider saves the new value to `config.toml` immediately (without a separate save button). The four Last.fm CLI buttons (Sync, Score, Expand, Fetch Tracks) remain present and clickable. Closing the window with the × button dismisses it. All of this is identical to pre-refactor behaviour.

*Verification:* Open settings, change "Queue size", close the app, inspect `config.toml` to confirm the value changed.

*BDD scenario:*
```
Given the app is running
When I click the settings gear button
Then a Settings window appears with sliders for queue size, exponents, and other parameters
When I drag the Queue size slider to a new value
Then the settings window stays open and no error appears
When I close the app and reopen config.toml
Then the queue size field reflects the value I set
```

---

**AC-10 — Error modal appears and can be dismissed**

When the polling thread sets an error (e.g., Spotify token expired), a centred modal titled "⚠  Error" displays the error message with "OK" and "Ignore" buttons. Clicking either button dismisses the modal. This behaviour is unchanged from pre-refactor.

*Verification:* Revoke the Spotify token, launch the app, observe the modal; click OK and confirm it closes.

*BDD scenario:*
```
Given the app is running and the Spotify token has been revoked
When the polling loop fails to refresh the token
Then a centred error modal appears with the error text
When I click OK
Then the modal closes and the main player window is visible again
```

---

**AC-11 — Queue button triggers a new recommendation queue**

Clicking the queue button (☰) generates a new recommendation queue from the database and begins playing it on Spotify. This behaviour is unchanged from pre-refactor.

*Verification:* Ensure `fetch-tracks` has been run so recommendations exist; click ☰; observe Spotify begins playing a new track from the recommended set within a few seconds.

*BDD scenario:*
```
Given the app is running, a Spotify device is active, and the database contains track recommendations
When I click the queue button
Then Spotify begins playing a recommended track within approximately 5 seconds
And subsequent tracks are also from the recommended set
```

---

**AC-12 — CJK track and artist names render without placeholders**

Track names and artist names containing Japanese, Chinese, or Korean characters display correctly in the window (no tofu blocks or question marks), identical to pre-refactor behaviour.

*Verification:* Play a Japanese track on Spotify; observe the track name in the Gurdo window renders in readable CJK glyphs.

*BDD scenario:*
```
Given the app is running and a track with a Japanese title is playing on Spotify
When the player window updates with the current track info
Then the track name and artist name are displayed in readable Japanese characters, not placeholder squares
```

---

## 5. Edge cases and failure modes

**Split-induced deadlock.** The polling loop holds `Arc<Mutex<PlayerState>>` across await points in several places (e.g., reading `album_art_bytes` then awaiting an HTTP fetch). If the mutex guard is inadvertently kept alive across an `.await`, the UI thread — which also locks `PlayerState` on every frame — will deadlock. Mitigation: all mutex guards in `poll.rs` must be dropped before any `.await`. The existing code already does this; the split must not rearrange that.

**Circular imports.** `poll.rs` imports from `state.rs`. `player.rs` imports from `state.rs`. `mod.rs` imports from both. No module may import from `player.rs` or `poll.rs` — only `mod.rs` orchestrates them.

**Visibility.** `PlayerState` and `PlayerCommand` must be `pub` in `state.rs`, and `state` must be a `pub(super)` or `pub` module in `mod.rs`, so that `player.rs` and `poll.rs` can reference `super::state::PlayerState`. `GurdoApp` and `SettingsDraft` remain private to `player.rs` — they are not part of the public API.

**Skeleton files and dead-code lints.** An empty `pub fn placeholder() {}` in a skeleton file would trigger an `unused` warning. Use comment-only files to avoid this entirely (AC-5).

**`extract_dominant_color` removal.** The function references `image` and `egui` and contains substantial dead commented-out code. Removing it eliminates a potential future dead-code warning. Confirm there is no other call site with `grep -r "extract_dominant_color" src/`.

**`SettingsDraft` visibility.** `SettingsDraft` is instantiated inside `run()` in `mod.rs` (via `SettingsDraft::from_config(...)`) and then passed into `GurdoApp`. After the split, `run()` lives in `mod.rs` and `SettingsDraft` lives in `player.rs`. `SettingsDraft` must therefore be at least `pub(super)` to be visible from `mod.rs`.

---

## 6. Notes for implementation

**Recommended order of edits**

1. Create `src/ui/state.rs` first — it has no dependencies within `src/ui/`.
2. Create `src/ui/poll.rs` — depends only on `state.rs` and external crates.
3. Create `src/ui/player.rs` — depends on `state.rs`; the egui render tree and `SettingsDraft` are self-contained.
4. Rewrite `src/ui/mod.rs` — wire everything together; move `pub fn run` here.
5. Create the five skeleton files.
6. Delete `src/ui/app.rs`.
7. Run `cargo build`; fix any visibility or import errors before proceeding.

**Preserving exact logic.** Do not simplify, inline, or reformat any logic during the move. The goal is a textual relocation, not a cleanup. Cleanup is deferred to the simplify pass after green build.

**`SettingsDraft::from_config` call site.** In the original code, `SettingsDraft::from_config(...)` is called inside `run()`. After the split `run()` is in `mod.rs` and `SettingsDraft` is in `player.rs`. Make `SettingsDraft` and `from_config` at minimum `pub(super)` so `mod.rs` can call it, or move the construction into a `GurdoApp::new()` constructor if preferred — either is acceptable as long as AC-2 (zero warnings) is satisfied.

**Polling thread ownership.** The `Arc<Mutex<PlayerState>>` and `Arc<Mutex<Config>>` are created in `run()` and cloned into the background thread. The clones are the only references held by `poll.rs`; `mod.rs` retains the originals to pass into `GurdoApp`. This two-clone pattern must be preserved exactly.

**No additions to `state.rs`.** Do not add `OperationsState`, `OperationCommand`, or `ProgressEvent` — these are EP-7 stubs and their absence is intentional in EP-1.
