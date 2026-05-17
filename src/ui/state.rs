// ── Operations state ──────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub enum OperationKind {
    SyncLastfm,
    Expand,
    FetchTracks,
    Score,
    SpotifyLogin,
}

impl OperationKind {
    pub fn label(&self) -> &'static str {
        match self {
            OperationKind::SyncLastfm    => "Sync Last.fm",
            OperationKind::Expand        => "Expand similar artists",
            OperationKind::FetchTracks   => "Fetch top tracks",
            OperationKind::Score         => "Recalculate scores",
            OperationKind::SpotifyLogin  => "Spotify login",
        }
    }
}

#[derive(Clone)]
pub struct ActiveOperation {
    pub kind:    OperationKind,
    pub step:    Option<(u8, u8)>,   // (current, total) for multi-step sequences
    pub stage:   String,
    pub current: u64,
    pub total:   Option<u64>,
    #[allow(dead_code)]
    pub message: String,
}

#[derive(Clone)]
pub enum OperationResult {
    Ok(String),
    Failed(String),
}

#[derive(Clone, Default)]
pub struct OperationsState {
    pub active:      Option<ActiveOperation>,
    pub last_result: Option<OperationResult>,
}

pub enum OperationCommand {
    Run(OperationKind),
    UpdateAll,
}

// ── Player state ──────────────────────────────────────────────────────────────

#[derive(Clone, Default)]
pub struct PlayerState {
    pub is_playing:              bool,
    pub track_name:              String,
    pub artist_name:             String,
    pub album_name:              String,
    pub album_art_url:           Option<String>,
    pub album_art_bytes:         Option<Vec<u8>>,
    pub track_id:                Option<String>,
    pub progress_ms:             u64,
    pub duration_ms:             u64,
    pub feedback:                Option<bool>,
    pub track_uri:               Option<String>,
    pub error:                   Option<String>,
    pub api_error_snooze_until:  Option<std::time::Instant>,
}

pub enum PlayerCommand {
    PlayPause,
    Next,
    Previous,
    SeekRelative(i64), // offset in milliseconds, can be negative
    SaveTrack,
    UnlikeTrack,
    RemoveTrack,
    StartQueue,
}
