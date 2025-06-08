use uuid::Uuid;

pub struct UserAccount {
    user_info: UserInformation,
    id: Uuid,
}
struct UserInformation {
    username: String,
    email: String,
    password: String,
}
