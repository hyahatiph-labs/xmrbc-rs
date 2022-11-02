use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// diesel orm imports
pub mod models;
pub mod schema;

use self::models::{NewTxMessage, TxMessage};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub subaddress: String,
    pub network: String,
    pub transaction: Transaction,
    pub txm: String,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Transaction(pub Vec<u8>);

impl FromStr for Transaction {
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Transaction(hex::decode(s)?))
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Why no `encode_to_fmt`???
        f.write_str(&hex::encode(&self.0))
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_message(conn: &mut PgConnection, subaddress: &str, body: &str) -> TxMessage {
    use crate::schema::messages;

    let new_msg = NewTxMessage { subaddress, body };

    diesel::insert_into(messages::table)
        .values(&new_msg)
        .get_result(conn)
        .expect("Error saving new message")
}

