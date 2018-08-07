use std::thread;
use std::net::TcpStream;
use websocket::server::upgrade::{WsUpgrade, sync::Buffer};
use websocket::{OwnedMessage, sync::Server};
use protocol::{RequestFrame, ResponseFrame, Response};
use serde_json;
use requests::process;
use rusqlite::Connection;

fn connection_thread(request: WsUpgrade<TcpStream, Option<Buffer>>) {
    let client = request.use_protocol("atom-client").accept().unwrap();
    let ip = client.peer_addr().unwrap();
    println!("Connection from {}", ip);

    let (mut receiver, mut sender) = client.split().unwrap();

    let mut conn = Connection::open("AtomReader.sqlite").unwrap();

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
            OwnedMessage::Binary(_) => {
                println!("Unknown binary message from {}", ip);
            }
            OwnedMessage::Text(message) => {
                let message: RequestFrame = match serde_json::from_str(&message[..]) {
                    Ok(message) => message,
                    Err(e) => {
                        println!("Parsing message failed: {}\nMessage: {}", e, message);
                        continue;
                    }
                };
                println!("[Request from {}]: {:#?}", ip, message);
                match process(message.body, &mut conn) {
                    Ok(body) => {
                        let response = ResponseFrame {
                            id: message.id,
                            body: body,
                        };
                        println!("[Response to {}]: {:#?}", ip, response);
                        let response = serde_json::to_string_pretty(&response).unwrap();
                        sender.send_message(&OwnedMessage::Text(response)).unwrap();
                    },
                    Err(e) => {
                        let response = ResponseFrame {
                            id: message.id,
                            body: Response::Error {
                                error: format!("{:#?}", e)
                            },
                        };
                        let response = serde_json::to_string_pretty(&response).unwrap();
                        sender.send_message(&OwnedMessage::Text(response)).unwrap();
                    }
                }
            },
            _ => ()
        }
    }
}

pub fn server_thread() {
    let server = Server::bind("127.0.0.1:2794").unwrap();

    for request in server.filter_map(Result::ok) {
        // Spawn a new thread for each connection.
        thread::spawn(move || connection_thread(request));
    }
}
