mod config;
mod db;
mod engine;
mod lastfm;
mod progress;
mod spotify;
mod sync;
mod ui;

use anyhow::{Context, Result};
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("gurdo=info".parse()?)
                .add_directive("gurdo::ui=debug".parse()?)
        )
        .without_time()
        .init();

    let gurdo_dir = config::gurdo_dir()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine home directory; cannot locate ~/.gurdo/"))?;
    std::fs::create_dir_all(&gurdo_dir)
        .with_context(|| "Cannot create config directory ~/.gurdo/")?;

    let config_path = parse_config_arg();

    let secrets_path = config::Config::secrets_path(&config_path);
    if config::needs_setup(&secrets_path) {
        ui::setup::run(&config_path)?;
    }

    ui::setup::write_default_config_if_absent(&config_path)?;

    let config = config::Config::load(&config_path)?;
    std::fs::create_dir_all(config.data_dir())?;
    ui::run(config, config_path)
}

fn parse_config_arg() -> PathBuf {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "-c" || arg == "--config" {
            if let Some(val) = args.next() {
                return val.into();
            }
        }
    }
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".gurdo/config.toml")
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_config_arg_default() {
        let p = super::parse_config_arg();
        assert!(
            p.ends_with(".gurdo/config.toml"),
            "expected path ending with .gurdo/config.toml, got {:?}",
            p
        );
    }
}
