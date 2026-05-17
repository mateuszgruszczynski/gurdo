# Gurdo

A desktop music player and recommendation engine powered by Last.fm and Spotify.

## Running

```sh
cargo run --release
```

On first launch a setup wizard will appear and collect your credentials. It writes:

- `~/.gurdo/secrets.toml` — Last.fm API key, username, Spotify client ID (chmod 600)
- `~/.gurdo/config.toml` — default configuration (edit to customise)

Subsequent launches skip the wizard and open the player directly.

To use a custom config location:

```sh
cargo run --release -- -c /path/to/config.toml
```

`~/.gurdo/secrets.toml` is always used for credentials regardless of the `-c` flag.

## Configuration

Edit `~/.gurdo/config.toml` after first run. For reference see `config.toml.example`.

Sensitive values (`api_key`, `username`, `client_id`) live in `~/.gurdo/secrets.toml`
and are overlaid on top of `config.toml` at startup.

## Data

The database and output files are stored in the directory set by `[app].data_dir`
(default: `~/.gurdo/`).
