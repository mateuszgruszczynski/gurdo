use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

// ── Secrets overlay ───────────────────────────────────────────────────────────

#[derive(Debug, Default, Deserialize)]
struct SecretsConfig {
    #[serde(default)]
    lastfm: SecretsLastfm,
}

#[derive(Debug, Default, Deserialize)]
struct SecretsLastfm {
    username: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub lastfm: LastfmConfig,
    #[serde(default)]
    pub spotify: SpotifyConfig,
    pub app: AppConfig,
    pub sync: SyncConfig,
    pub engine: EngineConfig,
    #[serde(default)]
    pub artist_scoring: ArtistScoringConfig,
    #[serde(default)]
    pub recommendations: RecommendConfig,
    #[serde(default)]
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LastfmConfig {
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpotifyConfig {
    #[serde(default = "default_redirect_uri")]
    pub redirect_uri: String,
    #[serde(default = "default_callback_port")]
    pub callback_port: u16,
}

impl Default for SpotifyConfig {
    fn default() -> Self {
        Self {
            redirect_uri: default_redirect_uri(),
            callback_port: default_callback_port(),
        }
    }
}

fn default_redirect_uri() -> String {
    "http://127.0.0.1:8888/callback".to_string()
}
fn default_callback_port() -> u16 { 8888 }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    #[serde(default = "default_data_dir")]
    pub data_dir: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UiConfig {
    /// Background color as [R, G, B] (0–255). Used as a fixed panel fill color.
    #[serde(default = "default_bg_color")]
    pub background_color: [u8; 3],
    #[serde(default = "default_player_window_size")]
    pub player_window_size: [u32; 2],
    #[serde(default = "default_settings_window_size")]
    pub settings_window_size: [u32; 2],
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            background_color: default_bg_color(),
            player_window_size: default_player_window_size(),
            settings_window_size: default_settings_window_size(),
        }
    }
}

fn default_bg_color() -> [u8; 3] { [27, 27, 27] }
fn default_player_window_size() -> [u32; 2] { [440, 660] }
fn default_settings_window_size() -> [u32; 2] { [800, 900] }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyncConfig {
    #[serde(default = "default_seed_artists_limit")]
    pub seed_artists_limit: u32,
    #[serde(default = "default_seed_tracks_limit")]
    pub seed_tracks_limit: u32,
    #[serde(default = "default_loved_tracks_limit")]
    pub loved_tracks_limit: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArtistScoringConfig {
    /// Scoring exponent: base_score = total_playcount ^ score_exponent.
    /// Default log10(2) ≈ 0.301 means 1000 plays scores 2× more than 100 plays.
    #[serde(default = "default_score_exponent")]
    pub score_exponent: f64,
    /// Per-year bonus: final_score = base_score × (1 + year_bonus_pct/100) ^ years_active.
    /// Default 5.0 means each year the artist appears adds 5% to their final score.
    #[serde(default = "default_year_bonus_pct")]
    pub year_bonus_pct: f64,
    /// Artists with total all-time playcount >= this value are included in the scoring pool.
    #[serde(default = "default_min_playcount_threshold")]
    pub min_playcount_threshold: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecommendConfig {
    /// How many tracks to generate.
    #[serde(default = "default_recommend_count")]
    pub count: usize,
    /// Exponent applied to artist score before sampling.
    /// 1.0 = proportional to score, >1.0 = more top-heavy, <1.0 = flatter distribution.
    #[serde(default = "default_artist_score_exponent")]
    pub artist_score_exponent: f64,
    /// Exponent applied to track rank: weight = 1/rank^exponent.
    /// 1.0 = inverse rank, >1.0 = strongly prefer top tracks, <1.0 = flatter.
    #[serde(default = "default_track_rank_exponent")]
    pub track_rank_exponent: f64,
}

fn default_recommend_count() -> usize { 50 }
fn default_artist_score_exponent() -> f64 { 1.0 }
fn default_track_rank_exponent() -> f64 { 1.0 }

impl Default for RecommendConfig {
    fn default() -> Self {
        Self {
            count: default_recommend_count(),
            artist_score_exponent: default_artist_score_exponent(),
            track_rank_exponent: default_track_rank_exponent(),
        }
    }
}

fn default_score_exponent() -> f64 { 0.301 }   // log10(2): 1000 plays = 2× 100 plays
fn default_year_bonus_pct() -> f64 { 5.0 }
fn default_min_playcount_threshold() -> u64 { 40 }

impl Default for ArtistScoringConfig {
    fn default() -> Self {
        Self {
            score_exponent: default_score_exponent(),
            year_bonus_pct: default_year_bonus_pct(),
            min_playcount_threshold: default_min_playcount_threshold(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EngineConfig {
    /// Multiplier applied to parent artist's score when scoring similar artists.
    /// 0.5 means similar artists get at most 50% of their best parent's score.
    #[serde(default = "default_similarity_multiplier")]
    pub similarity_multiplier: f64,
    /// Bonus per additional parent artist that lists a similar artist.
    /// 0.05 = 5% extra per additional source.
    #[serde(default = "default_multi_source_bonus_pct")]
    pub multi_source_bonus_pct: f64,
    /// Flat score bonus per loved track (from Last.fm).
    #[serde(default = "default_like_bonus_flat")]
    pub like_bonus_flat: f64,
    /// Per-liked-track modifier: each like adds this fraction to the artist weight.
    /// 0.05 = each like adds 5%.
    #[serde(default = "default_like_modifier_pct")]
    pub like_modifier_pct: f64,
    /// Per-disliked-track penalty: each dislike subtracts this fraction from the artist weight.
    /// 0.10 = each dislike removes 10%.
    #[serde(default = "default_dislike_modifier_pct")]
    pub dislike_modifier_pct: f64,
    #[serde(default = "default_similar_artists_limit")]
    pub similar_artists_limit: u32,
    #[serde(default = "default_artist_top_tracks_limit")]
    pub artist_top_tracks_limit: u32,
    #[serde(default = "default_tag_top_tracks_limit")]
    pub tag_top_tracks_limit: u32,
    #[serde(default = "default_recommendation_pool_size")]
    pub recommendation_pool_size: u32,
    /// Max tracks allowed from a single seed artist in the final pool.
    /// Prevents one heavily-played artist from dominating recommendations.
    #[serde(default = "default_max_tracks_per_seed")]
    pub max_tracks_per_seed: u32,
}

fn default_data_dir() -> String {
    "~/.gurdo".to_string()
}
fn default_seed_artists_limit() -> u32 { 50 }
fn default_seed_tracks_limit() -> u32 { 50 }
fn default_loved_tracks_limit() -> u32 { 500 }
fn default_similarity_multiplier() -> f64 { 0.5 }
fn default_multi_source_bonus_pct() -> f64 { 0.05 }
fn default_like_bonus_flat() -> f64 { 5.0 }
fn default_like_modifier_pct() -> f64 { 0.05 }
fn default_dislike_modifier_pct() -> f64 { 0.10 }
fn default_similar_artists_limit() -> u32 { 20 }
fn default_artist_top_tracks_limit() -> u32 { 10 }
fn default_tag_top_tracks_limit() -> u32 { 30 }
fn default_recommendation_pool_size() -> u32 { 200 }
fn default_max_tracks_per_seed() -> u32 { 20 }

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            seed_artists_limit:  default_seed_artists_limit(),
            seed_tracks_limit:   default_seed_tracks_limit(),
            loved_tracks_limit:  default_loved_tracks_limit(),
        }
    }
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            similarity_multiplier:    default_similarity_multiplier(),
            multi_source_bonus_pct:   default_multi_source_bonus_pct(),
            like_bonus_flat:          default_like_bonus_flat(),
            like_modifier_pct:        default_like_modifier_pct(),
            dislike_modifier_pct:     default_dislike_modifier_pct(),
            similar_artists_limit:    default_similar_artists_limit(),
            artist_top_tracks_limit:  default_artist_top_tracks_limit(),
            tag_top_tracks_limit:     default_tag_top_tracks_limit(),
            recommendation_pool_size: default_recommendation_pool_size(),
            max_tracks_per_seed:      default_max_tracks_per_seed(),
        }
    }
}

impl Config {
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        std::fs::write(path, content)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self> {
        Self::load_inner(path, &Self::secrets_path(path))
    }

    fn load_inner(config_path: &Path, secrets_path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(config_path)
            .with_context(|| format!("Cannot read config file: {}", config_path.display()))?;
        let mut config: Config = toml::from_str(&content)
            .with_context(|| format!("Invalid config file: {}", config_path.display()))?;
        if secrets_path.exists() {
            let sc = Self::load_secrets(secrets_path)?;
            if let Some(u) = sc.lastfm.username { config.lastfm.username = u; }
        }
        Ok(config)
    }

    /// Test-only: load config with an explicitly specified secrets path.
    #[cfg(test)]
    pub(crate) fn load_with_secrets_at(config_path: &Path, secrets_path: &Path) -> Result<Self> {
        Self::load_inner(config_path, secrets_path)
    }

    /// Always returns `~/.gurdo/secrets.toml` regardless of `config_path`.
    pub fn secrets_path(_config_path: &Path) -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".gurdo/secrets.toml")
    }

    fn load_secrets(path: &Path) -> Result<SecretsConfig> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Cannot read secrets file: {}", path.display()))?;
        toml::from_str(&content)
            .with_context(|| format!("Invalid secrets file: {}", path.display()))
    }

    pub fn data_dir(&self) -> PathBuf {
        let raw = &self.app.data_dir;
        if raw.starts_with("~/") {
            if let Some(home) = dirs_home() {
                return home.join(&raw[2..]);
            }
        }
        PathBuf::from(raw)
    }

    pub fn db_path(&self) -> PathBuf {
        self.data_dir().join("gurdo.db")
    }

    pub fn output_dir(&self) -> PathBuf {
        self.data_dir().join("recommendations")
    }

    pub fn token_path(&self) -> PathBuf {
        self.data_dir().join("spotify_token.json")
    }
}

fn dirs_home() -> Option<PathBuf> {
    dirs::home_dir()
}

/// Returns the canonical `~/.gurdo/` directory path, or `None` if the home
/// directory cannot be determined.
pub fn gurdo_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".gurdo"))
}

/// Returns `true` when the user needs to complete first-run setup.
///
/// Setup is required when `secrets_path` is absent, unparseable, or missing
/// `api_key` or `username` after trim.
pub fn needs_setup(secrets_path: &Path) -> bool {
    let content = match std::fs::read_to_string(secrets_path) {
        Ok(c) => c,
        Err(_) => return true,
    };
    let sc: SecretsConfig = match toml::from_str(&content) {
        Ok(c) => c,
        Err(_) => return true,
    };
    let user_ok = sc.lastfm.username.as_deref().map(str::trim).unwrap_or("").len() > 0;
    !user_ok
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn minimal_config_toml() -> String {
        r#"
[lastfm]
username = "PLACEHOLDER_USER"

[spotify]
redirect_uri = "http://127.0.0.1:8888/callback"
callback_port = 8888

[app]
data_dir = "/tmp/gurdo-test"

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
"#.to_string()
    }

    #[test]
    fn secrets_path_always_returns_gurdo_path() {
        let path_a = Config::secrets_path(&PathBuf::from("/some/dir/config.toml"));
        let path_b = Config::secrets_path(&PathBuf::from("/tmp/custom/my.toml"));
        assert!(path_a.ends_with(".gurdo/secrets.toml"),
            "expected path ending with .gurdo/secrets.toml, got {:?}", path_a);
        assert_eq!(path_a, path_b, "secrets_path must be identical for any config_path input");
    }

    #[test]
    fn load_overlays_secrets_when_present() {
        let dir = tempfile::tempdir().unwrap();
        let cfg_path = dir.path().join("config.toml");
        let sec_path = dir.path().join("secrets.toml");

        fs::write(&cfg_path, minimal_config_toml()).unwrap();
        fs::write(&sec_path, "[lastfm]\nusername = \"real_user\"\n").unwrap();

        let config = Config::load_with_secrets_at(&cfg_path, &sec_path).unwrap();
        assert_eq!(config.lastfm.username, "real_user");
    }

    #[test]
    fn load_uses_config_values_when_secrets_absent() {
        let dir = tempfile::tempdir().unwrap();
        let cfg_path = dir.path().join("config.toml");
        let no_secrets = dir.path().join("nonexistent_secrets.toml");

        fs::write(&cfg_path, r#"
[lastfm]
username = "direct_user"

[spotify]
redirect_uri  = "http://127.0.0.1:8888/callback"
callback_port = 8888

[app]
data_dir = "/tmp/gurdo-test"

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
"#).unwrap();

        let config = Config::load_with_secrets_at(&cfg_path, &no_secrets).unwrap();
        assert_eq!(config.lastfm.username, "direct_user");
    }

    // ── needs_setup ────────────────────────────────────────────────────────────

    #[test]
    fn needs_setup_true_when_file_absent() {
        assert!(needs_setup(std::path::Path::new("/nonexistent/path/secrets.toml")));
    }

    #[test]
    fn needs_setup_false_when_all_keys_present() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("secrets.toml");
        fs::write(&p, "[lastfm]\nusername = \"u\"\n").unwrap();
        assert!(!needs_setup(&p));
    }

    #[test]
    fn needs_setup_true_when_username_empty() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("secrets.toml");
        fs::write(&p, "[lastfm]\nusername = \"\"\n").unwrap();
        assert!(needs_setup(&p));
    }

    #[test]
    fn needs_setup_true_when_file_unparseable() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("secrets.toml");
        fs::write(&p, "NOT VALID TOML {{{").unwrap();
        assert!(needs_setup(&p));
    }

}
