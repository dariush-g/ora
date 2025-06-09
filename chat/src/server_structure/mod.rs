use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{logs::ChannelLog, user::UserAccount};

#[derive(Debug, Clone)]
pub struct GlobalServer {
    user_accounts: Arc<RwLock<HashMap<Uuid, UserAccount>>>,
    channels: Arc<RwLock<HashMap<Uuid, Channel>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Channel {
    id: Uuid,
    name: String,
    members: HashMap<UserAccount, >,
    logs: ChannelLog,
}
