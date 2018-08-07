use rusqlite::{Connection};
use protocol::request::AddFeed;
use protocol::Response;
use super::Error;

pub fn add_feed(request: AddFeed, conn: &mut Connection) -> Result<Response, Error> {
    let mut stmt = conn.prepare_cached("INSERT INTO feeds(url) VALUES ( ? )")?;

    stmt.execute(&[ &request.url ])?;

    Ok(Response::Success)
}
