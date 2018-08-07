use protocol::types::{FeedDesc, FeedEntry};

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum Response {
    Success,
    Error {
        error: String,
    },
    FeedEntries {
        list: Vec<FeedEntry>,
    },
    FeedList {
        list: Vec<FeedDesc>,
    },
}

#[derive(Serialize, Debug)]
pub struct ResponseFrame {
    pub id: u64,
    pub body: Response,
}
