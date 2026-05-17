use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::config::Config;
use crate::db;
use crate::engine::recommend;
use crate::spotify;

use super::state::{PlayerCommand, PlayerState};

const QUEUE_CHUNK_SIZE: usize = 10;

fn set_background_error(state: &Arc<Mutex<PlayerState>>, msg: String) {
    let mut s = state.lock().unwrap();
    let snoozed = s.api_error_snooze_until
        .map(|t| t > std::time::Instant::now())
        .unwrap_or(false);
    if !snoozed {
        s.error = Some(msg);
    }
}

async fn do_poll(state: &Arc<Mutex<PlayerState>>, config: &Config, http: &reqwest::Client) {
    let result: anyhow::Result<()> = async {
        let Some(token) = spotify::auth::load_or_refresh_token(config).await? else {
            return Ok(());
        };
        let client = spotify::SpotifyClient::new(token);

        match client.get_currently_playing().await? {
            None => {
                let mut s = state.lock().unwrap();
                s.is_playing             = false;
                s.track_name             = String::new();
                s.artist_name            = String::new();
                s.album_art_url          = None;
                s.album_art_bytes        = None;
                s.track_id               = None;
                s.track_uri              = None;
                s.progress_ms            = 0;
                s.duration_ms            = 0;
                s.feedback               = None;
                s.error                  = None;
                s.api_error_snooze_until = None;
            }
            Some(playing) => {
                if let Some(track) = playing.item {
                    let new_art_url = track.best_image_url().map(|u| u.to_string());

                    let current_url = state.lock().unwrap().album_art_url.clone();
                    let art_bytes = if new_art_url != current_url {
                        if let Some(url) = &new_art_url {
                            http.get(url).send().await.ok()
                                .and_then(|r| tokio::task::block_in_place(|| {
                                    tokio::runtime::Handle::current()
                                        .block_on(r.bytes())
                                        .ok()
                                }))
                                .map(|b| b.to_vec())
                        } else {
                            None
                        }
                    } else {
                        state.lock().unwrap().album_art_bytes.clone()
                    };

                    let artist_name = track.artists.iter()
                        .map(|a| a.name.as_str()).collect::<Vec<_>>().join(", ");
                    let track_name = track.name.clone();

                    let feedback = if let Ok(conn) = db::open(&config.db_path()) {
                        crate::db::queries::get_track_feedback(&conn, &artist_name, &track_name).ok().flatten()
                    } else {
                        None
                    };

                    let mut s = state.lock().unwrap();
                    s.is_playing      = playing.is_playing;
                    s.track_name      = track_name;
                    s.artist_name     = artist_name;
                    s.album_name      = track.album.name.clone();
                    s.album_art_url   = new_art_url;
                    s.album_art_bytes = art_bytes;
                    s.track_id        = track.id;
                    s.track_uri       = Some(track.uri.clone());
                    s.progress_ms            = playing.progress_ms.unwrap_or(0);
                    s.duration_ms            = track.duration_ms.unwrap_or(0);
                    s.feedback               = feedback;
                    s.error                  = None;
                    s.api_error_snooze_until = None;
                }
            }
        }
        Ok(())
    }.await;

    if let Err(e) = result {
        set_background_error(&state, e.to_string());
    }
}

async fn handle_cmd(
    cmd: PlayerCommand,
    state: &Arc<Mutex<PlayerState>>,
    config: &Config,
    client: &spotify::SpotifyClient,
    our_uris: &mut HashSet<String>,
) -> Option<u64> {
    let result: anyhow::Result<Option<u64>> = async {
        match cmd {
            PlayerCommand::PlayPause => {
                let is_playing = state.lock().unwrap().is_playing;
                let device = client.active_device().await?;
                let id = device.id.unwrap_or_default();
                if is_playing {
                    client.pause(&id).await?;
                    state.lock().unwrap().is_playing = false;
                } else {
                    client.resume(&id).await?;
                    state.lock().unwrap().is_playing = true;
                }
                Ok(None)
            }
            PlayerCommand::Next => {
                let device = client.active_device().await?;
                client.next(&device.id.unwrap_or_default()).await?;
                Ok(Some(300))
            }
            PlayerCommand::Previous => {
                let device = client.active_device().await?;
                client.previous(&device.id.unwrap_or_default()).await?;
                Ok(Some(300))
            }
            PlayerCommand::SeekRelative(offset_ms) => {
                let (progress, duration) = {
                    let s = state.lock().unwrap();
                    (s.progress_ms, s.duration_ms)
                };
                let new_pos = (progress as i64 + offset_ms)
                    .max(0)
                    .min(duration as i64) as u64;
                let device = client.active_device().await?;
                client.seek(&device.id.unwrap_or_default(), new_pos).await?;
                state.lock().unwrap().progress_ms = new_pos;
                Ok(None)
            }
            PlayerCommand::SaveTrack => {
                let (artist, track) = {
                    let s = state.lock().unwrap();
                    (s.artist_name.clone(), s.track_name.clone())
                };
                let conn = db::open(&config.db_path())?;
                crate::db::queries::record_feedback(&conn, &artist, &track, true)?;
                crate::db::queries::recalculate_artist_score(
                    &conn, &artist,
                    config.engine.like_bonus_flat, config.engine.dislike_modifier_pct,
                    config.engine.multi_source_bonus_pct,
                )?;
                Ok(None)
            }
            PlayerCommand::UnlikeTrack => {
                let (artist, track) = {
                    let s = state.lock().unwrap();
                    (s.artist_name.clone(), s.track_name.clone())
                };
                let conn = db::open(&config.db_path())?;
                crate::db::queries::remove_feedback(&conn, &artist, &track)?;
                crate::db::queries::recalculate_artist_score(
                    &conn, &artist,
                    config.engine.like_bonus_flat, config.engine.dislike_modifier_pct,
                    config.engine.multi_source_bonus_pct,
                )?;
                Ok(None)
            }
            PlayerCommand::RemoveTrack => {
                let (artist, track) = {
                    let s = state.lock().unwrap();
                    (s.artist_name.clone(), s.track_name.clone())
                };
                let conn = db::open(&config.db_path())?;
                crate::db::queries::record_feedback(&conn, &artist, &track, false)?;
                crate::db::queries::recalculate_artist_score(
                    &conn, &artist,
                    config.engine.like_bonus_flat, config.engine.dislike_modifier_pct,
                    config.engine.multi_source_bonus_pct,
                )?;
                Ok(None)
            }
            PlayerCommand::StartQueue => {
                let conn = db::open(&config.db_path())?;
                let recs = recommend::generate_recommendations(&conn, config)?;
                if recs.is_empty() {
                    anyhow::bail!("No recommendations — run fetch-tracks first");
                }
                let now = chrono::Utc::now().timestamp();
                let mut uris: Vec<String> = Vec::new();
                for (artist, track, _score) in recs.iter().take(QUEUE_CHUNK_SIZE) {
                    if let Ok(Some(item)) = client.search_track(artist, track).await {
                        let art = item.best_image_url().map(|s| s.to_string());
                        crate::db::queries::upsert_spotify_uri(
                            &conn, artist, track, Some(&item.uri), art.as_deref(), now,
                        )?;
                        uris.push(item.uri);
                    }
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
                if !uris.is_empty() {
                    let device = client.active_device().await?;
                    let device_id = device.id.unwrap_or_default();
                    client.play(&device_id, &uris[..1]).await?;
                    for uri in &uris[1..] {
                        client.add_to_queue(&device_id, uri).await?;
                    }
                    our_uris.clear();
                    for uri in &uris[1..] {
                        our_uris.insert(uri.clone());
                    }
                }
                Ok(Some(500))
            }
        }
    }.await;

    match result {
        Ok(v) => v,
        Err(e) => {
            state.lock().unwrap().error = Some(e.to_string());
            None
        }
    }
}

async fn extend_queue_if_needed(
    state: &Arc<Mutex<PlayerState>>,
    config: &Config,
    our_uris: &mut HashSet<String>,
) {
    let is_playing = state.lock().unwrap().is_playing;
    if !is_playing { return; }

    let result: anyhow::Result<()> = async {
        let Some(token) = spotify::auth::load_or_refresh_token(config).await? else {
            return Ok(());
        };
        let client = spotify::SpotifyClient::new(token);

        if let Some(uri) = state.lock().unwrap().track_uri.clone() {
            our_uris.remove(&uri);
        }

        let spotify_queue = client.get_queue().await?;
        let remaining: usize = spotify_queue.iter()
            .filter(|uri| our_uris.contains(uri.as_str()))
            .count();

        tracing::debug!("extend_queue_if_needed: {}/{} tracks in queue are ours",
            remaining, spotify_queue.len());

        if remaining > 3 {
            return Ok(());
        }

        tracing::info!("extend_queue_if_needed: {} of our tracks remaining, extending...", remaining);

        let device = client.active_device().await?;
        let device_id = device.id.unwrap_or_default();

        let conn = db::open(&config.db_path())?;
        let recs = recommend::generate_recommendations(&conn, config)?;
        let now = chrono::Utc::now().timestamp();

        let mut added = 0;
        for (artist, track, _score) in recs.iter().take(QUEUE_CHUNK_SIZE) {
            if let Ok(Some(item)) = client.search_track(artist, track).await {
                let art = item.best_image_url().map(|s| s.to_string());
                crate::db::queries::upsert_spotify_uri(
                    &conn, artist, track, Some(&item.uri), art.as_deref(), now,
                )?;
                client.add_to_queue(&device_id, &item.uri).await?;
                our_uris.insert(item.uri);
                added += 1;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        tracing::info!("Extended queue with {} tracks", added);
        Ok(())
    }.await;

    if let Err(e) = result {
        set_background_error(&state, format!("Queue extend error: {}", e));
    }
}

pub(super) async fn polling_loop(
    state:         Arc<Mutex<PlayerState>>,
    mut cmd_rx:    tokio::sync::mpsc::UnboundedReceiver<PlayerCommand>,
    shared_config: Arc<Mutex<Config>>,
) {
    let http = reqwest::Client::new();
    let mut our_uris: HashSet<String> = HashSet::new();
    let mut last_track_uri: Option<String> = None;

    let config = shared_config.lock().unwrap().clone();
    do_poll(&state, &config, &http).await;
    last_track_uri = state.lock().unwrap().track_uri.clone();

    async fn check_track_change(
        state: &Arc<Mutex<PlayerState>>,
        config: &Config,
        our_uris: &mut HashSet<String>,
        last_track_uri: &mut Option<String>,
    ) {
        let current_uri = state.lock().unwrap().track_uri.clone();
        if current_uri != *last_track_uri {
            *last_track_uri = current_uri;
            extend_queue_if_needed(state, config, our_uris).await;
        }
    }

    let mut progress_tick = tokio::time::interval(Duration::from_secs(1));
    progress_tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    progress_tick.tick().await;

    let mut heartbeat_tick = tokio::time::interval(Duration::from_secs(5));
    heartbeat_tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    heartbeat_tick.tick().await;

    loop {
        tokio::select! {
            _ = heartbeat_tick.tick() => {
                let config = shared_config.lock().unwrap().clone();
                do_poll(&state, &config, &http).await;
                check_track_change(&state, &config, &mut our_uris, &mut last_track_uri).await;
            }

            _ = progress_tick.tick() => {
                let track_ended = {
                    let mut s = state.lock().unwrap();
                    if s.is_playing && s.duration_ms > 0 {
                        if s.progress_ms + 1000 >= s.duration_ms {
                            s.progress_ms = s.duration_ms;
                            s.is_playing = false;
                            true
                        } else {
                            s.progress_ms += 1000;
                            false
                        }
                    } else {
                        false
                    }
                };
                if track_ended {
                    let config = shared_config.lock().unwrap().clone();
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    do_poll(&state, &config, &http).await;
                    check_track_change(&state, &config, &mut our_uris, &mut last_track_uri).await;
                }
            }

            cmd = cmd_rx.recv() => {
                let Some(cmd) = cmd else { break };
                let config = shared_config.lock().unwrap().clone();
                match spotify::auth::load_or_refresh_token(&config).await {
                    Err(e) => state.lock().unwrap().error = Some(format!("Token error: {}", e)),
                    Ok(None) => state.lock().unwrap().error = Some(
                        "Spotify not connected — complete setup to enable playback".to_string()
                    ),
                    Ok(Some(token)) => {
                        let client = spotify::SpotifyClient::new(token);
                        if let Some(delay_ms) = handle_cmd(cmd, &state, &config, &client, &mut our_uris).await {
                            tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                            do_poll(&state, &config, &http).await;
                            check_track_change(&state, &config, &mut our_uris, &mut last_track_uri).await;
                        }
                    }
                }
            }
        }
    }
}
