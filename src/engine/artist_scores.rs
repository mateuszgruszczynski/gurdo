use std::io::Write as _;

use anyhow::Result;
use rusqlite::Connection;
use tracing::info;

use crate::config::Config;
use crate::db::queries;
use crate::progress::ProgressReporter;

/// Unified scoring formula (applied to ALL artists — external and similar):
///
///   playcount_base  = playcount_score × year_bonus         (0 if no play history)
///   similarity_base = similarity_score × multi_source_mod  (0 if no connections)
///   base_score      = playcount_base + similarity_base
///
///   feedback_bonus   = likes × like_bonus_flat              (flat additive)
///   feedback_penalty = max(0, 1.0 − dislikes × dislike_pct) (percentage multiplier)
///
///   final_score = (base_score + feedback_bonus) × feedback_penalty

/// Sync feedback counts and recalculate final_score for ALL artists, then write report.
/// Playcount and similarity components must already be stored by `sync-lastfm` and `expand`.
pub fn score_artists(conn: &Connection, config: &Config, progress: &dyn ProgressReporter) -> Result<()> {
    progress.stage("Recalculating scores");

    // ── 1. Sync feedback + recalculate all final_scores ─────────────────────
    queries::sync_artist_feedback_to_artists(conn)?;
    queries::recalculate_all_scores(
        conn,
        config.engine.like_bonus_flat,
        config.engine.dislike_modifier_pct,
        config.engine.multi_source_bonus_pct,
    )?;

    let artist_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM artists WHERE final_score > 0", [], |r| r.get(0),
    )?;
    info!("Recalculated final_score for {} artists", artist_count);

    // ── 2. Write report ─────────────────────────────────────────────────────
    write_report(conn, config)?;

    progress.finish(true, &format!("{} artists scored", artist_count));
    Ok(())
}

fn write_report(conn: &Connection, config: &Config) -> Result<()> {
    let cfg = &config.artist_scoring;
    let output_dir = config.output_dir();
    std::fs::create_dir_all(&output_dir)?;
    let path = output_dir.join("artist_scores.txt");

    let mut f = std::fs::File::create(&path)?;

    let now_str = chrono::Utc::now().format("%Y-%m-%d %H:%M UTC").to_string();
    writeln!(f, "=== Gurdo Artist Score Chart ===")?;
    writeln!(f, "Generated:  {}", now_str)?;
    writeln!(f, "Formula:    final = (pc_base + sim_base + likes×{}) × max(0, 1 - dislikes×{}%)",
             config.engine.like_bonus_flat, config.engine.dislike_modifier_pct * 100.0)?;
    writeln!(f, "            pc_base = playcount_score(0-100) × year_bonus(1+{}%/yr)",
             cfg.year_bonus_pct)?;
    writeln!(f, "Pool:       ≥{} total plays or liked tracks", cfg.min_playcount_threshold)?;
    writeln!(f)?;

    let ranked = queries::get_all_artists_ranked(conn)?;

    writeln!(f, "{:>5}  {:>8}  {:>6}  {:>9}  {:>5}  {:>5}  {:>5}  {}",
             "Rank", "Final", "PCScr", "Playcount", "Years", "Sim", "Likes", "Artist")?;
    writeln!(f, "{}", "-".repeat(78))?;
    for (rank, (name, _source, final_score, playcount, years, pc_score, sim_score, likes, _dislikes)) in ranked.iter().enumerate() {
        writeln!(
            f,
            "{:>5}  {:>8.2}  {:>6.1}  {:>9}  {:>5}  {:>5.1}  {:>5}  {}",
            format!("#{}", rank + 1),
            final_score,
            pc_score,
            playcount,
            years,
            sim_score,
            if *likes > 0 { format!("{}", likes) } else { String::new() },
            name,
        )?;
    }

    println!("Artist score chart written to {}", path.display());
    Ok(())
}
