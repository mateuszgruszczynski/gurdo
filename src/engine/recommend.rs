use anyhow::Result;
use rand::Rng;
use rusqlite::Connection;
use tracing::info;

use crate::config::Config;
use crate::db::queries;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;
    use crate::db::schema::init_db;

    fn open_mem() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();
        conn
    }

    // ── weighted_sample ──────────────────────────────────────────────────────

    #[test]
    fn weighted_sample_deterministic() {
        let weights = vec![1.0_f64, 3.0, 1.0];
        let mut rng = StdRng::seed_from_u64(42);
        // Run 100 samples; middle weight should win ~60% of the time.
        let counts: Vec<usize> = (0..100).map(|_| weighted_sample(&weights, &mut rng)).collect();
        // With seed 42 the sequence is fixed — verify first value and overall distribution.
        assert_eq!(counts[0], 1, "first sample with seed 42 should pick index 1");
        let mid = counts.iter().filter(|&&i| i == 1).count();
        assert!(mid > 40, "middle weight (3×) should dominate: got {}/100", mid);
    }

    #[test]
    fn weighted_sample_single_weight_always_zero() {
        let weights = vec![5.0_f64];
        let mut rng = StdRng::seed_from_u64(0);
        for _ in 0..10 {
            assert_eq!(weighted_sample(&weights, &mut rng), 0);
        }
    }

    #[test]
    fn weighted_sample_equal_weights_all_valid() {
        let weights = vec![1.0_f64; 5];
        let mut rng = StdRng::seed_from_u64(7);
        for _ in 0..50 {
            let idx = weighted_sample(&weights, &mut rng);
            assert!(idx < 5, "index out of range: {}", idx);
        }
    }

    // ── generate_recommendations ─────────────────────────────────────────────

    fn minimal_config() -> Config {
        Config::load(std::path::Path::new("config.toml")).unwrap()
    }

    #[test]
    fn generate_recommendations_returns_results_with_scores() {
        let conn = open_mem();

        // Seed two artists with tracks and a positive final_score.
        for (artist, score) in [("alpha", 10.0_f64), ("beta", 5.0)] {
            queries::upsert_artist_external(&conn, artist, 100, 5, score, 1.0, 0).unwrap();
            conn.execute(
                "UPDATE artists SET final_score = ?1 WHERE artist_name = ?2",
                rusqlite::params![score, artist],
            ).unwrap();
            for i in 1u32..=3 {
                queries::upsert_artist_top_track(
                    &conn, artist, &format!("track{}", i), "", 1000 / i as u64, 500, i, 0,
                ).unwrap();
            }
        }

        let mut cfg = minimal_config();
        cfg.recommendations.count = 5;

        let results = generate_recommendations(&conn, &cfg).unwrap();
        assert!(!results.is_empty(), "expected results from seeded DB");
        for (_artist, _track, score) in &results {
            assert!(*score > 0.0, "every result should have a positive artist score");
        }
    }
}

/// Pick a random index from a slice of weights using weighted random sampling.
fn weighted_sample(weights: &[f64], rng: &mut impl Rng) -> usize {
    let total: f64 = weights.iter().sum();
    let mut r = rng.r#gen::<f64>() * total;
    for (i, w) in weights.iter().enumerate() {
        r -= w;
        if r <= 0.0 {
            return i;
        }
    }
    weights.len() - 1
}

/// Generate a list of (artist, track) pairs using weighted random sampling.
/// Artist probability ∝ score^artist_score_exponent.
/// Track probability ∝ 1/rank^track_rank_exponent.
pub fn generate_recommendations(conn: &Connection, config: &Config) -> Result<Vec<(String, String, f64)>> {
    let cfg = &config.recommendations;

    // ── Load artist pool ──────────────────────────────────────────────────────
    let t0 = std::time::Instant::now();
    let artists = queries::get_scoreable_artists_with_tracks(conn)?;
    info!("get_scoreable_artists_with_tracks: {}ms ({} artists)", t0.elapsed().as_millis(), artists.len());

    if artists.is_empty() {
        info!("No artists with tracks found — run `gurdo fetch-tracks` first");
        return Ok(vec![]);
    }

    // Load disliked tracks to exclude from recommendations.
    let t2 = std::time::Instant::now();
    let disliked = queries::get_disliked_tracks(conn)?;
    info!("feedback loaded: {}ms ({} disliked tracks)", t2.elapsed().as_millis(), disliked.len());

    // final_score already includes feedback adjustments, so use it directly.
    let artist_weights: Vec<f64> = artists
        .iter()
        .map(|(_, score)| score.powf(cfg.artist_score_exponent))
        .collect();

    info!(
        "Artist pool: {} artists, score_exponent={}, track_rank_exponent={}",
        artists.len(), cfg.artist_score_exponent, cfg.track_rank_exponent
    );

    // ── Load tracks per artist (single query) ────────────────────────────────
    let t1 = std::time::Instant::now();
    let track_cache = queries::get_all_artist_top_tracks(conn)?;
    info!("get_all_artist_top_tracks: {}ms ({} artists)", t1.elapsed().as_millis(), track_cache.len());

    // ── Sample N tracks ───────────────────────────────────────────────────────
    let mut rng = rand::thread_rng();
    let mut results: Vec<(String, String, f64)> = Vec::with_capacity(cfg.count);
    let max_attempts = cfg.count * 10;

    let mut attempts = 0usize;
    while results.len() < cfg.count && attempts < max_attempts {
        attempts += 1;

        let artist_idx = weighted_sample(&artist_weights, &mut rng);
        let (artist_name, _) = &artists[artist_idx];

        let tracks = match track_cache.get(artist_name) {
            Some(t) if !t.is_empty() => t,
            _ => continue,
        };

        let track_weights: Vec<f64> = tracks
            .iter()
            .map(|(_, rank)| 1.0 / (*rank as f64).max(1.0).powf(cfg.track_rank_exponent))
            .collect();

        let track_idx = weighted_sample(&track_weights, &mut rng);
        let (track_name, _) = &tracks[track_idx];

        // Skip disliked tracks.
        let key = (artist_name.to_lowercase(), track_name.to_lowercase());
        if disliked.contains(&key) {
            continue;
        }

        results.push((artist_name.clone(), track_name.clone(), artists[artist_idx].1));
    }

    Ok(results)
}
