#[derive(Serialize, Debug)]
pub struct FeedDesc {
    pub id: i64,
    pub last_update: i64,
    pub title: String,
    pub url: String,
}

#[derive(Serialize, Debug)]
pub struct FeedEntry {
    pub row_id: i64,
    pub feed_id: i64,
    pub title: String,
    pub id: String,
    pub updated: i64,
    pub summary: String,
    pub content: String,
}
