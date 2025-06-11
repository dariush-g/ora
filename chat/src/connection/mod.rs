use std::{sync::Arc, time::SystemTime};

use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use tokio::{net::TcpStream, sync::RwLock};

use crate::message::content::MessageContent;
#[derive(Debug, Clone)]
pub struct ClientConnection {
    key: BigUint,
    info: Arc<RwLock<TcpStream>>,
    last_online: SystemTime,
}
