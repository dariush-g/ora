use std::sync::Arc;

use tokio::{net::TcpStream, sync::RwLock};

pub struct ClientConnection {
    info: ConnectionInfo,
    stored_messages: 
}

#[derive(Clone, Debug)]
pub struct ConnectionInfo {
    connected: bool,
    stream: Arc<RwLock<TcpStream>>,
}
