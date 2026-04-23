use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;

use super::client::{Client, WsMessage};

pub struct Broadcast {
    clients: HashMap<String, Arc<Client>>,
}

impl Broadcast {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub fn register(&mut self, user_id: String, client: Arc<Client>) {
        self.clients.insert(user_id, client);
    }

    pub fn unregister(&mut self, user_id: &str) {
        self.clients.remove(user_id);
    }

    pub fn send_to_user(&self, user_id: &str, msg: WsMessage) -> Result<(), broadcast::error::SendError<WsMessage>> {
        if let Some(client) = self.clients.get(user_id) {
            client.send(msg);
        }
        Ok(())
    }

    pub fn broadcast(&self, msg: WsMessage) {
        for client in self.clients.values() {
            let _ = client.send(msg.clone());
        }
    }

    pub fn broadcast_to_followers(&self, follower_ids: &[String], msg: WsMessage) {
        for user_id in follower_ids {
            if let Some(client) = self.clients.get(user_id) {
                let _ = client.send(msg.clone());
            }
        }
    }
}

impl Default for Broadcast {
    fn default() -> Self {
        Self::new()
    }
}
