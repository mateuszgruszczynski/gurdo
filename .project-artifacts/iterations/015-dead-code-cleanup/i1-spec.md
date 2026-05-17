# Iteration 15 Spec — Dead-code cleanup (EP-16)

## Problem
CLI subcommands removed in EP-2 left ~47 dead `pub fn`, `pub struct`, and struct fields across `src/db/queries.rs`, `src/lastfm/`, and `src/spotify/`. These generate 46 dead-code compiler warnings (47 total; one benign `last_track_uri` assignment in poll.rs is excluded). EP-13 already removed the `similar_tracks` family; this epic removes the rest.

## Goal
Delete all orphaned API surface so `cargo build` produces ≤1 warning (the pre-existing `last_track_uri` value-assignment in poll.rs, which requires a logic change and is deferred).

## Acceptance criteria

| ID | Criterion |
|----|-----------|
| AC-1 | `src/db/queries.rs` no longer contains: `upsert_top_artist`, `upsert_top_track`, `upsert_tag_top_track`, `is_tag_synced`, `get_top_tracks_for_period`, `get_top_tags`, `get_top_artists_by_period`, `get_tag_top_tracks`, `get_spotify_uri`, `get_recent_period_artists`, `get_loved_track_artists`, `get_artist_top_tracks`, `get_all_top_artist_names`, `get_all_known_tracks`, `clear_artists` (15 functions). |
| AC-2 | `src/lastfm/models.rs` no longer contains: `Image`, `TopArtistsResponse`/`TopArtists`/`TopArtist`, `TopTracksResponse`/`TopTracks`/`TopTrack`, `TagTopTracksResponse`/`TagTopTracks`/`TagTopTrack` structs; dead fields (`ArtistRef.mbid`, `ArtistRef.url`, `LovedTrack.mbid`, `WeeklyChartEntry.to`, `WeeklyArtistEntry.mbid`, `PageAttr.page`, `PageAttr.total`); dead methods (`to_ts`, `total_u32`) are also removed. |
| AC-3 | `src/lastfm/client.rs` no longer contains `user_top_artists`, `user_top_tracks`, or `tag_top_tracks` methods. |
| AC-4 | `src/spotify/models.rs` no longer contains: `SavedTracksResponse`, `SavedTrackItem`, `SavedTrack`, `PlaylistsResponse`, `PlaylistSummary`, `PlaylistItemsResponse`, `PlaylistItemEntry`, `PlaylistTrack` structs; `AlbumImage.height`, `Device.device_type`, `Device.is_restricted`, `Device.volume_percent` fields are removed. |
| AC-5 | `src/spotify/client.rs` no longer contains `bearer`, `get_liked_songs`, `get_playlists`, `get_playlist_tracks`, `save_track`, or `remove_saved_track`. |
| AC-6 | `cargo build` produces exactly 1 warning (the `last_track_uri` value-assignment in `src/ui/poll.rs`). `cargo test` green with 16 tests passing. |

## Out of scope
- The `last_track_uri` unused-assignment warning in `src/ui/poll.rs` — requires logic change, deferred.
- Any changes to live sync/engine/UI code paths.
- `similar_artists` table and its query helpers (still live).

## Key decisions
- Removing fields from serde Deserialize structs is safe — serde silently ignores extra JSON fields.
- No `#[allow(dead_code)]` attributes; actual deletion is the goal.
- `Device.name` and `Device.is_active`, `Device.id` are retained — they are read in live UI code.
