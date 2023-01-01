use std::collections::HashMap;

use mini_redis::{Connection, Frame, Command, Command::{Set, Get}, };
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    let mut db = HashMap::new();

    // Connection was the abstract of redis reading/writing process
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("Got: {:?}", frame);

        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("Ok".to_string())
            },
            Get(cmd) => {
                if let Some(val) = db.get(cmd.key()) {
                    Frame::Bulk(val.clone().into())
                } else {
                    Frame::Null
                }
            }
            _ => unimplemented!()
        };

        connection.write_frame(&response).await.unwrap();
    }
}
