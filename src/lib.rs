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
    pub metadata: Metadata,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Metadata(pub Vec<u8>);

impl Network {
    pub fn api_endpoint(&self) -> &str {
        match self {
            Network::Monero => "https://hiahatf.org/xmr/relay",
            Network::Stagenet => "https://hiahatf.org/xmr/stagenet/relay"
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

impl FromStr for Metadata {
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Metadata(hex::decode(s)?))
    }
}

impl Display for Metadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Why no `encode_to_fmt`???
        f.write_str(&hex::encode(&self.0))
    }
}
