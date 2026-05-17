// Security audit: no credential values are emitted in any tracing/log/eprintln/dbg call
// in this module. Field names may appear in log output, but never their values.

use anyhow::{Context, Result};
use eframe::egui;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

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
enum Phase { Fields, OAuth }

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
    outcome: Arc<Mutex<SetupOutcome>>,
}

impl eframe::App for SetupApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.viewport().close_requested()) {
            let mut outcome = self.outcome.lock().unwrap();
            if *outcome == SetupOutcome::InProgress {
                *outcome = match self.phase {
                    Phase::Fields => SetupOutcome::CancelledPhase1,
                    Phase::OAuth  => SetupOutcome::CancelledOAuth,
                };
            }
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
            *self.outcome.lock().unwrap() = SetupOutcome::Complete;
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.phase {
                Phase::Fields => self.show_fields(ui),
                Phase::OAuth  => self.show_oauth(ui, ctx),
            }
        });
    }
}

impl SetupApp {
    fn show_fields(&mut self, ui: &mut egui::Ui) {
        ui.heading("Welcome to Gurdo");
        ui.add_space(8.0);
        ui.label("Enter your Last.fm username to get started.");
        ui.add_space(16.0);

        ui.label("Last.fm Username");
        ui.add(egui::TextEdit::singleline(&mut self.username).desired_width(f32::INFINITY));
        ui.add_space(12.0);

        if let Some(ref err) = self.write_error {
            ui.colored_label(egui::Color32::RED, err);
            ui.add_space(8.0);
        }

        let all_filled = !self.username.trim().is_empty();

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
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
        ui.heading("Connect Spotify");
        ui.add_space(16.0);

        let (status_text, color) = match &self.oauth_status {
            OAuthStatus::Idle      => ("Connect your Spotify account to enable playback.".to_owned(), egui::Color32::GRAY),
            OAuthStatus::Pending   => ("Waiting for Spotify authorisation\u{2026}".to_owned(), egui::Color32::GRAY),
            OAuthStatus::Failed(e) => (format!("Error: {}", e), egui::Color32::RED),
            OAuthStatus::Success   => ("Connected!".to_owned(), egui::Color32::GREEN),
        };
        ui.colored_label(color, &status_text);
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new(
                "If your browser shows a certificate warning for 127.0.0.1,\n\
                 click \u{201c}Advanced\u{201d} \u{2192} \u{201c}Proceed to 127.0.0.1\u{201d} to continue."
            ).weak().small()
        );
        ui.add_space(12.0);

        let is_pending = self.oauth_status == OAuthStatus::Pending;
        let connect_label = if matches!(self.oauth_status, OAuthStatus::Failed(_)) { "Retry" } else { "Connect Spotify" };

        ui.add_enabled_ui(!is_pending, |ui| {
            ui.vertical_centered(|ui| {
                if ui.button(connect_label).clicked() {
                    self.start_oauth(ctx);
                }
                ui.add_space(8.0);
                if ui.button("Skip for now").clicked() {
                    *self.outcome.lock().unwrap() = SetupOutcome::Complete;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
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
