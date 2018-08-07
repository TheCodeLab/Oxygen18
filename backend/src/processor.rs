use chrono::prelude::*;
use rusqlite::Connection;
use std::io::BufReader;
use std::fs::File;
use atom_syndication::Feed;
use std::time::Duration;
use std::thread;

fn update_feed(conn: &mut Connection, feed_id: i64, url: &str) {
    println!("updating: {}", url);
    let now: DateTime<Utc> = Utc::now();

    let feed;
    if url.starts_with("file://") {
        let file = File::open(&url[7..]).unwrap();
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
        tx.execute("INSERT OR REPLACE INTO feedEntries VALUES ( ?, ?, ?, ?, ?, ? )", &[
            &feed_id,
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
        &feed_id,
    ]).unwrap();

    tx.commit().unwrap();
}

fn sync() {
    let mut conn = Connection::open("AtomReader.sqlite").unwrap();

    struct ToUpdate {
        id: i64,
        url: String,
        last_update: i64,
    }

    let result: Vec<ToUpdate> = {
        let mut stmt = conn.prepare("SELECT id, url, lastUpdate FROM feeds").unwrap();
        let rows = stmt.query_map(&[], |row| {
            ToUpdate {
                id: row.get(0),
                url: row.get(1),
                last_update: row.get::<i32, Option<i64>>(2).unwrap_or(0)
            }
        }).unwrap();
        let result = rows.map(|v| v.unwrap()).collect::<Vec<ToUpdate>>();
        result
    };

    let now: DateTime<Utc> = Utc::now();
    for to_update in result {
        let cooldown = 30*60;
        if to_update.last_update + cooldown < now.timestamp() {
            update_feed(&mut conn, to_update.id, &to_update.url[..]);
        }
    }
}

pub fn processor_thread() {
    loop {
        sync();
        thread::sleep(Duration::from_secs(10));
    }
}
