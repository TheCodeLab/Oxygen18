#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate atom_syndication;
extern crate rusqlite;
extern crate chrono;
extern crate websocket;
extern crate rss;
extern crate reqwest;

mod protocol;
mod requests;
mod processor;
mod server;
mod migration;

use rusqlite::Connection;

fn create_db() {
    let conn = Connection::open("AtomReader.sqlite").unwrap();

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
    ).unwrap();
}

fn main() {
    use std::thread;

    create_db();

    {
        let mut conn = Connection::open("AtomReader.sqlite").unwrap();
        migration::migrate_db(&mut conn).expect("couldn't migrate database");
    }

    thread::spawn(processor::processor_thread);
    server::server_thread();
}
