// Simplified websocket implementation
use std::net::TcpStream;
use thiserror::Error;
//use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use url::Url;
use ewebsock::{connect, WsMessage, WsSender, WsReceiver, WsEvent};

#[derive(Error, Debug, Eq, PartialEq)]
pub enum SimplifiedWSError {
    #[error("Error while connecting to the websocket server")]
    ConnectionError,

    #[error("Error parsing the websocket url, the url must be in the format wss://<host>:<port>")]
    UrlParseError,

    #[error("Error while sending the message to the websocket server")]
    SendMessageError,

    #[error("Error while receiving the message from the websocket server")]
    ReceiveMessageError,
}

pub struct SimplifiedWS {
    pub url: Url,
    pub sender: WsSender,
    pub reciver: WsReceiver,
}

impl SimplifiedWS {
    pub fn new(url: &str) -> Result<Self, SimplifiedWSError> {
        let url = match Url::parse(url) {
            Ok(url) => url,
            Err(_) => return Err(SimplifiedWSError::UrlParseError),
        };

        let (sender, reciver) = match connect(&*url.to_string()) {
            Ok((socket, response)) => (socket, response),
            Err(_) => return Err(SimplifiedWSError::ConnectionError),
        };

        Ok(Self { url, sender, reciver })
    }

    pub fn send_message(&mut self, message: &WsMessage) -> Result<(), SimplifiedWSError> {
        self.sender.send(message.clone());
       Ok(())
    }

    pub fn read_message(&mut self) -> Result<WsEvent, SimplifiedWSError> {
        let mut events = vec![];
        while let Some(event) = self.reciver.try_recv() {
            events.push(event);
        }

        // println!("{:?}", events);

        // Err(SimplifiedWSError::ReceiveMessageError)


        match self.reciver.try_recv() {
            Some(message) => Ok(message),
            None => Err(SimplifiedWSError::ReceiveMessageError),
        }
    }
}
