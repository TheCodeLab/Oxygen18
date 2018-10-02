use rusqlite::{Connection};
use protocol::request::MarkAsRead;
use protocol::{Response};
use super::Error;

pub fn mark_read(request: MarkAsRead, conn: &mut Connection) -> Result<Response, Error> {
	conn.execute("
		UPDATE feedEntries SET isRead = 1 WHERE rowid = ?;
	", &[&request.entry_id])?;

	Ok(Response::Success)
}