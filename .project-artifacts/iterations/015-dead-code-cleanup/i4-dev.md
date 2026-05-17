# Iteration 15 Development — Dead-code cleanup (EP-16)

## Files changed

| File | Change |
|------|--------|
| `src/db/queries.rs` | Removed 15 dead functions (upsert_top_artist, upsert_top_track, upsert_tag_top_track, is_tag_synced, get_top_tracks_for_period, get_top_tags, get_top_artists_by_period, get_tag_top_tracks, get_spotify_uri, get_recent_period_artists, get_loved_track_artists, get_artist_top_tracks, get_all_top_artist_names, get_all_known_tracks, clear_artists) |
| `src/lastfm/models.rs` | Removed: `Image` struct; `TopArtistsResponse/TopArtists/TopArtist` + impl; `TopTracksResponse/TopTracks/TopTrack` + impl; `TagTopTracksResponse/TagTopTracks/TagTopTrack` + impl; `ArtistRef.mbid`, `ArtistRef.url`, `SimilarArtist.mbid`, `WeeklyChartEntry.to`, `PageAttr.page`, `PageAttr.total`; methods `to_ts`, `total_u32` |
| `src/lastfm/client.rs` | Removed `user_top_artists`, `user_top_tracks`, `tag_top_tracks` methods; removed `warn` import |
| `src/spotify/models.rs` | Removed: `SavedTracksResponse`, `SavedTrackItem`, `SavedTrack`, `PlaylistsResponse`, `PlaylistSummary`, `PlaylistItemsResponse`, `PlaylistItemEntry`, `PlaylistTrack`; fields `AlbumImage.height`, `Device.device_type`, `Device.is_restricted`, `Device.volume_percent`, `Device.name` |
| `src/spotify/client.rs` | Removed `bearer`, `get_liked_songs`, `get_playlists`, `get_playlist_tracks`, `save_track`, `remove_saved_track`; cleaned up use imports |

No test code changes. No production behaviour changes.

## Key decisions

- `LovedTrack.mbid` and `WeeklyArtistEntry.mbid` were initially removed incorrectly — both are read in sync code (`sync/mod.rs:40`, `sync/artists.rs:33`). Restored after compile error caught the mistake. The dead-code warnings were for `ArtistRef.mbid` and `SimilarArtist.mbid` (never read anywhere).
- `Device.name` removed after confirming it is never accessed in UI code.

## Warning budget

`cargo build`: 1 warning (the pre-existing `last_track_uri` value-assignment in `src/ui/poll.rs` — deferred). Down from 47 → 1.

## Self-review

- AC-1 through AC-6 all verified via grep and `cargo build` output.
- 16 tests pass unchanged.
