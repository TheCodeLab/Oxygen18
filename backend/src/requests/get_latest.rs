use rusqlite::{Connection, Error as RusqliteError};
use protocol::request::GetLatest;
use protocol::{FeedEntry, Response};
use super::Error;

pub fn get_latest(request: GetLatest, conn: &mut Connection) -> Result<Response, Error> {
    let mut stmt = conn.prepare_cached("
        SELECT feedId, title, id, updated, summary, content
        FROM feedEntries
        WHERE (:filterFeed ISNULL OR feedId == :filterFeed)
        ORDER BY updated DESC
        LIMIT :limit OFFSET :offset"
    )?;

    let result = stmt.query_map(&[&request.feed_id, &request.num_entries, &request.offset], |row| {
        FeedEntry {
            feed_id: row.get(0),
            title: row.get(1),
            id: row.get(2),
            updated: row.get(3),
            summary: row.get::<i32, Option<String>>(4).unwrap_or(String::new()),
            content: row.get::<i32, Option<String>>(5).unwrap_or(String::new()),
        }
    })?.collect::<Result<Vec<FeedEntry>, RusqliteError>>()?;

    Ok(Response::FeedEntries {
        list: result
    })
}
