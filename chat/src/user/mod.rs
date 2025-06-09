use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAccount {
    user_info: UserInformation,
    id: Uuid,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct UserInformation {
    username: String,
    email: String,
    password: String,
}
