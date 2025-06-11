use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageContent {
    Text(Vec<u8>),
    Image(Vec<u8>),
    Video(Vec<u8>),
    Audio(Vec<u8>),
    File(Vec<u8>),
    LEFTSERVER,
    JOINEDSERVER,
}

impl MessageContent {
    pub fn get_content(&self) -> &[u8] {
        match self {
            MessageContent::Text(content) => content,
            MessageContent::Image(content) => content,
            MessageContent::Video(content) => content,
            MessageContent::Audio(content) => content,
            MessageContent::File(content) => content,
            _ => &[],
        }
    }
}
