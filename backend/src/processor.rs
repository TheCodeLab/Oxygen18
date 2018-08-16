use chrono::prelude::*;
use rusqlite::{ Connection, Transaction };
use std::io::Read;
use std::fs::File;
use atom_syndication::Feed;
use rss::Channel;
use std::time::Duration;
use std::thread;
use std::str::FromStr;

#[derive(Debug)]
enum UpdateError {
    /// Couldn't fetch the file from the network
    NetworkError(::reqwest::Error),
    /// The file cannot be parsed (malformed, unrecognized format, etc.)
    ParseError,
    /// There was an error writing to the database
    DatabaseError(::rusqlite::Error),
}

impl From<::rss::Error> for UpdateError {
    fn from(_rss_error: ::rss::Error) -> UpdateError {
        UpdateError::ParseError
    }
}

impl From<::atom_syndication::Error> for UpdateError {
    fn from(_atom_error: ::atom_syndication::Error) -> UpdateError {
        UpdateError::ParseError
    }
}

impl From<::reqwest::Error> for UpdateError {
    fn from(network_error: ::reqwest::Error) -> UpdateError {
        UpdateError::NetworkError(network_error)
    }
}

impl From<::rusqlite::Error> for UpdateError {
    fn from(db_error: ::rusqlite::Error) -> UpdateError {
        UpdateError::DatabaseError(db_error)
    }
}

fn fetch_url(url: &str) -> Result<String, UpdateError> {
    if url.starts_with("file://") {
        let mut file = File::open(&url[7..]).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        return Ok(buf)
    }

    let body = ::reqwest::get(url)?.text()?;
    Ok(body)
}

fn process_atom_feed(tx: &Transaction, feed_id: i64, feed_body: &str) -> Result<String, UpdateError> {
    let feed = Feed::from_str(feed_body)?;
    let now: DateTime<Utc> = Utc::now();

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
        ])?;
    }

    Ok(feed.title().to_owned())
}

fn process_rss_feed(tx: &Transaction, feed_id: i64, feed_body: &str) -> Result<String, UpdateError> {
    let channel = Channel::from_str(feed_body)?;
    let now: DateTime<Utc> = Utc::now();

    for item in channel.items() {
        let pub_date: DateTime<Utc> =
            // TODO: What happens if there's no publication date for the item?
            DateTime::parse_from_rfc2822(item.pub_date().unwrap())
            .map(|t| t.with_timezone::<Utc>(&Utc))
            .unwrap_or(now);
        
        tx.execute("INSERT OR REPLACE INTO feedEntries VALUES ( ?, ?, ?, ?, ?, ? )", &[
            &feed_id,
            &item.title(),
            &item.guid().unwrap().value(),
            &pub_date.timestamp(),
            &item.description().unwrap(),
            &item.content().unwrap(),
        ])?;
    }

    Ok(channel.title().to_owned())
}

fn update_feed(conn: &mut Connection, feed_id: i64, url: &str) -> Result<(), UpdateError> {
    println!("updating: {}", url);

    let body = fetch_url(url)?;
    let tx = conn.transaction()?;

    let new_title = match process_atom_feed(&tx, feed_id, &body) {
        Ok(new_title) => new_title,
        Err(UpdateError::ParseError) => match process_rss_feed(&tx, feed_id, &body) {
            Ok(new_title) => new_title,
            Err(error) => return Err(error)
        },
        Err(unknown_error) => return Err(unknown_error),
    };

    tx.execute("UPDATE feeds SET ( title, lastUpdate ) = ( ?, ? ) WHERE id = ?", &[
        &new_title,
        &Utc::now().timestamp(),
        &feed_id,
    ])?;

    tx.commit()?;

    Ok(())
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
            match update_feed(&mut conn, to_update.id, &to_update.url) {
                Ok(()) => println!("feed updated successfully"),
                Err(error) => println!("error updating feed: {:?}", error)
            }
        }
    }
}

pub fn processor_thread() {
    loop {
        sync();
        thread::sleep(Duration::from_secs(10));
    }
}
