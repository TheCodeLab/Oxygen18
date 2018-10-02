// Database migration system.

use rusqlite::{Connection, Transaction, Error as RusqliteError};

fn get_schema_version(conn: &Connection) -> Result<i64, RusqliteError> {
	conn.query_row("PRAGMA user_version;", &[], |row| row.get(0))
}

fn migrate_1(tx: &Transaction) -> Result<(), RusqliteError> {
	tx.execute("ALTER TABLE feedEntries ADD COLUMN isRead INTEGER DEFAULT 0;", &[])?;
	Ok(())
}

pub fn migrate_db(conn: &mut Connection) -> Result<(), RusqliteError> {
	// hack?: declare migrations here, not in a lazy_static block
	// lazy_static requires that all its values be Sync
	// Fn types cannot be shared between threads safely.
	let migrations: Vec<&Fn(&Transaction) -> Result<(), RusqliteError>> = vec![
		&migrate_1,
	];

	let schema_version = get_schema_version(conn)? as usize;
	let current_version = migrations.len();

	for migration_version in (schema_version + 1)..=current_version {
		let migration = migrations[migration_version - 1];
		let tx = conn.transaction()?;
		migration(&tx)?;
		tx.execute(&format!("PRAGMA user_version = {};", migration_version), &[])?;
		tx.commit()?;
		println!("migrated database to schema version {}", migration_version);
	}

	Ok(())
}

#[cfg(test)]
mod tests {
	use rusqlite::Connection;
	use super::*;
	use std::collections::HashSet;

	#[test]
	fn test_migrate() {
		let mut conn = Connection::open_in_memory().unwrap();
		conn.execute_batch(
			"BEGIN;
			CREATE TABLE IF NOT EXISTS feeds (
				id INTEGER PRIMARY KEY,
				url TEXT NOT NULL,
				lastUpdate INTEGER,
				title TEXT
			);
			CREATE TABLE IF NOT EXISTS feedEntries (
				feedId INTEGER,
				title TEXT,
				id TEXT,
				updated INTEGER NOT NULL,
				summary TEXT,
				content TEXT,

				FOREIGN KEY(feedId) REFERENCES feeds(id),
				PRIMARY KEY(feedId, id)
			);
			CREATE INDEX IF NOT EXISTS feedsByDate ON feedEntries (updated DESC);
			COMMIT;"
		).expect("couldn't create basic schema");
		
		migrate_db(&mut conn).expect("couldn't migrate db");
		let mut stmt = conn.prepare("PRAGMA table_info(feedEntries);").expect("couldn't create statement");
		let names: HashSet<String> = stmt.query_map(&[], |row| row.get(1)).expect("couldn't query").map(|x| x.expect("aaa")).collect();
		assert!(names.contains("isRead"));
		assert_eq!(get_schema_version(&conn).expect("couldn't get version"), 1);
	}
}