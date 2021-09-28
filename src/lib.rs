use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum Network {
    Monero,
    Stagenet
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub network: Network,
    pub transaction: Transaction,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Transaction(pub Vec<u8>);

impl Network {
    pub fn api_endpoint(&self) -> &str {
        match self {
            Network::Monero => "http://xmr3wzqt4r4ypgrljbt7xpjemswbefkfmo6xu4s7j34dscf5ji3q.b32.i2p/checkandpush",
            Network::Stagenet => "http://xmr326jan2ysnf4bwuut5apbajtab4kgbv5r5cwduio2xos7abrq.b32.i2p/checkandpush"
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct UnknownNetworkError(String);

impl Error for UnknownNetworkError {}

impl Display for UnknownNetworkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown network \"{}\"", self.0)
    }
}

impl FromStr for Network {
    type Err = UnknownNetworkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "monero" => Ok(Network::Monero),
            "stagenet" => Ok(Network::Stagenet),
            // Annoying that we need to clone here, but `FromStr::Err` doesn't allow lifetimes
            other => Err(UnknownNetworkError(other.to_string())),
        }
    }
}

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
