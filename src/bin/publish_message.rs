use self::models::TxMessage;
use diesel::prelude::*;
use std::env::args;
use xmrbc::*;

// manual message publishing
fn main() {
    use self::schema::messages::dsl::{messages, published};

    let id = args()
        .nth(1)
        .expect("publish_message requires a message id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();

    let m = diesel::update(messages.find(id))
        .set(published.eq(true))
        .get_result::<TxMessage>(connection)
        .unwrap();
    println!("Published message {}", m.subaddress);
}
