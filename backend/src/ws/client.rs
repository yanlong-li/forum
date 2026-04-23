use std::sync::Arc;
use tokio::sync::broadcast;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub payload: serde_json::Value,
}

pub struct Client {
    pub user_id: String,
    sender: broadcast::Sender<WsMessage>,
}

pub type ClientSender = broadcast::Sender<WsMessage>;
pub type ClientReceiver = broadcast::Receiver<WsMessage>;

impl Client {
    pub fn new() -> Arc<Self> {
        let (tx, _rx) = broadcast::channel(100);
        Arc::new(Self {
            user_id: String::new(),
            sender: tx,
        })
    }

    pub fn with_user(user_id: String) -> Arc<Self> {
        let (tx, _rx) = broadcast::channel(100);
        Arc::new(Self {
            user_id,
            sender: tx,
        })
    }

    pub fn sender(&self) -> broadcast::Sender<WsMessage> {
        self.sender.clone()
    }

    pub fn subscribe(&self) -> broadcast::Receiver<WsMessage> {
        self.sender.subscribe()
    }

    pub fn send(&self, msg: WsMessage) {
        let _ = self.sender.send(msg);
    }
}

impl Default for Client {
    fn default() -> Self {
        Self {
            user_id: String::new(),
            sender: broadcast::channel(100).0,
        }
    }
}
