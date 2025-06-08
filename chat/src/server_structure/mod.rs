use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{logs::ChannelLog, user::UserAccount};

pub struct GlobalServer {
    servers: Arc<RwLock<HashMap<Uuid, Server>>>,
}

pub struct Server {
    info: ServerInfo,
    channels: Arc<RwLock<HashMap<Uuid, Channel>>>,
    user_accounts: Arc<RwLock<HashMap<Uuid, UserAccount>>>,
}

pub struct ServerInfo {
    id: Uuid,
    name: String,
    description: String,
}

pub struct Channel {
    id: Uuid,
    name: String,
    logs: Arc<RwLock<ChannelLog>>,
}
