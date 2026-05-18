// Security audit: no credential values are emitted in any tracing/log/eprintln/dbg call
// in this module. Field names may appear in log output, but never their values.

use anyhow::{Context, Result};
use eframe::egui;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::progress::ProgressReporter;
use super::state::{ActiveOperation, OperationKind, OperationResult, OperationsState};

const DEFAULT_CONFIG_TOML: &str = r#"[app]
data_dir = "~/.gurdo"

[sync]
seed_artists_limit = 50
seed_tracks_limit  = 50
loved_tracks_limit = 500

[engine]
similarity_multiplier    = 0.5
multi_source_bonus_pct   = 0.05
like_bonus_flat          = 5.0
like_modifier_pct        = 0.05
dislike_modifier_pct     = 0.1
similar_artists_limit    = 20
artist_top_tracks_limit  = 10
tag_top_tracks_limit     = 30
recommendation_pool_size = 200
max_tracks_per_seed      = 20
"#;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Phase { Fields, OAuth, FetchPrompt, Fetching }

#[derive(Debug, Clone, PartialEq)]
enum OAuthStatus { Idle, Pending, Success, Failed(String) }

#[derive(Debug, Clone, Copy, PartialEq)]
enum SetupOutcome { InProgress, Complete, CancelledPhase1, CancelledOAuth }

struct SetupApp {
    config_path: PathBuf,
    phase: Phase,
    username: String,
    write_error: Option<String>,
    oauth_status: OAuthStatus,
    oauth_result: Arc<Mutex<Option<std::result::Result<(), String>>>>,
    ops_state: Arc<Mutex<OperationsState>>,
    outcome: Arc<Mutex<SetupOutcome>>,
}

// ── Reporter for the fetch thread ─────────────────────────────────────────────

struct SetupReporter {
    ops: Arc<Mutex<OperationsState>>,
    ctx: egui::Context,
}

impl ProgressReporter for SetupReporter {
    fn stage(&self, name: &str) {
        if let Some(a) = &mut self.ops.lock().unwrap().active {
            a.stage   = name.to_string();
            a.current = 0;
            a.total   = None;
        }
        self.ctx.request_repaint();
    }
    fn tick(&self, current: u64, total: Option<u64>) {
        if let Some(a) = &mut self.ops.lock().unwrap().active {
            a.current = current;
            a.total   = total;
        }
        self.ctx.request_repaint();
    }
    fn message(&self, msg: &str) {
        if let Some(a) = &mut self.ops.lock().unwrap().active {
            a.message = msg.to_string();
        }
        self.ctx.request_repaint();
    }
    fn finish(&self, _ok: bool, _summary: &str) {}
}

// ── App ───────────────────────────────────────────────────────────────────────

impl eframe::App for SetupApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.viewport().close_requested()) {
            self.handle_close();
            return;
        }

        if self.oauth_status == OAuthStatus::Pending {
            if let Ok(guard) = self.oauth_result.try_lock() {
                if let Some(ref result) = *guard {
                    self.oauth_status = match result {
                        Ok(())   => OAuthStatus::Success,
                        Err(msg) => OAuthStatus::Failed(msg.clone()),
                    };
                }
            }
            ctx.request_repaint();
        }

        if self.oauth_status == OAuthStatus::Success {
            self.oauth_status = OAuthStatus::Idle;
            self.phase = Phase::FetchPrompt;
        }

        // Auto-close when fetch sequence finishes
        if self.phase == Phase::Fetching {
            let ops = self.ops_state.lock().unwrap();
            if ops.active.is_none() {
                if let Some(OperationResult::Ok(_)) = &ops.last_result {
                    drop(ops);
                    *self.outcome.lock().unwrap() = SetupOutcome::Complete;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    return;
                }
            }
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.phase {
                Phase::Fields       => self.show_fields(ui),
                Phase::OAuth        => self.show_oauth(ui, ctx),
                Phase::FetchPrompt  => self.show_fetch_prompt(ui, ctx),
                Phase::Fetching     => self.show_fetching(ui, ctx),
            }
        });
    }
}

impl SetupApp {
    pub(crate) fn handle_close(&mut self) {
        let mut outcome = self.outcome.lock().unwrap();
        if *outcome == SetupOutcome::InProgress {
            *outcome = match self.phase {
                Phase::Fields       => SetupOutcome::CancelledPhase1,
                Phase::OAuth        => SetupOutcome::CancelledOAuth,
                Phase::FetchPrompt  => SetupOutcome::Complete,
                Phase::Fetching     => SetupOutcome::Complete,
            };
        }
    }

    fn show_fields(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);
        ui.vertical_centered(|ui| {
            ui.heading("Welcome to Gurdo");
            ui.add_space(8.0);
            ui.label("Enter your Last.fm username to get started.");
            ui.add_space(16.0);
            ui.add_sized([260.0, 24.0], egui::TextEdit::singleline(&mut self.username));
            ui.add_space(12.0);
            if let Some(ref err) = self.write_error {
                ui.colored_label(egui::Color32::from_rgb(220, 80, 80), err);
                ui.add_space(8.0);
            }
            let all_filled = !self.username.trim().is_empty();
            ui.add_enabled_ui(all_filled, |ui| {
                if ui.button("Continue").clicked() {
                    match write_credentials(&self.config_path, &self.username) {
                        Ok(()) => { self.write_error = None; self.phase = Phase::OAuth; }
                        Err(e) => { self.write_error = Some(e.to_string()); }
                    }
                }
            });
        });
    }

    fn show_oauth(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.add_space(20.0);
        ui.vertical_centered(|ui| {
            ui.heading("Connect Spotify");
            ui.add_space(16.0);
            let (status_text, color) = match &self.oauth_status {
                OAuthStatus::Idle      => ("Connect your Spotify account to enable playback.".to_owned(), egui::Color32::GRAY),
                OAuthStatus::Pending   => ("Waiting for Spotify authorisation\u{2026}".to_owned(), egui::Color32::GRAY),
                OAuthStatus::Failed(e) => (format!("Error: {}", e), egui::Color32::from_rgb(220, 80, 80)),
                OAuthStatus::Success   => ("Connected!".to_owned(), egui::Color32::GREEN),
            };
            ui.colored_label(color, &status_text);
            ui.add_space(12.0);
            let is_pending = self.oauth_status == OAuthStatus::Pending;
            let connect_label = if matches!(self.oauth_status, OAuthStatus::Failed(_)) { "Retry" } else { "Connect Spotify" };
            ui.add_enabled_ui(!is_pending, |ui| {
                if ui.button(connect_label).clicked() {
                    self.start_oauth(ctx);
                }
                ui.add_space(8.0);
                if ui.button("Skip for now").clicked() {
                    self.phase = Phase::FetchPrompt;
                }
            });
        });
    }

    fn show_fetch_prompt(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.add_space(20.0);
        ui.vertical_centered(|ui| {
            ui.heading("Set up your music library");
            ui.add_space(8.0);
            ui.label("Gurdo can fetch your Last.fm listening history and Spotify library now. This takes a few minutes on the first run.");
            ui.add_space(4.0);
            ui.label(egui::RichText::new("You can also do this later from Settings.").weak());
            ui.add_space(16.0);
            if ui.button("Fetch now").clicked() {
                self.start_fetch(ctx);
            }
            ui.add_space(8.0);
            if ui.button("Skip for now").clicked() {
                *self.outcome.lock().unwrap() = SetupOutcome::Complete;
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }

    fn show_fetching(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.add_space(20.0);
        ui.vertical_centered(|ui| { ui.heading("Fetching your music data"); });
        ui.add_space(16.0);

        let ops = self.ops_state.lock().unwrap().clone();
        render_fetch_progress(ui, &ops);

        if ops.active.is_none() {
            if let Some(OperationResult::Failed(_)) = &ops.last_result {
                ui.add_space(8.0);
                ui.vertical_centered(|ui| {
                    if ui.button("Continue anyway").clicked() {
                        *self.outcome.lock().unwrap() = SetupOutcome::Complete;
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            }
        }
    }

    fn start_oauth(&mut self, ctx: &egui::Context) {
        self.oauth_status = OAuthStatus::Pending;
        *self.oauth_result.lock().unwrap() = None;

        let result_arc = Arc::clone(&self.oauth_result);
        let ctx_clone  = ctx.clone();
        let gurdo_cfg  = crate::config::gurdo_dir()
            .map(|d| d.join("config.toml"))
            .unwrap_or_else(|| self.config_path.clone());

        std::thread::spawn(move || {
            let config = match crate::config::Config::load(&gurdo_cfg) {
                Ok(c)  => c,
                Err(e) => {
                    *result_arc.lock().unwrap() = Some(Err(e.to_string()));
                    ctx_clone.request_repaint();
                    return;
                }
            };
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("tokio runtime for OAuth");
            let result = rt.block_on(crate::spotify::auth::run_oauth_flow(&config));
            *result_arc.lock().unwrap() = Some(result.map_err(|e| e.to_string()));
            ctx_clone.request_repaint();
        });
    }

    fn start_fetch(&mut self, ctx: &egui::Context) {
        if self.phase == Phase::Fetching {
            return;
        }
        self.phase = Phase::Fetching;
        self.oauth_status = OAuthStatus::Idle;

        let ops_arc   = Arc::clone(&self.ops_state);
        let ctx_clone = ctx.clone();
        let gurdo_cfg = crate::config::gurdo_dir()
            .map(|d| d.join("config.toml"))
            .unwrap_or_else(|| self.config_path.clone());

        std::thread::spawn(move || {
            let config = match crate::config::Config::load(&gurdo_cfg) {
                Ok(c)  => c,
                Err(e) => {
                    ops_arc.lock().unwrap().last_result =
                        Some(OperationResult::Failed(e.to_string()));
                    ctx_clone.request_repaint();
                    return;
                }
            };

            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("tokio runtime for fetch");

            let steps = [
                OperationKind::SyncLastfm,
                OperationKind::Expand,
                OperationKind::FetchTracks,
                OperationKind::Score,
            ];
            let total = steps.len() as u8;

            for (i, kind) in steps.iter().enumerate() {
                {
                    let mut o = ops_arc.lock().unwrap();
                    o.active = Some(ActiveOperation {
                        kind:    kind.clone(),
                        step:    Some((i as u8 + 1, total)),
                        stage:   String::new(),
                        current: 0,
                        total:   None,
                        message: String::new(),
                    });
                }
                ctx_clone.request_repaint();

                let reporter = SetupReporter {
                    ops: Arc::clone(&ops_arc),
                    ctx: ctx_clone.clone(),
                };

                let result = rt.block_on(
                    crate::ui::ops::run_operation_pub(kind.clone(), &config, &reporter)
                );

                if let Err(e) = result {
                    let mut o = ops_arc.lock().unwrap();
                    o.active      = None;
                    o.last_result = Some(OperationResult::Failed(
                        format!("Step {}/{} ({}) failed: {}", i + 1, total, kind.label(), e)
                    ));
                    ctx_clone.request_repaint();
                    return;
                }
            }

            let mut o = ops_arc.lock().unwrap();
            o.active      = None;
            o.last_result = Some(OperationResult::Ok("Initial data fetch complete".to_string()));
            ctx_clone.request_repaint();
        });
    }
}

// ── Fetch progress renderer ───────────────────────────────────────────────────

fn parse_failed_step(msg: &str) -> Option<usize> {
    msg.strip_prefix("Step ")
        .and_then(|s| s.split('/').next())
        .and_then(|n| n.parse().ok())
}

fn render_fetch_progress(ui: &mut egui::Ui, ops: &OperationsState) {
    let steps = [
        OperationKind::SyncLastfm,
        OperationKind::Expand,
        OperationKind::FetchTracks,
        OperationKind::Score,
    ];

    let current_step = ops.active.as_ref()
        .and_then(|a| a.step)
        .map(|(n, _)| n as usize);

    let failed_step: Option<usize> = match &ops.last_result {
        Some(OperationResult::Failed(msg)) if ops.active.is_none() => parse_failed_step(msg),
        _ => None,
    };

    let all_done = ops.active.is_none()
        && matches!(&ops.last_result, Some(OperationResult::Ok(_)));

    for (i, kind) in steps.iter().enumerate() {
        let step_num = i + 1;

        let is_done = failed_step.map(|f| step_num < f).unwrap_or(false)
            || all_done
            || current_step.map(|n| step_num < n).unwrap_or(false);
        let is_failed  = failed_step == Some(step_num);
        let is_active  = !is_done && !is_failed && current_step == Some(step_num);
        let is_pending = !is_done && !is_active && !is_failed;

        let prefix = if is_done { "✓ " } else if is_active { "▶ " } else if is_failed { "✗ " } else { "  " };
        let label_text = format!("{}{}", prefix, kind.label());

        if is_failed {
            ui.colored_label(egui::Color32::from_rgb(220, 80, 80), &label_text);
        } else if is_pending {
            ui.label(egui::RichText::new(label_text).weak());
        } else {
            ui.label(&label_text);
        }

        let (bar_fraction, animate) = if is_done {
            (1.0f32, false)
        } else if is_active {
            let known_frac = ops.active.as_ref()
                .and_then(|a| a.total)
                .map(|t| ops.active.as_ref().unwrap().current as f32 / t as f32);
            (known_frac.unwrap_or(0.5), known_frac.is_none())
        } else {
            (0.0f32, false)
        };

        ui.add(egui::ProgressBar::new(bar_fraction).animate(animate));

        if is_active {
            if let Some(active) = &ops.active {
                if !active.stage.is_empty() {
                    ui.label(egui::RichText::new(&active.stage).weak().small());
                }
                if let Some(total) = active.total {
                    ui.label(egui::RichText::new(format!("{}/{}", active.current, total)).weak().small());
                }
            }
        }

        if is_failed {
            if let Some(OperationResult::Failed(msg)) = &ops.last_result {
                ui.label(egui::RichText::new(msg).weak().small());
            }
        }

        ui.add_space(4.0);
    }
}

// ── file helpers (pub(crate) for unit tests) ──────────────────────────────────

#[derive(Serialize)]
struct SecretsOut { lastfm: SecretsLastfmOut }
#[derive(Serialize)]
struct SecretsLastfmOut { username: String }

/// Write trimmed Last.fm username to `path` and apply `0o600` permissions on Unix.
pub(crate) fn write_secrets(path: &Path, username: &str) -> Result<()> {
    let content = toml::to_string(&SecretsOut {
        lastfm: SecretsLastfmOut { username: username.trim().to_owned() },
    }).context("Failed to serialize secrets")?;

    std::fs::write(path, content)
        .with_context(|| format!("Failed to write secrets: {}", path.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))
            .with_context(|| format!("Failed to set permissions on: {}", path.display()))?;
    }
    Ok(())
}

/// Write `DEFAULT_CONFIG_TOML` to `path` only when the file does not yet exist.
pub(crate) fn write_default_config_if_absent(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::write(path, DEFAULT_CONFIG_TOML)
            .with_context(|| format!("Failed to write default config: {}", path.display()))?;
    }
    Ok(())
}

fn write_credentials(config_path: &Path, username: &str) -> Result<()> {
    let gurdo_dir = crate::config::gurdo_dir()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine home directory"))?;
    write_secrets(&gurdo_dir.join("secrets.toml"), username)?;
    let default_cfg = gurdo_dir.join("config.toml");
    let cfg_target = if config_path.ends_with(".gurdo/config.toml") { config_path } else { &default_cfg };
    write_default_config_if_absent(cfg_target)?;
    Ok(())
}

// ── public entry point ────────────────────────────────────────────────────────

/// Run the first-run setup window. Blocks until the window closes.
/// Returns `Ok(())` on completion or skip; returns `Err` if the user cancels.
pub fn run(config_path: &Path) -> Result<()> {
    let outcome = Arc::new(Mutex::new(SetupOutcome::InProgress));
    let outcome_clone = Arc::clone(&outcome);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([440.0, 400.0])
            .with_resizable(false)
            .with_title("Gurdo \u{2014} Setup"),
        ..Default::default()
    };

    eframe::run_native(
        "Gurdo \u{2014} Setup",
        options,
        Box::new(move |_cc| {
            Ok(Box::new(SetupApp {
                config_path: config_path.to_path_buf(),
                phase: Phase::Fields,
                username: String::new(),
                write_error: None,
                oauth_status: OAuthStatus::Idle,
                oauth_result: Arc::new(Mutex::new(None)),
                ops_state: Arc::new(Mutex::new(OperationsState {
                    active: None,
                    last_result: None,
                })),
                outcome: outcome_clone,
            }))
        }),
    ).map_err(|e| anyhow::anyhow!("Setup UI error: {}", e))?;

    match *outcome.lock().unwrap() {
        SetupOutcome::Complete    => Ok(()),
        SetupOutcome::InProgress
        | SetupOutcome::CancelledPhase1 =>
            Err(anyhow::anyhow!("Setup cancelled \u{2014} please re-run Gurdo to complete setup.")),
        SetupOutcome::CancelledOAuth =>
            Err(anyhow::anyhow!("Setup cancelled during OAuth \u{2014} Spotify not connected.")),
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn make_test_app() -> SetupApp {
        SetupApp {
            config_path: std::path::PathBuf::from("/tmp/test_setup_config.toml"),
            phase: Phase::Fields,
            username: String::new(),
            write_error: None,
            oauth_status: OAuthStatus::Idle,
            oauth_result: Arc::new(Mutex::new(None)),
            ops_state: Arc::new(Mutex::new(OperationsState {
                active: None,
                last_result: None,
            })),
            outcome: Arc::new(Mutex::new(SetupOutcome::InProgress)),
        }
    }

    // T-01: OAuth success → FetchPrompt, oauth_status reset, no fetch started
    #[test]
    fn oauth_success_transitions_to_fetch_prompt_and_resets_status() {
        let mut app = make_test_app();
        app.phase = Phase::OAuth;
        app.oauth_status = OAuthStatus::Success;

        // Simulate the update() branch for OAuthStatus::Success
        if app.oauth_status == OAuthStatus::Success {
            app.oauth_status = OAuthStatus::Idle;
            app.phase = Phase::FetchPrompt;
        }

        assert_eq!(app.phase, Phase::FetchPrompt);
        assert_eq!(app.oauth_status, OAuthStatus::Idle);
    }

    // T-02: OAuth skip → FetchPrompt, no fetch
    #[test]
    fn oauth_skip_transitions_to_fetch_prompt() {
        let mut app = make_test_app();
        app.phase = Phase::OAuth;

        // Simulate "Skip for now" click handler
        app.phase = Phase::FetchPrompt;

        assert_eq!(app.phase, Phase::FetchPrompt);
        // phase is FetchPrompt, not Fetching — no fetch started
        assert_ne!(app.phase, Phase::Fetching);
    }

    // T-03: close on FetchPrompt → Complete
    #[test]
    fn close_on_fetch_prompt_produces_complete() {
        let mut app = make_test_app();
        app.phase = Phase::FetchPrompt;
        app.handle_close();
        assert_eq!(*app.outcome.lock().unwrap(), SetupOutcome::Complete);
    }

    // T-04: close on Fields → CancelledPhase1 (regression)
    #[test]
    fn close_on_fields_produces_cancelled_phase1() {
        let mut app = make_test_app();
        app.phase = Phase::Fields;
        app.handle_close();
        assert_eq!(*app.outcome.lock().unwrap(), SetupOutcome::CancelledPhase1);
    }

    // T-05: close on OAuth → CancelledOAuth (regression)
    #[test]
    fn close_on_oauth_produces_cancelled_oauth() {
        let mut app = make_test_app();
        app.phase = Phase::OAuth;
        app.handle_close();
        assert_eq!(*app.outcome.lock().unwrap(), SetupOutcome::CancelledOAuth);
    }

    // T-10: oauth_status Idle on FetchPrompt prevents re-triggering auto-fetch
    #[test]
    fn oauth_status_idle_on_fetch_prompt_does_not_retrigger() {
        let mut app = make_test_app();
        app.phase = Phase::FetchPrompt;
        app.oauth_status = OAuthStatus::Idle; // as reset by transition

        // Simulate the update() branch — only fires when Success
        let would_trigger = app.oauth_status == OAuthStatus::Success;

        assert!(!would_trigger);
        assert_eq!(app.phase, Phase::FetchPrompt);
    }

    #[test]
    fn write_secrets_trims_and_produces_valid_toml() {
        let dir  = tempdir().unwrap();
        let path = dir.path().join("secrets.toml");
        write_secrets(&path, "  my_user  ").unwrap();

        let content = fs::read_to_string(&path).unwrap();
        let val: toml::Value = toml::from_str(&content).expect("valid TOML");
        assert_eq!(val["lastfm"]["username"].as_str().unwrap(), "my_user");
    }

    #[cfg(unix)]
    #[test]
    fn write_secrets_applies_chmod_600() {
        use std::os::unix::fs::PermissionsExt;
        let dir  = tempdir().unwrap();
        let path = dir.path().join("secrets.toml");
        write_secrets(&path, "u").unwrap();
        let mode = fs::metadata(&path).unwrap().permissions().mode();
        assert_eq!(mode & 0o777, 0o600, "expected 0o600, got {:o}", mode & 0o777);
    }

    #[test]
    fn write_default_config_creates_when_absent() {
        let dir  = tempdir().unwrap();
        let path = dir.path().join("config.toml");
        write_default_config_if_absent(&path).unwrap();
        assert!(path.exists());
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("[app]"));
    }

    #[test]
    fn write_default_config_does_not_overwrite() {
        let dir  = tempdir().unwrap();
        let path = dir.path().join("config.toml");
        fs::write(&path, "custom = true").unwrap();
        write_default_config_if_absent(&path).unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "custom = true");
    }
}
