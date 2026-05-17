# Iteration 15 Decomposition — Dead-code cleanup (EP-16)

## Tasks

### DEV-1 — `src/db/queries.rs`: remove 15 dead functions
- Remove: `upsert_top_artist`, `upsert_top_track`, `upsert_tag_top_track`, `is_tag_synced`, `get_top_tracks_for_period`, `get_top_tags`, `get_top_artists_by_period`, `get_tag_top_tracks`, `get_spotify_uri`, `get_recent_period_artists`, `get_loved_track_artists`, `get_artist_top_tracks`, `get_all_top_artist_names`, `get_all_known_tracks`, `clear_artists`.
- **AC:** AC-1

### DEV-2 — `src/lastfm/models.rs`: remove dead structs, fields, methods
- Remove structs + impls: `Image`, `TopArtistsResponse`/`TopArtists`/`TopArtist`, `TopTracksResponse`/`TopTracks`/`TopTrack`, `TagTopTracksResponse`/`TagTopTracks`/`TagTopTrack`.
- Remove fields: `ArtistRef.mbid`, `ArtistRef.url`, `LovedTrack.mbid`, `WeeklyChartEntry.to`, `WeeklyArtistEntry.mbid`, `PageAttr.page`, `PageAttr.total`.
- Remove methods: `to_ts` (on WeeklyChartEntry), `total_u32` (on PageAttr).
- **AC:** AC-2

### DEV-3 — `src/lastfm/client.rs`: remove 3 dead methods
- Remove `user_top_artists`, `user_top_tracks`, `tag_top_tracks`.
- **AC:** AC-3

### DEV-4 — `src/spotify/models.rs`: remove dead structs and fields
- Remove structs: `SavedTracksResponse`, `SavedTrackItem`, `SavedTrack`, `PlaylistsResponse`, `PlaylistSummary`, `PlaylistItemsResponse`, `PlaylistItemEntry`, `PlaylistTrack`.
- Remove fields: `AlbumImage.height`, `Device.device_type`, `Device.is_restricted`, `Device.volume_percent`.
- **AC:** AC-4

### DEV-5 — `src/spotify/client.rs`: remove 6 dead methods
- Remove: `bearer`, `get_liked_songs`, `get_playlists`, `get_playlist_tracks`, `save_track`, `remove_saved_track`.
- **AC:** AC-5

### Cross-cutting — Warning budget
- `cargo build` = 1 warning; `cargo test` green.
- **AC:** AC-6
