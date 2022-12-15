use serde::{Deserialize, Serialize};
use thiserror::Error;

// Implementation of the NIP11 protocol
// https://github.com/nostr-protocol/nips/blob/master/11.md

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayInformationDocument {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub pubkey: Option<String>,
    pub contact: Option<String>,
    pub supported_nips: Option<Vec<u16>>,
    pub software: Option<String>,
    pub version: Option<String>,
}

#[derive(Error, Debug)]
pub enum NIP11Error {
    #[error("The relay information document is invalid")]
    InvalidRelayInformationDocument,

    #[error("The relay information document is not accessible")]
    RelayInformationDocumentNotAccessible,

    #[error("Min request error")]
    MinreqError(minreq::Error),
}

impl From<minreq::Error> for NIP11Error {
    fn from(err: minreq::Error) -> Self {
        Self::MinreqError(err)
    }
}

pub fn get_relay_information_document(
    relay_url: &str,
) -> Result<RelayInformationDocument, NIP11Error> {
    let relay_url = relay_url.replacen("ws", "http", 1);
    let response = minreq::get(relay_url)
        .with_header("Accept", "application/nostr+json")
        .send()?
        .json::<RelayInformationDocument>()?;

    Ok(response)
}
