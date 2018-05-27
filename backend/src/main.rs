extern crate atom_syndication;
extern crate rusqlite;
extern crate chrono;

use std::fs::File;
use std::io::BufReader;
use atom_syndication::Feed;
use rusqlite::Connection;
use std::env;
use chrono::prelude::*;

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

            FOREIGN KEY(feedId) REFERENCES feeds(id)
        );
        CREATE INDEX IF NOT EXISTS feedsByDate ON feedEntries (updated DESC);
        COMMIT;"
    ).unwrap();
}

fn add_feed(url: &str) {
    println!("addFeed {}", url);

    let conn = Connection::open("AtomReader.sqlite").unwrap();

    conn.execute("INSERT INTO feeds(url) VALUES ( ? )", &[ &url ]).unwrap();
}

fn sync() {
    println!("sync");

    let mut conn = Connection::open("AtomReader.sqlite").unwrap();

    struct ToUpdate {
        id: i64,
        url: String,
        lastUpdate: i64,
    }

    let result: Vec<ToUpdate> = {
        let mut stmt = conn.prepare("SELECT id, url, lastUpdate FROM feeds").unwrap();
        let rows = stmt.query_map(&[], |row| {
            ToUpdate {
                id: row.get(0),
                url: row.get(1),
                lastUpdate: row.get::<i32, Option<i64>>(2).unwrap_or(0)
            }
        }).unwrap();
        let result = rows.map(|v| v.unwrap()).collect::<Vec<ToUpdate>>();
        result
    };

    for to_update in result {
        let now: DateTime<Utc> = Utc::now();

        let feed;
        if to_update.url.starts_with("file://") {
            let file = File::open(&to_update.url[7..]).unwrap();
            feed = Feed::read_from(BufReader::new(file)).unwrap();
        }
        else {
            unimplemented!();
        }

        let tx = conn.transaction().unwrap();

        for entry in feed.entries() {
            let updated: DateTime<Utc> = 
                DateTime::parse_from_rfc3339(entry.updated())
                .map(|t| t.with_timezone::<Utc>(&Utc))
                .unwrap_or(now);
            tx.execute("INSERT INTO feedEntries VALUES ( ?, ?, ?, ?, ?, ? )", &[
                &to_update.id,
                &entry.title(),
                &entry.id(),
                &updated.timestamp(),
                &entry.summary(),
                &entry.content().and_then(|x| x.value())
            ]).unwrap();
        }

        tx.execute("UPDATE feeds SET ( title, lastUpdate ) = ( ?, ? ) WHERE id = ?", &[
            &feed.title(),
            &Utc::now().timestamp(),
            &to_update.id,
        ]).unwrap();

        tx.commit().unwrap();
    }
}

fn import() {
    println!("import");
    let file = File::open("input.atom").unwrap();
    let feed = Feed::read_from(BufReader::new(file)).unwrap();

    let mut conn = Connection::open("AtomReader.sqlite").unwrap();
    let tx = conn.transaction().unwrap();

    tx.execute("INSERT INTO feeds(title, url) VALUES ( ?, ? )", &[ &feed.title(), &"file://input.atom" ]).unwrap();
    let feed_id = tx.last_insert_rowid();

    for entry in feed.entries() {
        let now: DateTime<Utc> = Utc::now();
        let updated: DateTime<Utc> = 
            DateTime::parse_from_rfc3339(entry.updated())
            .map(|t| t.with_timezone::<Utc>(&Utc))
            .unwrap_or(now);
        tx.execute("INSERT INTO feedEntries VALUES ( ?, ?, ?, ?, ?, ? )", &[
            &feed_id,
            &entry.title(),
            &entry.id(),
            &updated.timestamp(),
            &entry.summary(),
            &entry.content().and_then(|x| x.value())
        ]).unwrap();
    }

    tx.commit().unwrap();
}

fn get_latest() {
    let conn = Connection::open("AtomReader.sqlite").unwrap();

    struct Entry {
        title: String,
        updated: i64,
        summary: String,
        content: String,
    }

    let mut stmt = conn.prepare("SELECT title, updated, summary, content FROM feedEntries ORDER BY updated DESC LIMIT 3").unwrap();
    let result = stmt.query_map(&[], |row| {
        Entry {
            title: row.get(0),
            updated: row.get(1),
            summary: row.get::<i32, Option<String>>(2).unwrap_or(String::new()),
            content: row.get::<i32, Option<String>>(3).unwrap_or(String::new()),
        }
    }).unwrap();

    for entry in result {
        let entry = entry.unwrap();
        let date = NaiveDateTime::from_timestamp(entry.updated, 0);
        let date = DateTime::<Utc>::from_utc(date, Utc).with_timezone(&Local);
        println!("------------------------------------");
        println!("{}", entry.title);
        println!("Updated: {}", date.format("%c"));
        println!("Summary: {}", entry.summary);
        println!("");
        println!("{}", entry.content);
        println!("");
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    create_db();

    match &args[1][..] {
        "-import" => import(),
        "-getLatest" => get_latest(),
        "-addFeed" => add_feed(&args[2][..]),
        "-sync" => sync(),
        _ => (),
    }
}
