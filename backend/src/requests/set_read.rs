use rusqlite::{Connection};
use protocol::request::SetRead;
use protocol::{Response};
use super::Error;

pub fn set_read(request: SetRead, conn: &mut Connection) -> Result<Response, Error> {
	let mut stmt = conn.prepare_cached("
		UPDATE feedEntries SET isRead = ? WHERE rowid = ?;
	")?;

	let read_int = if request.is_read {
		1
	}
	else {
		0
	};

	for entry_id in request.entry_ids {
		stmt.execute(&[&entry_id, &read_int])?;
	}

	Ok(Response::Success)
}