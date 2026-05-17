mod config;
mod db;
mod engine;
mod lastfm;
mod progress;
mod spotify;
mod sync;
mod ui;

use anyhow::Result;
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

    let _ = rustls::crypto::ring::default_provider().install_default();

    let config_path = parse_config_arg();
    let config = config::Config::load(&config_path)?;
    std::fs::create_dir_all(config.data_dir())?;
    ui::run(config, config_path)
}

fn parse_config_arg() -> std::path::PathBuf {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "-c" || arg == "--config" {
            if let Some(val) = args.next() {
                return val.into();
            }
        }
    }
    "config.toml".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config_arg_default() {
        let p = super::parse_config_arg();
        assert_eq!(p.to_str().unwrap(), "config.toml");
    }
}
