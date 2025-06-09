pub fn run() {}

#[derive(Debug, Clone)]
pub enum Message {
    ServerSelected(usize),
    ChannelSelected(usize),
    MessageInputChanged(String),
    SendMessage,
}
