#[derive(Hash, Eq)]
pub struct Song {
    pub time_str: String,
    pub title: String,
    pub interprets: String,
}

impl std::cmp::PartialEq for Song {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
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

