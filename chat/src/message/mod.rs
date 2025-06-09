pub mod content;
pub mod dh;
pub mod encrypt;
mod sha256;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{logs::ChannelLog, server_structure::Server, user::UserAccount};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalMessage {
    pub sender: String,
    // input is message -> encrypt it
    pub content: crate::message::encrypt::MessageType,
    pub timestamp: SystemTime,
    pub channel: Uuid,
}

impl GlobalMessage {
    pub fn new(
        sender: String,
        content: crate::message::encrypt::MessageType,
        channel: Uuid,
    ) -> Self {
        Self {
            sender,
            content,
            timestamp: SystemTime::now(),
            channel,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    ServerUpdate(Server),
    ChatLogUpdate(Vec<ChannelLog>),
    MemberJoined(UserAccount),
    MemberLeft(UserAccount),
    ClientMessage(GlobalMessage),
}
