use rusqlite::{Connection, Error as RusqliteError};
use protocol::request::GetFeedList;
use protocol::{FeedDesc, Response};
use super::Error;

pub fn get_feeds(_request: GetFeedList, conn: &mut Connection) -> Result<Response, Error> {
    let mut stmt = conn.prepare("SELECT id, lastUpdate, title, url FROM feeds ORDER BY id DESC")?;

    let result = stmt.query_map(&[], |row| {
        FeedDesc {
            id: row.get(0),
            last_update: row.get::<i32, Option<i64>>(1).unwrap_or(0),
            title: row.get::<i32, Option<String>>(2).unwrap_or(String::new()),
            url: row.get(3),
        }
    })?.collect::<Result<Vec<FeedDesc>, RusqliteError>>()?;

    Ok(Response::FeedList {
        list: result
    })
}
