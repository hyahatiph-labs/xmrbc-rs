#[macro_use] extern crate rocket;
#[cfg(test)] mod tests;

use std::str::FromStr;
use std::time::{SystemTime};
use xmrbc::{Request, Transaction};
use futures::SinkExt;
use nym_addressing::clients::Recipient;
use structopt::StructOpt;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

#[derive(StructOpt)]
struct Options {
    #[structopt(short, long, default_value = "ws://127.0.0.1:1977")]
    websocket: String,
    #[structopt(
        short,
        long,
        parse(try_from_str = Recipient::try_from_base58_string),
        default_value = "D5ehV2EdXb8LcPZM5kNL1C1D61pcJqzKRKNsg2hrB6ig.3iArACGGkrd5unbmqHBJkAWNqNRd29EA79aFWJZUxGwR@ERFGo6CbzLe51EZUgGJoYg2zsYnVXVVfLyB5CZ8P9vsU"
    )]
    service_provider: Recipient,
    #[structopt(
        short,
        long,
        default_value = "https://stagenet.xmrchain.net/checkandpush",
        help = "enter block explorer url"
    )]
    network: String,
}

impl Options {
    fn into_parts(self, tx: Transaction) -> (String, Request, Recipient) {
        let req = Request {
            network: self.network,
            transaction: tx,
        };
        (self.websocket, req, self.service_provider)
    }
}

#[get("/relay?<tx>")]
async fn relay(tx: String) -> String {
    let t: Result<Transaction, hex::FromHexError> = Transaction::from_str(&tx);
    let opts: Options = StructOpt::from_args();
    let (websocket, request, recipient) = opts.into_parts(t.unwrap());
    
    let (mut ws, _) = connect_async(&websocket)
        .await
        .expect("Couldn't connect to nym websocket");

    let nym_packet = nym_websocket::requests::ClientRequest::Send {
        recipient,
        message: bincode::serialize(&request).expect("can't fail"),
        with_reply_surb: false,
    };

    ws.send(Message::Binary(nym_packet.serialize()))
        .await
        .expect("couldn't send request");

    ws.close(None).await.expect("Failed to close websocket.");
    
    let mut response = String::new();
    let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) =>  n.as_secs(),
        Err(_) => 0,
    };
    let msg = format!(" {{\"date\": {} ,\"msg\": \"Relay attempted.\"}} ", now);
    response.push_str(&msg);
    response
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![relay])
}
