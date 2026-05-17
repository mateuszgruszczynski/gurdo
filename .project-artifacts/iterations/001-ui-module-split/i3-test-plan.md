# Iteration 1 Test Plan — UI module split (EP-1)

*Epic: EP-1 · Phase: Test Plan · Date: 2026-05-11*

---

## Level summary

| Level | Count | Owned by | Notes |
|---|---|---|---|
| Component (in-process) | 5 | Development | Build + structural checks; run inside dev container |
| E2E / UI (out-of-process) | 7 | Verification | Manual smoke on host (hybrid mode — GUI cannot run in container) |

No Unit-level scenarios: this epic relocates code verbatim; there is no new logic to test in isolation. No System-integration scenarios: there are no new interfaces or wire protocols added.

---

## Component scenarios (Development-owned)

### T-01
**Feature:** UI module split  
**Scenario:** Project builds without errors after the refactor  
**Covers AC:** AC-2, AC-3  
**Level:** Component  
**Type:** File-batch (build output)  
**Owned by:** Development

```
Given the refactor has been applied
When I run the project build command in debug mode
Then the build completes with exit code 0
And no warning messages appear in the build output
```

*Notes:* The success of this scenario transitively verifies AC-3 (ui::run is reachable from main.rs), since main.rs is not modified.

---

### T-02
**Feature:** UI module split  
**Scenario:** Target module files exist and app.rs is absent  
**Covers AC:** AC-1  
**Level:** Component  
**Type:** File-batch (directory listing)  
**Owned by:** Development

```
Given the refactor has been applied
When I list the contents of the src/ui/ directory
Then I see exactly these files: mod.rs, state.rs, player.rs, poll.rs, settings.rs, background.rs, ops.rs, assets.rs, knobs.rs
And I do not see app.rs in that directory
And I see no other .rs files in that directory
```

*Notes:* Verified in-process by `ls src/ui/`. Combined with T-01 this confirms all modules compile and link.

---

### T-03
**Feature:** UI module split  
**Scenario:** Dead code `extract_dominant_color` is fully absent  
**Covers AC:** AC-4  
**Level:** Component  
**Type:** File-batch (source search)  
**Owned by:** Development

```
Given the refactor has been applied
When I search the entire source directory for the text "extract_dominant_color"
Then no results are found in any file
```

*Notes:* Verified by `grep -r "extract_dominant_color" src/`. The commented call site (`// self.bg_color = ...`) must also be absent.

---

### T-04
**Feature:** UI module split  
**Scenario:** Skeleton files contain no compilable Rust declarations  
**Covers AC:** AC-5  
**Level:** Component  
**Type:** File-batch (file content inspection)  
**Owned by:** Development

```
Given the refactor has been applied
When I open each skeleton file: settings.rs, background.rs, ops.rs, assets.rs, knobs.rs
Then each file contains only comment lines
And no file declares a struct, enum, function, trait, or import
```

*Notes:* T-01 (zero warnings) provides runtime confirmation — any Rust item in a skeleton file would trigger an unused-item lint.

---

### T-05
**Feature:** UI module split — regression (existing codebase)  
**Scenario:** Future-typed items are not present in state.rs  
**Covers AC:** AC-1 (scope constraint)  
**Level:** Component  
**Type:** File-batch (source search)  
**Owned by:** Development

```
Given the refactor has been applied
When I search src/ui/state.rs for the identifiers "OperationsState", "OperationCommand", "ProgressEvent"
Then none of those identifiers appear in the file
```

*Notes:* Guards against accidentally adding EP-7 stubs. These types must not exist until EP-7 lands.

---

## E2E / UI scenarios (Verification-owned, manual on host)

> **Hybrid mode:** These scenarios must be run on the host machine, not inside the dev container. Build in the container with `cargo build --release`, then copy the binary to the host or run `cargo run` from a host terminal.

---

### T-06
**Feature:** UI module split — regression  
**Scenario:** App launches and displays the player window  
**Covers AC:** AC-6  
**Level:** E2E  
**Type:** UI  
**Owned by:** Verification

```
Given the app has been built after the refactor
And a valid config.toml pointing to an active Spotify account exists
When I launch the app
Then a single window titled "Gurdo" appears on screen
And the window contains an album art area, track name, artist name, a progress bar, and playback control buttons
And the window size is approximately 440×660 pixels, consistent with the pre-refactor build
```

---

### T-07
**Feature:** UI module split — regression  
**Scenario:** Transport controls send the correct Spotify commands  
**Covers AC:** AC-7  
**Level:** E2E  
**Type:** UI  
**Owned by:** Verification

```
Given the app is running and Spotify is actively playing a track
When I click the play/pause button
Then Spotify pauses and the button changes to the play symbol
When I click play/pause again
Then Spotify resumes and the button shows the pause symbol
When I click the next track button
Then Spotify skips to the next track and the track name in the window updates
When I click the seek-forward button
Then the progress bar advances approximately 10 seconds
```

---

### T-08
**Feature:** UI module split — regression  
**Scenario:** Like and Unlike save and remove feedback  
**Covers AC:** AC-8  
**Level:** E2E  
**Type:** UI  
**Owned by:** Verification

```
Given the app is running and a track is playing
When I click the Like button
Then the button label changes to "Unlike" and the text turns green
When I click Unlike
Then the button returns to "Like" with no green colouring

Given the app is running and a different track is playing
When I click the Dislike button
Then the button turns red, playback skips to the next track, and the new track begins playing
```

---

### T-09
**Feature:** UI module split — regression  
**Scenario:** Settings modal opens, accepts slider input, and saves to config  
**Covers AC:** AC-9  
**Level:** E2E  
**Type:** UI  
**Owned by:** Verification

```
Given the app is running
When I click the settings gear button
Then a Settings window appears containing sliders for queue size, score exponents, and other parameters
And the Last.fm section shows Sync, Score, Expand, and Fetch Tracks buttons
When I drag the Queue size slider to a new value
Then no error appears and the settings window stays open
When I close the app and inspect config.toml
Then the queue size field reflects the value I set
When I reopen the app and open Settings again
Then the slider shows the value I previously set
```

---

### T-10
**Feature:** UI module split — regression  
**Scenario:** Error modal appears when Spotify is unreachable and can be dismissed  
**Covers AC:** AC-10  
**Level:** E2E  
**Type:** UI  
**Owned by:** Verification

```
Given the app is running and the Spotify token has been invalidated
When the polling loop attempts to contact Spotify
Then a centred modal window titled "⚠  Error" appears containing an error description
When I click the OK button
Then the modal closes and the main player window is visible and interactive
```

*Notes:* Token can be invalidated by editing `~/.gurdo/spotify_token.json` to contain an expired access_token.

---

### T-11
**Feature:** UI module split — regression  
**Scenario:** Queue button starts a recommendation queue  
**Covers AC:** AC-11  
**Level:** E2E  
**Type:** UI  
**Owned by:** Verification

```
Given the app is running
And the database contains track recommendations (fetch-tracks has been run)
And a Spotify device is active
When I click the queue button (☰)
Then Spotify begins playing a recommended track within approximately 5 seconds
And subsequent tracks played are also from the recommended set
```

---

### T-12
**Feature:** UI module split — regression  
**Scenario:** CJK track and artist names display correctly  
**Covers AC:** AC-12  
**Level:** E2E  
**Type:** UI  
**Owned by:** Verification

```
Given the app is running
And a track with a Japanese, Chinese, or Korean title is playing on Spotify
When the player window shows the current track information
Then the track name and artist name are rendered in readable CJK characters
And no placeholder squares or question marks appear in place of characters
```

*Notes:* The pre-refactor font loading code (OS-font probe loop in `run()`) is moved verbatim — this scenario confirms it was not accidentally dropped.

---

## AC coverage table

| AC | Scenario(s) | Owned by |
|---|---|---|
| AC-1 | T-02, T-05 | Development |
| AC-2 | T-01 | Development |
| AC-3 | T-01 (transitively) | Development |
| AC-4 | T-03 | Development |
| AC-5 | T-04 | Development |
| AC-6 | T-06 | Verification |
| AC-7 | T-07 | Verification |
| AC-8 | T-08 | Verification |
| AC-9 | T-09 | Verification |
| AC-10 | T-10 | Verification |
| AC-11 | T-11 | Verification |
| AC-12 | T-12 | Verification |
