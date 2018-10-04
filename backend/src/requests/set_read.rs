use rusqlite::{Connection};
use protocol::request::SetRead;
use protocol::{Response};
use super::Error;

pub fn set_read(request: SetRead, conn: &mut Connection) -> Result<Response, Error> {
	let tx = conn.transaction()?;

	let read_int = if request.is_read {
		1
	}
	else {
		0
	};

	{
		let mut stmt = tx.prepare_cached("UPDATE feedEntries SET isRead = ? WHERE rowid = ?;")?;

		for entry_id in request.entry_ids {
			stmt.execute(&[&entry_id, &read_int])?;
		}
	}

	tx.commit()?;

	Ok(Response::Success)
}