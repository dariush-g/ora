use crate::message::content::MessageContent;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChannelLog {
    pub channel_id: uuid::Uuid,
    pub logs: Vec<MessageLog>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MessageLog {
    content: MessageContent,
    time: std::time::SystemTime,
    from: String,
}

impl MessageLog {
    pub fn new(content: MessageContent, from: String) -> Self {
        Self {
            content,
            time: std::time::SystemTime::now(),
            from,
        }
    }

    pub fn get_content(&self) -> &MessageContent {
        &self.content
    }

    pub fn get_time(&self) -> std::time::SystemTime {
        self.time
    }

    pub fn get_from(&self) -> &String {
        &self.from
    }
}
