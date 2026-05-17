# Iteration 8 Test Plan — Full config-knob exposure (EP-8)

## BDD scenarios

### S-01 — Draft is None when settings opens, Some after first edit (Unit / AC-2, AC-5)
Given `settings_draft = Arc::new(Mutex::new(None))`  
When a knob change triggers draft initialisation  
Then `settings_draft.lock().unwrap().is_some()` == true

### S-02 — Discard sets draft back to None (Unit / AC-3)
Given `settings_draft` is `Some(config_with_change)`  
When discard is triggered  
Then `settings_draft.lock().unwrap().is_none()` == true

### S-03 — Save writes to shared_config (Unit / AC-2)
Given a draft with a changed `recommendations.count`  
When save is triggered  
Then `shared_config.lock().unwrap().recommendations.count` reflects the draft value

### S-04 — fetch_artist_tracks reads artist_top_tracks_limit from config (Unit / AC-7)
Given `config.engine.artist_top_tracks_limit = 25`  
When `TRACKS_PER_ARTIST` is removed and the constant is replaced  
Then `cargo build` succeeds and no reference to `TRACKS_PER_ARTIST` exists

### S-05 — Zero new warnings (cross-cutting / AC-8)
`cargo build` warning count == 53

## Level assignments

| Scenario | Level | Runs in |
|---|---|---|
| S-01 – S-03 | Unit | Development (in-process) |
| S-04 | Unit / compile check | Development |
| S-05 | System-integration | Integration (cargo build) |
| AC-1,4,5,6 | System-integration | Integration (manual UI smoke) |

## Regression scenarios

R-01 — Existing ops buttons (Sync/Expand/Fetch/Score/Login) still work after settings render
signature change. Verified by compile + manual smoke.
