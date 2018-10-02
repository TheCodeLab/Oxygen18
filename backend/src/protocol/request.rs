#[derive(Deserialize, Debug)]
pub struct GetLatest {
    pub feed_id: Option<i64>,
    #[serde(default)]
    pub offset: i32,
    pub num_entries: i32,
}

#[derive(Deserialize, Debug)]
pub struct GetFeedList;

#[derive(Deserialize, Debug)]
pub struct AddFeed {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct MarkAsRead {
    pub entry_id: i64,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Request {
    GetLatest(GetLatest),
    GetFeedList(GetFeedList),
    AddFeed(AddFeed),
    MarkAsRead(MarkAsRead),
}

#[derive(Deserialize, Debug)]
pub struct RequestFrame {
    pub id: u64,
    pub body: Request,
}
