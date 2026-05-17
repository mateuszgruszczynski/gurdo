use serde::Deserialize;

// ── Shared primitives ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct ArtistRef {
    pub name: String,
}

// ── user.getLovedTracks ──────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct LovedTracksResponse {
    pub lovedtracks: LovedTracks,
}

#[derive(Debug, Deserialize)]
pub struct LovedTracks {
    #[serde(rename = "track")]
    pub tracks: Vec<LovedTrack>,
    #[serde(rename = "@attr")]
    pub attr: PageAttr,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LovedTrack {
    pub name: String,
    #[serde(default)]
    pub mbid: String,
    pub artist: ArtistRef,
    pub date: Option<ScrobbleDate>,
}

// ── user.getTopTags ──────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct TopTagsResponse {
    pub toptags: TopTags,
}

#[derive(Debug, Deserialize)]
pub struct TopTags {
    #[serde(rename = "tag")]
    pub tags: Vec<TopTag>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TopTag {
    pub name: String,
    pub count: u32,
}

// ── artist.getSimilar ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SimilarArtistsResponse {
    pub similarartists: SimilarArtists,
}

#[derive(Debug, Deserialize)]
pub struct SimilarArtists {
    #[serde(rename = "artist")]
    pub artists: Vec<SimilarArtist>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SimilarArtist {
    pub name: String,
    pub r#match: String,
}

impl SimilarArtist {
    pub fn match_score(&self) -> f64 {
        self.r#match.parse().unwrap_or(0.0)
    }
}

// ── artist.getTopTracks ──────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ArtistTopTracksResponse {
    pub toptracks: ArtistTopTracks,
}

#[derive(Debug, Deserialize)]
pub struct ArtistTopTracks {
    #[serde(rename = "track")]
    pub tracks: Vec<ArtistTopTrack>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ArtistTopTrack {
    pub name: String,
    #[serde(default)]
    pub mbid: String,
    pub playcount: String,
    pub listeners: String,
    pub artist: ArtistRef,
    #[serde(rename = "@attr")]
    pub attr: Option<RankAttr>,
}

impl ArtistTopTrack {
    pub fn rank(&self) -> u32 {
        self.attr.as_ref().map(|a| a.rank.parse().unwrap_or(0)).unwrap_or(0)
    }
}

// ── user.getWeeklyChartList ──────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct WeeklyChartListResponse {
    pub weeklychartlist: WeeklyChartList,
}

#[derive(Debug, Deserialize)]
pub struct WeeklyChartList {
    #[serde(rename = "chart")]
    pub charts: Vec<WeeklyChartEntry>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WeeklyChartEntry {
    pub from: String,
}

impl WeeklyChartEntry {
    pub fn from_ts(&self) -> i64 { self.from.parse().unwrap_or(0) }
}


// ── user.getWeeklyArtistChart ────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct WeeklyArtistChartResponse {
    pub weeklyartistchart: WeeklyArtistChart,
}

#[derive(Debug, Deserialize)]
pub struct WeeklyArtistChart {
    #[serde(rename = "artist", default)]
    pub artists: Vec<WeeklyArtistEntry>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WeeklyArtistEntry {
    pub name: String,
    #[serde(default)]
    pub mbid: String,
    pub playcount: String,
}

impl WeeklyArtistEntry {
    pub fn playcount_u64(&self) -> u64 {
        self.playcount.parse().unwrap_or(0)
    }
}

// ── Shared attr structs ──────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct RankAttr {
    pub rank: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PageAttr {
    #[serde(rename = "totalPages")]
    pub total_pages: String,
}

impl PageAttr {
    pub fn total_pages_u32(&self) -> u32 {
        self.total_pages.parse().unwrap_or(1)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ScrobbleDate {
    pub uts: String,
}

impl ScrobbleDate {
    pub fn timestamp(&self) -> i64 {
        self.uts.parse().unwrap_or(0)
    }
}
