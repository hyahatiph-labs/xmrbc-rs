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
        default_value = "5NkuNyAUkeMZQRxbb77FxXHCTUti1tgFdRSYkXxvycP4.G9J5L4CkJS7qoirQvfxVDGvRSjn3ANjHhMopK5i3CH4E@6LdVTJhRfJKsrUtnjFqE3TpEbCYs3VZoxmaoNFqRWn4x"
    )]
    service_provider: Recipient,
    #[structopt(
        short,
        long,
        default_value = "http://cvryvrat7qfg2zqbbxthmeemw3j27emfk2n5yuuchfreepbmuybq.b32.i2p/checkandpush",
        help = "enter block explorer url"
    )]
    network: String,
    transaction: Transaction,
}

impl Options {
    fn into_parts(self) -> (String, Request, Recipient) {
        let req = Request {
            network: self.network,
            transaction: self.transaction,
        };
        (self.websocket, req, self.service_provider)
    }
}

#[tokio::main]
async fn main() {
    let opts: Options = StructOpt::from_args();
    let (websocket, request, recipient) = opts.into_parts();

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
}
