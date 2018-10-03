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

fn main() {
    use std::thread;

    {
        let mut conn = Connection::open("AtomReader.sqlite").unwrap();
        migration::migrate_db(&mut conn).expect("couldn't migrate database");
    }

    thread::spawn(processor::processor_thread);
    server::server_thread();
}
