use std::path::Path;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use eframe::egui;
use tokio::sync::mpsc::UnboundedSender;

use crate::config::Config;
use super::ops::token_exists;
use super::state::{OperationCommand, OperationKind, OperationResult, OperationsState};

pub(super) fn render(
    ctx: &egui::Context,
    settings_open: &Arc<AtomicBool>,
    ops_state: &Arc<Mutex<OperationsState>>,
    ops_cmd_tx: &UnboundedSender<OperationCommand>,
    shared_config: &Arc<Mutex<Config>>,
    settings_draft: &Arc<Mutex<Option<Config>>>,
    config_path: &Path,
) {
    let ops = ops_state.lock().unwrap().clone();
    let busy = ops.active.is_some();

    // Work on a display copy — either the live draft or shared config.
    let mut display = {
        let draft = settings_draft.lock().unwrap();
        match &*draft {
            Some(d) => d.clone(),
            None    => shared_config.lock().unwrap().clone(),
        }
    };
    let dirty = settings_draft.lock().unwrap().is_some();

    let mut any_changed = false;

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {

            // ── Data section ───────────────────────────────────────────────
            ui.add_space(8.0);
            ui.heading("Data");
            ui.separator();
            ui.add_space(4.0);

            ui.add_enabled_ui(!busy, |ui| {
                if ui.button("Update everything").clicked() {
                    let _ = ops_cmd_tx.send(OperationCommand::UpdateAll);
                }
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    if ui.button("Sync Last.fm").clicked() {
                        let _ = ops_cmd_tx.send(OperationCommand::Run(OperationKind::SyncLastfm));
                    }
                    if ui.button("Expand similar artists").clicked() {
                        let _ = ops_cmd_tx.send(OperationCommand::Run(OperationKind::Expand));
                    }
                    if ui.button("Fetch top tracks").clicked() {
                        let _ = ops_cmd_tx.send(OperationCommand::Run(OperationKind::FetchTracks));
                    }
                    if ui.button("Recalculate scores").clicked() {
                        let _ = ops_cmd_tx.send(OperationCommand::Run(OperationKind::Score));
                    }
                });
            });

            if let Some(active) = &ops.active {
                ui.add_space(4.0);
                let step_prefix = active.step
                    .map(|(n, t)| format!("Step {}/{}: ", n, t))
                    .unwrap_or_default();
                ui.label(format!("{}{}: {}", step_prefix, active.kind.label(), active.stage));
                if let Some(total) = active.total {
                    ui.label(format!("{}/{}", active.current, total));
                } else if active.current > 0 {
                    ui.label(format!("{}", active.current));
                }
            }

            if ops.active.is_none() {
                if let Some(result) = &ops.last_result {
                    ui.add_space(4.0);
                    match result {
                        OperationResult::Ok(s) =>
                            ui.label(egui::RichText::new(format!("✓ {}", s))
                                .color(egui::Color32::from_rgb(100, 200, 100))),
                        OperationResult::Failed(s) =>
                            ui.label(egui::RichText::new(format!("✗ {}", s))
                                .color(egui::Color32::RED)),
                    };
                }
            }

            ui.add_space(8.0);

            // ── Spotify section ────────────────────────────────────────────
            ui.heading("Spotify");
            ui.separator();
            ui.add_space(4.0);

            ui.add_enabled_ui(!busy, |ui| {
                if ui.button("Login").clicked() {
                    let _ = ops_cmd_tx.send(OperationCommand::Run(OperationKind::SpotifyLogin));
                }
            });

            let spotify_connected = token_exists(&shared_config.lock().unwrap());
            if spotify_connected {
                ui.label(egui::RichText::new("Connected").color(egui::Color32::from_rgb(100, 200, 100)));
            } else {
                ui.label(egui::RichText::new("Not connected").color(egui::Color32::GRAY));
            }

            ui.add_space(8.0);

            // ── Recommendations section ────────────────────────────────────
            ui.heading("Recommendations");
            ui.separator();
            ui.add_space(4.0);

            let r_default = crate::config::RecommendConfig::default();
            let r = &mut display.recommendations;

            knob_usize(ui, "Number of recommendations",
                "How many tracks Gurdo prepares for you each time it runs.",
                &mut r.count, 5, 500, r_default.count)
                .then(|| any_changed = true);

            knob_level_f64(ui, "Artist variety",
                "Controls whether your queue is spread across many artists or dominated by the ones you play most.",
                &mut r.artist_score_exponent,
                &["Max variety", "More variety", "Balanced", "Favour favourites", "Top artists only"],
                &[0.3, 0.6, 1.0, 1.5, 2.5])
                .then(|| any_changed = true);

            knob_level_f64(ui, "Track variety",
                "Controls whether the queue sticks to each artist's biggest hits or explores deeper cuts and B-sides.",
                &mut r.track_rank_exponent,
                &["Deep cuts welcome", "More B-sides", "Balanced", "Mostly big hits", "Hits only"],
                &[0.3, 0.6, 1.0, 1.5, 2.5])
                .then(|| any_changed = true);

            ui.add_space(8.0);

            // ── Engine section ─────────────────────────────────────────────
            ui.heading("Engine");
            ui.separator();
            ui.add_space(4.0);

            let e_default = crate::config::EngineConfig::default();
            let e = &mut display.engine;

            knob_u32(ui, "Similar artists per seed",
                "How many \"sounds like\" artists Gurdo looks up per artist you've played — more means a wider discovery net.",
                &mut e.similar_artists_limit, 5, 100, e_default.similar_artists_limit)
                .then(|| any_changed = true);

            knob_u32(ui, "Tracks per artist",
                "How many top tracks Gurdo fetches per artist — higher gives more songs to choose from per artist.",
                &mut e.artist_top_tracks_limit, 5, 200, e_default.artist_top_tracks_limit)
                .then(|| any_changed = true);

            knob_u32(ui, "Recommendation pool size",
                "Raise this for more variety before your queue is finalised; lower it to keep the selection tightly focused.",
                &mut e.recommendation_pool_size, 50, 2000, e_default.recommendation_pool_size)
                .then(|| any_changed = true);

            knob_level_f64(ui, "Similar artist influence",
                "Controls how strongly artists similar to your favourites are pulled into recommendations.",
                &mut e.similarity_multiplier,
                &["Stick to listened", "Slight exploration", "Balanced", "More similar artists", "Explore widely"],
                &[0.1, 0.25, 0.5, 1.0, 1.5])
                .then(|| any_changed = true);

            knob_level_f64(ui, "Multi-source boost",
                "Rewards artists that appear as a recommendation from several of your favourites at once.",
                &mut e.multi_source_bonus_pct,
                &["No consensus boost", "Subtle boost", "Moderate boost", "Strong boost", "Heavy consensus bias"],
                &[0.0, 0.03, 0.05, 0.10, 0.20])
                .then(|| any_changed = true);

            knob_level_f64(ui, "Loved-track bonus",
                "How much a Last.fm loved track pushes that artist higher in your recommendations.",
                &mut e.like_bonus_flat,
                &["Loves ignored", "Gentle nudge", "Noticeable boost", "Strong preference", "Loved artists first"],
                &[0.0, 2.0, 5.0, 15.0, 30.0])
                .then(|| any_changed = true);

            knob_level_f64(ui, "Dislike penalty",
                "How hard a single disliked track drops an artist in your recommendations.",
                &mut e.dislike_modifier_pct,
                &["Dislikes ignored", "Mild penalty", "Moderate penalty", "Heavy penalty", "Near-excluded"],
                &[0.0, 0.05, 0.10, 0.20, 0.50])
                .then(|| any_changed = true);

            ui.add_space(8.0);

            // ── Artist Scoring section ─────────────────────────────────────
            ui.heading("Artist Scoring");
            ui.separator();
            ui.add_space(4.0);

            let a_default = crate::config::ArtistScoringConfig::default();
            let a = &mut display.artist_scoring;

            knob_level_f64(ui, "Playcount factor",
                "Controls how much your most-played artists pull ahead of ones you only occasionally revisit.",
                &mut a.score_exponent,
                &["Minimal", "Slightly favour top played", "Favour top played"],
                &[0.1, 0.3, 0.5])
                .then(|| any_changed = true);

            knob_level_f64(ui, "Years-active bonus",
                "Rewards artists you have kept coming back to across many years of listening history.",
                &mut a.year_bonus_pct,
                &["No loyalty bonus", "Small loyalty bonus", "Moderate bonus", "Strong loyalty bonus", "Longevity first"],
                &[0.0, 2.0, 5.0, 10.0, 20.0])
                .then(|| any_changed = true);

            knob_u64(ui, "Min playcount threshold",
                "Artists you've played fewer times than this are ignored — raise it to filter out artists you've barely touched.",
                &mut a.min_playcount_threshold, 1, 500, a_default.min_playcount_threshold)
                .then(|| any_changed = true);

            ui.add_space(8.0);

            // ── Sync section ───────────────────────────────────────────────
            ui.heading("Sync");
            ui.separator();
            ui.add_space(4.0);

            let s_default = crate::config::SyncConfig::default();
            let s = &mut display.sync;

            knob_u32(ui, "Loved tracks limit",
                "How many of your Last.fm loved tracks are fetched during a sync.",
                &mut s.loved_tracks_limit, 50, 5000, s_default.loved_tracks_limit)
                .then(|| any_changed = true);

            knob_u32(ui, "Seed artists limit",
                "Raise this to pull more of your listened-to artists into recommendations; lower it to focus only on your most-played ones.",
                &mut s.seed_artists_limit, 10, 500, s_default.seed_artists_limit)
                .then(|| any_changed = true);

            knob_u32(ui, "Seed tracks limit",
                "How many of your most-listened tracks are used as the starting point when building recommendations.",
                &mut s.seed_tracks_limit, 10, 500, s_default.seed_tracks_limit)
                .then(|| any_changed = true);

            ui.add_space(8.0);

            // ── Appearance section (read-only) ─────────────────────────────
            ui.heading("Appearance");
            ui.separator();
            ui.add_space(4.0);
            {
                let cfg = shared_config.lock().unwrap();
                read_only_row(ui, "Data directory",    &cfg.data_dir().display().to_string());
                read_only_row(ui, "Database",          &cfg.db_path().display().to_string());
                read_only_row(ui, "Config file",       &config_path.display().to_string());
                read_only_row(ui, "Token file",        &cfg.token_path().display().to_string());
                read_only_row(ui, "Last.fm username",  &cfg.lastfm.username);
                let client_id_display = {
                    let id = &cfg.spotify.client_id;
                    if id.len() > 8 { format!("{}…", &id[..8]) } else { id.clone() }
                };
                read_only_row(ui, "Spotify client ID", &client_id_display);
            }

            ui.add_space(8.0);

            // ── Save / Discard ─────────────────────────────────────────────
            if dirty || any_changed {
                ui.add_space(8.0);
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.add_enabled(dirty || any_changed, egui::Button::new("• Save")).clicked() {
                        let draft = display.clone();
                        draft.save(config_path)
                            .unwrap_or_else(|e| tracing::error!("Config save failed: {}", e));
                        *shared_config.lock().unwrap() = draft;
                        *settings_draft.lock().unwrap() = None;
                    }
                    if ui.button("Discard changes").clicked() {
                        *settings_draft.lock().unwrap() = None;
                    }
                });
            }

            ui.add_space(16.0);
            ui.separator();
            ui.add_space(8.0);
            if ui.button("Close").clicked() {
                settings_open.store(false, Ordering::Relaxed);
                ctx.request_repaint_of(egui::ViewportId::ROOT);
            }
        });
    });

    // Commit any knob changes to the draft.
    if any_changed {
        *settings_draft.lock().unwrap() = Some(display);
    }

    if busy {
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

// ── Knob helpers ──────────────────────────────────────────────────────────────

fn knob_u32(ui: &mut egui::Ui, label: &str, desc: &str,
            value: &mut u32, min: u32, max: u32, default: u32) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label(label).on_hover_text(desc);
        if ui.add(egui::DragValue::new(value).speed(1.0).range(min..=max)).changed() {
            changed = true;
        }
        if ui.small_button("↺").on_hover_text("Reset to default").clicked() {
            *value = default;
            changed = true;
        }
    });
    changed
}

fn knob_u64(ui: &mut egui::Ui, label: &str, desc: &str,
            value: &mut u64, min: u64, max: u64, default: u64) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label(label).on_hover_text(desc);
        if ui.add(egui::DragValue::new(value).speed(1.0).range(min..=max)).changed() {
            changed = true;
        }
        if ui.small_button("↺").on_hover_text("Reset to default").clicked() {
            *value = default;
            changed = true;
        }
    });
    changed
}

fn knob_usize(ui: &mut egui::Ui, label: &str, desc: &str,
              value: &mut usize, min: usize, max: usize, default: usize) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label(label).on_hover_text(desc);
        if ui.add(egui::DragValue::new(value).speed(1.0).range(min..=max)).changed() {
            changed = true;
        }
        if ui.small_button("↺").on_hover_text("Reset to default").clicked() {
            *value = default;
            changed = true;
        }
    });
    changed
}

fn read_only_row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label).color(egui::Color32::GRAY));
        ui.label(value);
    });
}

fn knob_level_f64(ui: &mut egui::Ui, label: &str, desc: &str,
                  value: &mut f64, labels: &[&str], presets: &[f64]) -> bool {
    let active = closest_f64(*value, presets);
    let mut changed = false;
    ui.label(label).on_hover_text(desc);
    ui.horizontal(|ui| {
        for (i, btn_label) in labels.iter().enumerate() {
            if ui.selectable_label(active == i, *btn_label).clicked() && active != i {
                *value = presets[i];
                changed = true;
            }
        }
    });
    changed
}

fn closest_f64(value: f64, presets: &[f64]) -> usize {
    presets.iter().enumerate()
        .min_by(|(_, a), (_, b)|
            (value - *a).abs().partial_cmp(&(value - *b).abs()).unwrap())
        .map(|(i, _)| i)
        .unwrap_or(0)
}
