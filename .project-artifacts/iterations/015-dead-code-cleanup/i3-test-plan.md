# Iteration 15 Test Plan — Dead-code cleanup (EP-16)

Verification is structural. All 16 existing tests act as regression guard.

## Structural checks

| Check | Command | Expected |
|-------|---------|----------|
| No dead query functions | `grep -E "upsert_top_artist\|upsert_top_track\|upsert_tag_top_track\|is_tag_synced\|get_top_tracks_for_period\|get_top_tags\|get_top_artists_by_period\|get_tag_top_tracks\|get_spotify_uri\|get_recent_period_artists\|get_loved_track_artists\|get_artist_top_tracks\|get_all_top_artist_names\|get_all_known_tracks\|clear_artists" src/db/queries.rs` | no output |
| No dead lastfm model structs | `grep -E "TopArtist\|TopTrack\|TagTopTrack\|^pub struct Image" src/lastfm/models.rs` | no output |
| No dead lastfm client methods | `grep -E "user_top_artists\|user_top_tracks\|tag_top_tracks" src/lastfm/client.rs` | no output |
| No dead spotify structs | `grep -E "SavedTrack\|Playlist" src/spotify/models.rs` | no output |
| No dead spotify methods | `grep -E "bearer\|get_liked_songs\|get_playlists\|get_playlist_tracks\|save_track\|remove_saved_track" src/spotify/client.rs` | no output |
| Warning count | `cargo build 2>&1 \| grep "generated.*warnings"` | 1 warning |

## Regression scenarios

All 16 existing tests must pass unchanged.
