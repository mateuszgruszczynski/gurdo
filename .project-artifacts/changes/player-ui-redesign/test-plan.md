# Test Plan — Player UI polish: consistent ghost-style controls

## In-process (Unit / Component)

### S-1 — Ghost visuals constants [Unit]
**Given** the ghost-button visuals helper values are defined  
**When** idle fill colour is inspected  
**Then** it is fully transparent (alpha = 0)  
**And** hover fill is `rgba(255,255,255,20)`  
**And** press fill is `rgba(255,255,255,40)`  
Covers: AC-1, AC-2

### S-2 — Progress bar colours [Unit]
**Given** the progress bar is rendered  
**When** `extreme_bg_color` is read  
**Then** it equals `rgba(255,255,255,25)`  
**And** the bar fill equals `rgba(255,255,255,160)`  
Covers: AC-3

### S-3 — Build clean [Component]
**Given** all changes are in place  
**When** `cargo build` runs  
**Then** exit code is 0 and no new warnings appear (baseline: 2 pre-existing warnings)  
Covers: AC-9

### S-4 — Existing tests pass [Component]
**Given** all changes are in place  
**When** `cargo test` runs  
**Then** all 16 tests pass  
Covers: AC-10

## E2E / UI (manual — out-of-process)

### S-5 — Transport buttons consistent [E2E/UI]
**Given** the app is running with a track playing  
**When** the transport row is observed  
**Then** all five buttons (⏮ ⏪ ▶ ⏩ ⏭) have identical background treatment  
**And** none shows a distinctly darker or lighter fill than the others  
Covers: AC-1, AC-2

### S-6 — Feedback row separate [E2E/UI]
**Given** the app is running  
**When** the player window layout is observed  
**Then** Like and Dislike buttons appear on a row of their own below transport  
**And** Queue and Settings appear on a separate row below Like/Dislike  
**And** text labels "♥ Like" / "👎 Dislike" are visible  
Covers: AC-4, AC-5, AC-6

### S-7 — Liked state colour [E2E/UI]
**Given** the current track is marked as liked  
**When** the Like button is rendered  
**Then** the label text is Spotify green, not white  
**And** the button fill remains ghost (no dark fill)  
Covers: AC-8

### S-8 — Rounding uniformity [E2E/UI]
**Given** the app is running  
**When** all button corners are compared visually  
**Then** transport, feedback, and utility buttons all show the same corner radius  
Covers: AC-7

### S-9 — Progress bar on light cover [E2E/UI]
**Given** a track with a bright/light cover art is playing  
**When** the progress bar is observed  
**Then** the track and fill are visible (white-on-light reads better than dark-on-light)  
Covers: AC-3

## Regression

### S-10 — Transport commands still fire [E2E/UI]
**Given** the transport buttons are restyled  
**When** Previous / SeekBack / PlayPause / SeekForward / Next are clicked  
**Then** Spotify responds with the expected action  
**And** no button click is silently dropped

### S-11 — Settings viewport still opens [E2E/UI]
**Given** the ⚙ button moved to the utility row  
**When** ⚙ is clicked  
**Then** the settings viewport opens at the expected position
