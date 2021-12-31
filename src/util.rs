use itertools::Itertools;
use chrono::{Utc, Duration};
use crate::scraper::WDR;

#[derive(Hash, Eq, Clone)]
pub struct Song {
    pub title: String,
    pub interprets: String,
    pub radio: WDR
}

pub struct Track {
    pub title: String,
    pub track_id: String,
}

impl std::cmp::PartialEq for Song {
    fn eq(&self, other: &Self) -> bool {
        self.title.eq(&other.title)
    }
}

impl std::fmt::Debug for Song {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} by {}", self.title, self.interprets)
    }
}

pub struct SearchParams {
    pub date: String,
    pub hour: u8,
}

pub fn distinct_playlist(playlist: Vec<Song>) -> Vec<Song> {
    playlist.into_iter().unique().collect()
}

pub fn yesterday() -> String {
    let yesterday = Utc::now() - Duration::days(1);
    yesterday.format("%Y-%m-%d").to_string()
}