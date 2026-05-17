use std::sync::{Arc, Mutex};

use tokio::sync::mpsc::UnboundedReceiver;

use crate::config::Config;
use crate::db;
use crate::engine::artist_scores;
use crate::lastfm::LastfmClient;
use crate::progress::ProgressReporter;
use crate::spotify;
use crate::sync;

use super::state::{ActiveOperation, OperationCommand, OperationKind, OperationResult, OperationsState};

// ── StateReporter ─────────────────────────────────────────────────────────────

pub struct StateReporter {
    pub ops: Arc<Mutex<OperationsState>>,
}

impl ProgressReporter for StateReporter {
    fn stage(&self, name: &str) {
        if let Some(a) = &mut self.ops.lock().unwrap().active {
            a.stage = name.to_string();
            a.current = 0;
            a.total = None;
        }
    }
    fn tick(&self, current: u64, total: Option<u64>) {
        if let Some(a) = &mut self.ops.lock().unwrap().active {
            a.current = current;
            a.total = total;
        }
    }
    fn message(&self, msg: &str) {
        if let Some(a) = &mut self.ops.lock().unwrap().active {
            a.message = msg.to_string();
        }
    }
    fn finish(&self, _ok: bool, _summary: &str) {}
}

// ── Helpers ───────────────────────────────────────────────────────────────────

pub fn token_exists(config: &Config) -> bool {
    config.token_path().exists()
}

// ── Operation dispatch ────────────────────────────────────────────────────────

async fn run_operation(
    kind: OperationKind,
    config: &Config,
    progress: &dyn ProgressReporter,
) -> anyhow::Result<String> {
    match kind {
        OperationKind::SyncLastfm => {
            let conn = db::open(&config.db_path())?;
            let client = LastfmClient::new(config.lastfm.api_key.clone());
            sync::sync_lastfm(&conn, &client, config, progress).await?;
            Ok("Last.fm sync complete".to_string())
        }
        OperationKind::Expand => {
            let conn = db::open(&config.db_path())?;
            let client = LastfmClient::new(config.lastfm.api_key.clone());
            sync::expand_artists(&conn, &client, config, progress).await?;
            Ok("Similar artists expanded".to_string())
        }
        OperationKind::FetchTracks => {
            let conn = db::open(&config.db_path())?;
            let client = LastfmClient::new(config.lastfm.api_key.clone());
            sync::fetch_artist_tracks(&conn, &client, None, config, progress).await?;
            Ok("Top tracks fetched".to_string())
        }
        OperationKind::Score => {
            let conn = db::open(&config.db_path())?;
            artist_scores::score_artists(&conn, config, progress)?;
            Ok("Scores recalculated".to_string())
        }
        OperationKind::SpotifyLogin => {
            progress.stage("Waiting for browser authorization");
            spotify::auth::run_oauth_flow(config).await?;
            Ok("Authenticated".to_string())
        }
    }
}

// ── Dispatcher loop ───────────────────────────────────────────────────────────

pub async fn ops_dispatcher_loop(
    mut cmd_rx: UnboundedReceiver<OperationCommand>,
    ops: Arc<Mutex<OperationsState>>,
    shared_config: Arc<Mutex<Config>>,
    _settings_draft: Arc<Mutex<Option<Config>>>,
) {
    while let Some(cmd) = cmd_rx.recv().await {
        match cmd {
            OperationCommand::Run(kind) => {
                {
                    let mut o = ops.lock().unwrap();
                    o.active = Some(ActiveOperation {
                        kind:    kind.clone(),
                        step:    None,
                        stage:   String::new(),
                        current: 0,
                        total:   None,
                        message: String::new(),
                    });
                }
                let reporter = StateReporter { ops: Arc::clone(&ops) };
                let config = shared_config.lock().unwrap().clone();
                let result = run_operation(kind, &config, &reporter).await;
                let mut o = ops.lock().unwrap();
                o.active = None;
                o.last_result = Some(match result {
                    Ok(summary) => OperationResult::Ok(summary),
                    Err(e)      => OperationResult::Failed(e.to_string()),
                });
            }
            OperationCommand::UpdateAll => {
                let steps = [
                    OperationKind::SyncLastfm,
                    OperationKind::Expand,
                    OperationKind::FetchTracks,
                    OperationKind::Score,
                ];
                let total = steps.len() as u8;
                for (i, kind) in steps.iter().enumerate() {
                    {
                        let mut o = ops.lock().unwrap();
                        o.active = Some(ActiveOperation {
                            kind:    kind.clone(),
                            step:    Some((i as u8 + 1, total)),
                            stage:   String::new(),
                            current: 0,
                            total:   None,
                            message: String::new(),
                        });
                    }
                    let reporter = StateReporter { ops: Arc::clone(&ops) };
                    let config   = shared_config.lock().unwrap().clone();
                    if let Err(e) = run_operation(kind.clone(), &config, &reporter).await {
                        let mut o = ops.lock().unwrap();
                        o.active      = None;
                        o.last_result = Some(OperationResult::Failed(
                            format!("Step {}/{} ({}) failed: {}", i + 1, total, kind.label(), e)
                        ));
                        return;
                    }
                }
                let mut o = ops.lock().unwrap();
                o.active      = None;
                o.last_result = Some(OperationResult::Ok("Update complete (4 steps)".to_string()));
            }
        }
    }
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ops(kind: OperationKind) -> Arc<Mutex<OperationsState>> {
        Arc::new(Mutex::new(OperationsState {
            active: Some(ActiveOperation {
                kind,
                step:    None,
                stage:   String::new(),
                current: 0,
                total:   None,
                message: String::new(),
            }),
            last_result: None,
        }))
    }

    #[test]
    fn stage_resets_current_and_total() {
        let ops = make_ops(OperationKind::Score);
        {
            let mut o = ops.lock().unwrap();
            if let Some(a) = &mut o.active {
                a.current = 5;
                a.total = Some(10);
            }
        }
        let reporter = StateReporter { ops: Arc::clone(&ops) };
        reporter.stage("New stage");
        let o = ops.lock().unwrap();
        let a = o.active.as_ref().unwrap();
        assert_eq!(a.stage, "New stage");
        assert_eq!(a.current, 0);
        assert_eq!(a.total, None);
    }

    #[test]
    fn tick_updates_progress() {
        let ops = make_ops(OperationKind::Score);
        let reporter = StateReporter { ops: Arc::clone(&ops) };
        reporter.tick(42, Some(100));
        let o = ops.lock().unwrap();
        let a = o.active.as_ref().unwrap();
        assert_eq!(a.current, 42);
        assert_eq!(a.total, Some(100));
    }

    #[test]
    fn reporter_is_noop_when_active_is_none() {
        let ops = Arc::new(Mutex::new(OperationsState { active: None, last_result: None }));
        let reporter = StateReporter { ops: Arc::clone(&ops) };
        reporter.stage("x");
        reporter.tick(1, Some(2));
        reporter.message("y");
        assert!(ops.lock().unwrap().active.is_none());
    }
}
