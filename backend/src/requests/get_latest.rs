use rusqlite::{Connection, Error as RusqliteError};
use protocol::request::GetLatest;
use protocol::{FeedEntry, Response};
use super::Error;

pub fn get_latest(request: GetLatest, conn: &mut Connection) -> Result<Response, Error> {
    let mut stmt = conn.prepare_cached("
        SELECT rowid, feedId, title, id, updated, summary, content, isRead
        FROM feedEntries
        WHERE (:filterFeed ISNULL OR feedId == :filterFeed)
        ORDER BY updated DESC
        LIMIT :limit OFFSET :offset"
    )?;

    let result = stmt.query_map(&[&request.feed_id, &request.num_entries, &request.offset], |row| {
        FeedEntry {
            row_id: row.get(0),
            feed_id: row.get(1),
            title: row.get(2),
            id: row.get(3),
            updated: row.get(4),
            summary: row.get::<i32, Option<String>>(5).unwrap_or(String::new()),
            content: row.get::<i32, Option<String>>(6).unwrap_or(String::new()),
            // SQLite doesn't have a boolean type; it's stored as an integer
            is_read: row.get::<i32, i32>(7) == 1,
        }
    })?.collect::<Result<Vec<FeedEntry>, RusqliteError>>()?;

    Ok(Response::FeedEntries {
        list: result
    })
}
