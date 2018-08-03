#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate atom_syndication;
extern crate rusqlite;
extern crate chrono;
extern crate websocket;

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
        tx.execute("INSERT INTO feedEntries VALUES ( ?, ?, ?, ?, ?, ? )", &[
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

fn get_latest() -> Vec<FeedEntry> {
    let conn = Connection::open("AtomReader.sqlite").unwrap();

    let mut stmt = conn.prepare("SELECT feedId, title, id, updated, summary, content FROM feedEntries ORDER BY updated DESC LIMIT 3").unwrap();
    let result = stmt.query_map(&[], |row| {
        FeedEntry {
            feed_id: row.get(0),
            title: row.get(1),
            id: row.get(2),
            updated: row.get(3),
            summary: row.get::<i32, Option<String>>(4).unwrap_or(String::new()),
            content: row.get::<i32, Option<String>>(5).unwrap_or(String::new()),
        }
    }).unwrap().map(|r| r.unwrap()).collect::<Vec<FeedEntry>>();

    result
}

fn print_latest() {
    let result = get_latest();

    for entry in result {
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

#[derive(Deserialize)]
#[serde(tag = "type")]
enum RequestBody {
    GetLatest {
    },
    AddFeed {
        url: String,
    },
}

#[derive(Deserialize)]
struct Request {
    id: u64,
    #[serde(flatten)]
    body: RequestBody,
}

#[derive(Serialize)]
struct FeedEntry {
    feed_id: i64,
    title: String,
    id: String,
    updated: i64,
    summary: String,
    content: String,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum ResponseBody {
    Success,
    Error(String),
    FeedEntries {
        list: Vec<FeedEntry>,
    }
}

#[derive(Serialize)]
struct Response {
    id: u64,
    #[serde(flatten)]
    body: ResponseBody,
}

fn process_request(request: RequestBody) -> Result<ResponseBody, String> {
    match request {
        RequestBody::GetLatest {} => {
            Ok(ResponseBody::FeedEntries {
                list: get_latest(),
            })
        },
        RequestBody::AddFeed { url } => {
            add_feed(&url[..]);
            Ok(ResponseBody::Success)
        },
    }
}

fn server() {
    use std::thread;
    use websocket::OwnedMessage;
    use websocket::sync::Server;
    use std::time::Duration;

    thread::spawn(move || {
        loop {
            sync();
            thread::sleep(Duration::from_secs(10));
        }
    });

    let server = Server::bind("127.0.0.1:2794").unwrap();

    for request in server.filter_map(Result::ok) {
        // Spawn a new thread for each connection.
        thread::spawn(move || {
            let client = request.use_protocol("atom-client").accept().unwrap();
            let ip = client.peer_addr().unwrap();
            println!("Connection from {}", ip);

            let (mut receiver, mut sender) = client.split().unwrap();

            for message in receiver.incoming_messages() {
                let message = message.unwrap();

                match message {
                    OwnedMessage::Close(_) => {
                        let message = OwnedMessage::Close(None);
                        sender.send_message(&message).unwrap();
                        println!("Client {} disconnected", ip);
                        return;
                    }
                    OwnedMessage::Ping(ping) => {
                        let message = OwnedMessage::Pong(ping);
                        sender.send_message(&message).unwrap();
                    }
                    OwnedMessage::Text(message) => {
                        let message: Request = match serde_json::from_str(&message[..]) {
                            Ok(message) => message,
                            Err(e) => {
                                println!("Parsing message failed: {}", e);
                                continue;
                            }
                        };
                        match process_request(message.body) {
                            Ok(body) => {
                                let response = Response {
                                    id: message.id,
                                    body: body,
                                };
                                let response = serde_json::to_string_pretty(&response).unwrap();
                                sender.send_message(&OwnedMessage::Text(response)).unwrap();
                            },
                            Err(e) => {
                                let response = Response {
                                    id: message.id,
                                    body: ResponseBody::Error(e),
                                };
                                let response = serde_json::to_string_pretty(&response).unwrap();
                                sender.send_message(&OwnedMessage::Text(response)).unwrap();
                            }
                        }
                    },
                    _ => ()
                }
            }
        });
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    create_db();

    match &args[1][..] {
        "-getLatest" => print_latest(),
        "-addFeed" => add_feed(&args[2][..]),
        "-sync" => sync(),
        "-server" => server(),
        _ => (),
    }
}
