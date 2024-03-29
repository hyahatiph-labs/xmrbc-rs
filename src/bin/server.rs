
use xmrbc::{Request, Transaction};
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use nym_websocket::responses::ServerResponse;
use structopt::StructOpt;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;

use diesel::prelude::*;
use self::models::*;
use xmrbc::*;

const MSG_UPPER_BOUND: usize = 512;
const MSG_LOWER_BOUND: usize = 1;

#[derive(StructOpt)]
struct Options {
    #[structopt(short, long, default_value = "ws://127.0.0.1:1977")]
    websocket: String,
}

async fn submit_tx(net: String, tx: Transaction) {

    // TODO: dont do this if xmr rpc is available
    // implement xmr rpc connection
    debug!("Submitting transaction to network {:?}: {}", net, tx);

    let request = format!("rawtxdata={}&action=push", tx.to_string());

    let client = reqwest::Client::new();
    match client
        .post(&net)
        .body(request)
        .send()
        .await
    {
        Ok(response) => {
            debug!("Done. Response: {:?}", response.text().await);
        }
        Err(e) => {
            error!("Error submitting tx: {}", e);
        }
    }
}

fn build_identity_request() -> Message {
    let nym_message = nym_websocket::requests::ClientRequest::SelfAddress;
    Message::Binary(nym_message.serialize())
}

fn parse_nym_message(msg: Message) -> nym_websocket::responses::ServerResponse {
    match msg {
        Message::Binary(bytes) => nym_websocket::responses::ServerResponse::deserialize(&bytes)
            .expect("Could not decode nym client response"),
        msg => panic!("Unexpected message: {:?}", msg),
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let options: Options = Options::from_args();

    debug!("Connecting to websocket at {}", &options.websocket);
    let (mut ws, _) = connect_async(&options.websocket)
        .await
        .expect("Couldn't connect to nym websocket");

    debug!("Requesting own identity from nym client");
    ws.send(build_identity_request())
        .await
        .expect("failed to send identity request");

    while let Some(Ok(msg)) = ws.next().await {
        let msg = parse_nym_message(msg);

        let msg_bytes = match msg {
            ServerResponse::Received(msg_bytes) => {
                debug!("Received client request {:?}", msg_bytes);
                msg_bytes
            }
            ServerResponse::SelfAddress(addr) => {
                info!("Listening on {}", addr);
                continue;
            }
            ServerResponse::Error(err) => {
                error!("Received error from nym client: {}", err);
                continue;
            }
        };

        let request: Request = match bincode::deserialize(&msg_bytes.message) {
            Ok(msg) => msg,
            Err(e) => {
                warn!("Could not decode client request");
                debug!("Client request decoding error: {}", e);
                continue;
            }
        };
        // store txm
        store_txm(request.subaddress, request.txm);
        tokio::spawn(submit_tx(request.network, request.transaction));
    }
}

// got the message, put it in the db
// TODO: chaotic message storage.
//       encryption isnt really necessary
//       only msg sender and recipient and prove ownership?
// TODO: message size (512)?
fn store_txm(subaddress: String, body: String) {
    let connection = &mut establish_connection();
    if body.len() < MSG_UPPER_BOUND && body.len() > MSG_LOWER_BOUND {
        let msg = create_message(connection, &subaddress, &body);
        println!("\nSaved draft {} with id {}", subaddress, msg.id);
    }
}

// used to get stored messages
// TODO: signature verification
// TODO: move to rpc, implement a challenge response model
//      1) node requests messages
//      2) return 403 with random data to sign
//      3) node sends signed data with valid address
//      4) respond with messages for validated node, ezpz
fn _fetch_txm(txm_id: i32) -> String {
    use self::schema::messages::dsl::*;
    let connection = &mut establish_connection();
    let template = TxMessage {id: txm_id, subaddress: "".to_string(), body: "".to_string(), published: true };
    let r: TxMessage = match messages.find(txm_id).first(connection) {
        Ok(m) => m,
        _=> template
    };
    r.body.to_string()
}
