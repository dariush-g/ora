use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageContent {
    Text(Vec<u8>),
    Image(Vec<u8>),
    Video(Vec<u8>),
    Audio(Vec<u8>),
    File(Vec<u8>),
}

impl MessageContent {
    pub fn get_content(&self) -> Vec<u8> {
        match self {
            MessageContent::Text(content) => content.clone(),
            MessageContent::Image(content) => content.clone(),
            MessageContent::Video(content) => content.clone(),
            MessageContent::Audio(content) => content.clone(),
            MessageContent::File(content) => content.clone(),
        }
    }
}
