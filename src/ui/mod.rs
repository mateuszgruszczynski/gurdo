mod assets;
mod background;
mod knobs;
mod ops;
mod player;
mod poll;
mod settings;
mod state;
pub mod setup;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use eframe::egui;

use crate::config::Config;

use ops::ops_dispatcher_loop;
use player::GurdoApp;
use poll::polling_loop;
use state::{OperationCommand, OperationsState, PlayerState};

pub fn run(config: Config, config_path: PathBuf) -> anyhow::Result<()> {
    let state: Arc<Mutex<PlayerState>> = Arc::new(Mutex::new(PlayerState::default()));
    let (cmd_tx, cmd_rx) = tokio::sync::mpsc::unbounded_channel();

    let ops_state: Arc<Mutex<OperationsState>> = Arc::new(Mutex::new(OperationsState::default()));
    let (ops_cmd_tx, ops_cmd_rx) = tokio::sync::mpsc::unbounded_channel::<OperationCommand>();

    let shared_config  = Arc::new(Mutex::new(config));
    let settings_draft: Arc<Mutex<Option<Config>>> = Arc::new(Mutex::new(None));

    let state_bg        = state.clone();
    let shared_cfg_bg   = shared_config.clone();
    let ops_state_bg    = Arc::clone(&ops_state);
    let shared_cfg_bg2  = shared_config.clone();
    let settings_draft_bg = Arc::clone(&settings_draft);
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("tokio runtime");
        rt.block_on(async {
            tokio::join!(
                polling_loop(state_bg, cmd_rx, shared_cfg_bg),
                ops_dispatcher_loop(ops_cmd_rx, ops_state_bg, shared_cfg_bg2, settings_draft_bg),
            );
        });
    });

    let [pw, ph] = shared_config.lock().unwrap().ui.player_window_size;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([pw as f32, ph as f32])
            .with_resizable(false)
            .with_title("Gurdo"),
        ..Default::default()
    };

    eframe::run_native(
        "Gurdo",
        options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();

            fonts.font_data.insert("noto_sans_jp".into(),
                egui::FontData::from_static(assets::NOTO_SANS_JP));
            fonts.font_data.insert("noto_sans_sc".into(),
                egui::FontData::from_static(assets::NOTO_SANS_SC));
            fonts.font_data.insert("noto_sans_kr".into(),
                egui::FontData::from_static(assets::NOTO_SANS_KR));

            let proportional = fonts.families
                .entry(egui::FontFamily::Proportional)
                .or_default();
            proportional.push("noto_sans_jp".into());
            proportional.push("noto_sans_sc".into());
            proportional.push("noto_sans_kr".into());

            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(GurdoApp {
                state,
                cmd_tx,
                album_texture: None,
                placeholder_texture: None,
                blur: background::BackgroundPainter::new(),
                config_path,
                shared_config,
                settings_open:        std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
                settings_initial_pos: None,
                ops_state,
                ops_cmd_tx,
                settings_draft,
            }))
        }),
    ).map_err(|e| anyhow::anyhow!("UI error: {}", e))
}
