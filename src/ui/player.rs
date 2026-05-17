use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use eframe::egui;

use crate::config::Config;

use super::background::BackgroundPainter;
use super::state::{OperationCommand, OperationsState, PlayerCommand, PlayerState};

// ── egui App ──────────────────────────────────────────────────────────────────

pub(super) struct GurdoApp {
    pub(super) state:                Arc<Mutex<PlayerState>>,
    pub(super) cmd_tx:               tokio::sync::mpsc::UnboundedSender<PlayerCommand>,
    pub(super) album_texture:        Option<(String, egui::TextureHandle)>,
    pub(super) placeholder_texture:  Option<egui::TextureHandle>,
    pub(super) blur:                 BackgroundPainter,
    pub(super) config_path:          PathBuf,
    pub(super) shared_config:        Arc<Mutex<Config>>,
    pub(super) settings_draft:       Arc<Mutex<Option<Config>>>,
    pub(super) settings_open:        Arc<AtomicBool>,
    pub(super) settings_initial_pos: Option<egui::Pos2>,
    pub(super) ops_state:            Arc<Mutex<OperationsState>>,
    pub(super) ops_cmd_tx:           tokio::sync::mpsc::UnboundedSender<OperationCommand>,
}

impl eframe::App for GurdoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_secs(1));

        if self.placeholder_texture.is_none() {
            if let Ok(img) = decode_image(super::assets::PLACEHOLDER_COVER) {
                self.placeholder_texture = Some(
                    ctx.load_texture("placeholder_cover", img, Default::default())
                );
            }
        }

        let state = self.state.lock().unwrap().clone();

        if state.album_art_bytes.is_none() {
            self.album_texture = None;
        } else if let Some(bytes) = &state.album_art_bytes {
            let url = state.album_art_url.as_deref().unwrap_or("");
            let stale = self.album_texture.as_ref()
                .map(|(cached, _)| cached != url)
                .unwrap_or(true);

            if stale && !bytes.is_empty() {
                if let Ok(img) = decode_image(bytes) {
                    let tex = ctx.load_texture("album_art", img, Default::default());
                    self.album_texture = Some((url.to_string(), tex));
                }
            }
        }

        let (cover_url, cover_bytes) = {
            let s = self.state.lock().unwrap();
            (s.album_art_url.clone(), s.album_art_bytes.clone())
        };
        self.blur.update(ctx, cover_url.as_deref(), cover_bytes.as_deref());
        let fallback = self.shared_config.lock().unwrap().ui.background_color;
        self.blur.paint(ctx, fallback);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {

                // ── Album art ─────────────────────────────────────────────
                ui.add_space(10.0);
                if let Some((_, tex)) = &self.album_texture {
                    ui.add(egui::Image::new((tex.id(), egui::vec2(400.0, 400.0))).rounding(10.0));
                } else if let Some(tex) = &self.placeholder_texture {
                    ui.add(egui::Image::new((tex.id(), egui::vec2(400.0, 400.0))).rounding(10.0));
                } else {
                    ui.allocate_space(egui::vec2(400.0, 400.0));
                }

                ui.add_space(8.0);

                // ── Track info ────────────────────────────────────────────
                ui.add(egui::Label::new(egui::RichText::new(
                    if state.track_name.is_empty() { "" } else { &state.track_name }
                ).size(22.0).strong()).truncate());
                ui.add(egui::Label::new(egui::RichText::new(
                    if state.artist_name.is_empty() { "" } else { &state.artist_name }
                ).size(14.4)).truncate());

                ui.add_space(8.0);

                // ── Progress bar ──────────────────────────────────────────
                let progress = if state.duration_ms > 0 {
                    state.progress_ms as f32 / state.duration_ms as f32
                } else {
                    0.0
                };

                ui.visuals_mut().extreme_bg_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, 25);
                ui.add(
                    egui::ProgressBar::new(progress)
                        .desired_width(380.0)
                        .fill(egui::Color32::from_rgba_unmultiplied(255, 255, 255, 160)),
                );
                let snoozed = state.api_error_snooze_until
                    .map(|t| t > std::time::Instant::now())
                    .unwrap_or(false);
                if snoozed {
                    ui.label(egui::RichText::new("⚠ Spotify API unavailable")
                        .color(egui::Color32::from_rgb(255, 180, 0))
                        .size(10.0));
                } else {
                    ui.label(egui::RichText::new(format!("{} / {}",
                        fmt_ms(state.progress_ms), fmt_ms(state.duration_ms))).size(10.0));
                }

                ui.add_space(8.0);

                // Ghost-button visuals shared by all control rows
                {
                    let w = &mut ui.visuals_mut().widgets;
                    w.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
                    w.inactive.bg_fill      = egui::Color32::TRANSPARENT;
                    w.inactive.bg_stroke    = egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 60));
                    w.inactive.fg_stroke    = egui::Stroke::new(1.0, egui::Color32::WHITE);
                    w.hovered.weak_bg_fill  = egui::Color32::from_rgba_unmultiplied(255, 255, 255, 20);
                    w.hovered.bg_fill       = egui::Color32::from_rgba_unmultiplied(255, 255, 255, 20);
                    w.hovered.bg_stroke     = egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 100));
                    w.hovered.fg_stroke     = egui::Stroke::new(1.0, egui::Color32::WHITE);
                    w.active.weak_bg_fill   = egui::Color32::from_rgba_unmultiplied(255, 255, 255, 40);
                    w.active.bg_fill        = egui::Color32::from_rgba_unmultiplied(255, 255, 255, 40);
                    w.active.bg_stroke      = egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 140));
                    w.active.fg_stroke      = egui::Stroke::new(1.0, egui::Color32::WHITE);
                }

                // ── Playback controls (centered, 60×60) ───────────────────
                ui.horizontal(|ui| {
                    let btn_size  = egui::vec2(60.0, 60.0);
                    let spacing   = ui.style().spacing.item_spacing.x;
                    let total     = btn_size.x * 5.0 + spacing * 4.0;
                    ui.add_space((ui.available_width() - total) / 2.0);
                    let rounding  = egui::Rounding::same(6.0);
                    let icon_size = 28.0;
                    if ui.add_sized(btn_size, egui::Button::new(egui::RichText::new("⏮").size(icon_size)).rounding(rounding)).clicked() {
                        let _ = self.cmd_tx.send(PlayerCommand::Previous);
                    }
                    if ui.add_sized(btn_size, egui::Button::new(egui::RichText::new("⏪").size(icon_size)).rounding(rounding)).clicked() {
                        let _ = self.cmd_tx.send(PlayerCommand::SeekRelative(-10_000));
                    }
                    if ui.add_sized(btn_size, egui::Button::new(egui::RichText::new(if state.is_playing { "⏸" } else { "▶" }).size(icon_size)).rounding(rounding)).clicked() {
                        let _ = self.cmd_tx.send(PlayerCommand::PlayPause);
                    }
                    if ui.add_sized(btn_size, egui::Button::new(egui::RichText::new("⏩").size(icon_size)).rounding(rounding)).clicked() {
                        let _ = self.cmd_tx.send(PlayerCommand::SeekRelative(10_000));
                    }
                    if ui.add_sized(btn_size, egui::Button::new(egui::RichText::new("⏭").size(icon_size)).rounding(rounding)).clicked() {
                        let _ = self.cmd_tx.send(PlayerCommand::Next);
                    }
                });

                ui.add_space(8.0);

                // ── Like / Dislike / Queue / Settings ────────────────────
                ui.horizontal(|ui| {
                    let fb_size   = egui::vec2(130.0, 38.0);
                    let icon_size = egui::vec2(38.0, 38.0);
                    let spacing   = ui.style().spacing.item_spacing.x;
                    let total     = fb_size.x * 2.0 + icon_size.x * 2.0 + spacing * 3.0;
                    ui.add_space((ui.available_width() - total) / 2.0);
                    let rounding = egui::Rounding::same(6.0);
                    let is_liked = state.feedback == Some(true);
                    let like_label = egui::RichText::new(if is_liked { "♥  Unlike" } else { "♥  Like" })
                        .color(if is_liked { egui::Color32::from_rgb(29, 185, 84) } else { egui::Color32::WHITE });
                    if ui.add_sized(fb_size, egui::Button::new(like_label).rounding(rounding)).clicked() {
                        if is_liked {
                            self.state.lock().unwrap().feedback = None;
                            let _ = self.cmd_tx.send(PlayerCommand::UnlikeTrack);
                        } else {
                            self.state.lock().unwrap().feedback = Some(true);
                            let _ = self.cmd_tx.send(PlayerCommand::SaveTrack);
                        }
                    }
                    let dislike_label = egui::RichText::new("👎  Dislike")
                        .color(if state.feedback == Some(false) { egui::Color32::RED } else { egui::Color32::WHITE });
                    if ui.add_sized(fb_size, egui::Button::new(dislike_label).rounding(rounding)).clicked() && state.feedback != Some(false) {
                        self.state.lock().unwrap().feedback = Some(false);
                        let _ = self.cmd_tx.send(PlayerCommand::RemoveTrack);
                        let _ = self.cmd_tx.send(PlayerCommand::Next);
                    }
                    if ui.add_sized(icon_size, egui::Button::new("☰").rounding(rounding)).clicked() {
                        let _ = self.cmd_tx.send(PlayerCommand::StartQueue);
                    }
                    if ui.add_sized(icon_size, egui::Button::new("⚙").rounding(rounding)).clicked() {
                        let was_open = self.settings_open.load(Ordering::Relaxed);
                        if !was_open {
                            let player_rect = ctx.input(|i| i.viewport().outer_rect).unwrap_or(egui::Rect::ZERO);
                            let [sw, sh] = self.shared_config.lock().unwrap().ui.settings_window_size;
                            self.settings_initial_pos = Some(egui::pos2(
                                player_rect.center().x - sw as f32 / 2.0,
                                player_rect.center().y - sh as f32 / 2.0,
                            ));
                        }
                        self.settings_open.store(!was_open, Ordering::Relaxed);
                    }
                });

                ui.add_space(20.0);
            });
        });

        // ── Error modal ───────────────────────────────────────────────────────
        if let Some(err) = state.error.clone() {
            egui::Window::new("⚠  Error")
                .resizable(false)
                .collapsible(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .order(egui::Order::Foreground)
                .show(ctx, |ui| {
                    ui.set_min_width(280.0);
                    ui.label(&err);
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui.button("OK").clicked() {
                            self.state.lock().unwrap().error = None;
                        }
                        if ui.button("Snooze 10 min").clicked() {
                            let mut s = self.state.lock().unwrap();
                            s.api_error_snooze_until = Some(
                                std::time::Instant::now() + std::time::Duration::from_secs(600)
                            );
                            s.error = None;
                        }
                    });
                });
        }

        // ── Settings viewport ─────────────────────────────────────────────────
        if self.settings_open.load(Ordering::Relaxed) {
            let [sw, sh] = self.shared_config.lock().unwrap().ui.settings_window_size;
            let pos = self.settings_initial_pos.unwrap_or(egui::Pos2::ZERO);
            let settings_open    = Arc::clone(&self.settings_open);
            let ops_state        = Arc::clone(&self.ops_state);
            let ops_cmd_tx       = self.ops_cmd_tx.clone();
            let shared_config    = Arc::clone(&self.shared_config);
            let settings_draft   = Arc::clone(&self.settings_draft);
            let config_path      = self.config_path.clone();
            ctx.show_viewport_deferred(
                egui::ViewportId::from_hash_of("settings"),
                egui::ViewportBuilder::default()
                    .with_title("Gurdo — Settings")
                    .with_inner_size([sw as f32, sh as f32])
                    .with_position(pos),
                move |ctx, _class| {
                    if ctx.input(|i| i.viewport().close_requested()) {
                        settings_open.store(false, Ordering::Relaxed);
                    }
                    super::settings::render(
                        ctx, &settings_open,
                        &ops_state, &ops_cmd_tx,
                        &shared_config, &settings_draft, &config_path,
                    );
                },
            );
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn decode_image(bytes: &[u8]) -> anyhow::Result<egui::ColorImage> {
    let img = image::load_from_memory(bytes)?;
    let size = [img.width() as usize, img.height() as usize];
    let pixels = img.to_rgba8();
    Ok(egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_flat_samples().as_slice()))
}

fn fmt_ms(ms: u64) -> String {
    let s = ms / 1000;
    format!("{}:{:02}", s / 60, s % 60)
}
