#[derive(Deserialize, Debug)]
pub struct GetLatest;

#[derive(Deserialize, Debug)]
pub struct GetFeedList;

#[derive(Deserialize, Debug)]
pub struct AddFeed {
    pub url: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Request {
    GetLatest(GetLatest),
    GetFeedList(GetFeedList),
    AddFeed(AddFeed),
}

#[derive(Deserialize, Debug)]
pub struct RequestFrame {
    pub id: u64,
    pub body: Request,
}
