pub mod content;
pub mod dh;
pub mod encrypt;
mod sha256;
use std::time::SystemTime;

use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{logs::ChannelLog, user::UserAccount};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalMessage {
    pub from: String,
    // input is message -> encrypt it
    pub content: crate::message::encrypt::MessageType,
    pub timestamp: SystemTime,
    pub channel: Uuid,
}

impl GlobalMessage {
    pub fn new(from: String, content: crate::message::encrypt::MessageType, channel: Uuid) -> Self {
        Self {
            from,
            content,
            timestamp: SystemTime::now(),
            channel,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//
    //sends info abt server to joined members
    //also if offline then rejoined
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//
    //
    ServerUpdate {
        name: String,
        users: Vec<UserAccount>,
        logs: ChannelLog,
    },
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//
    //for online members to get userlist notifications
    //to see joined members
    MemberJoined(UserAccount),
    MemberLeft(UserAccount),
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//
    //also for online members to see live messages
    //message is encryped on send
    //
    ClientMessage(GlobalMessage),

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//
    // server generates random key and shares with
    // the requesting client -> establishing the shared
    // secret -> client sends encrypted message with nonce
    // then server decrypts message, stores it into the channel logs,
    // then establishes connection with the other clients in the channel
    // and shares the key with them as well
    //
    KeyShare(BigUint),
}
