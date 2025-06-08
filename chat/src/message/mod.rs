pub mod content;
mod sha256;
pub mod encrypt;
pub mod dh;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalMessage {
    pub sender: String,
    // input is message -> encrypt it
    pub content: crate::message::encrypt::MessageType,
    pub timestamp: SystemTime,
    pub destination: GlobalMessageDestinationInfo,
}

impl GlobalMessage {
    pub fn new(
        sender: String,
        content: crate::message::encrypt::MessageType,
        destination: GlobalMessageDestinationInfo,
    ) -> Self {
        Self {
            sender,
            content,
            timestamp: SystemTime::now(),
            destination,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalMessageDestinationInfo {
    pub server_id: Uuid,
    pub channel_id: Uuid,
}
