#[allow(dead_code)]
pub struct KnobSpec {
    pub label:       &'static str,
    pub description: &'static str,
}

#[allow(dead_code)]
pub static SYNC_KNOBS: &[KnobSpec] = &[
    KnobSpec { label: "Loved tracks limit",  description: "Maximum number of loved tracks to fetch from Last.fm per sync." },
    KnobSpec { label: "Seed artists limit",  description: "Maximum number of seed artists to include in the scoring pool." },
    KnobSpec { label: "Seed tracks limit",   description: "Maximum number of seed tracks used when building recommendations." },
];

#[allow(dead_code)]
pub static ENGINE_KNOBS: &[KnobSpec] = &[
    KnobSpec { label: "Similar artists per seed",    description: "How many similar artists to fetch from Last.fm per seed artist." },
    KnobSpec { label: "Tracks per artist",           description: "Top tracks to fetch per artist from Last.fm." },
    KnobSpec { label: "Recommendation pool size",    description: "Candidate track pool size before final weighted sampling." },
    KnobSpec { label: "Max tracks per seed artist",  description: "Caps how many tracks one artist can contribute to the final queue." },
    KnobSpec { label: "Similarity multiplier",       description: "Fraction of a seed artist's score that similar artists inherit." },
    KnobSpec { label: "Multi-source bonus",          description: "Extra score fraction per additional seed artist that lists a similar artist." },
    KnobSpec { label: "Like bonus (flat)",           description: "Flat score added per liked track from this artist." },
    KnobSpec { label: "Dislike penalty (%)",         description: "Score multiplier reduction per disliked track." },
];

#[allow(dead_code)]
pub static ARTIST_SCORING_KNOBS: &[KnobSpec] = &[
    KnobSpec { label: "Playcount score exponent",   description: "Power law exponent for raw playcount → score." },
    KnobSpec { label: "Year active bonus (%)",      description: "Percentage added to score per year in listening history." },
    KnobSpec { label: "Min playcount threshold",    description: "Artists below this threshold are excluded from scoring unless liked." },
];

#[allow(dead_code)]
pub static RECOMMEND_KNOBS: &[KnobSpec] = &[
    KnobSpec { label: "Number of recommendations",  description: "How many tracks the recommendation engine generates per run." },
    KnobSpec { label: "Artist score exponent",      description: "Exponent applied to artist score before weighted sampling." },
    KnobSpec { label: "Track rank exponent",        description: "Exponent on track rank weight (1/rank^e)." },
];
