use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub network: String,
    pub transaction: Transaction,
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
