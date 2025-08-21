pub struct User {
    pub username: String,
    pub public_key: String,
    pub avatar: String,
}

pub struct UserCredentials {
    pub username: String,
    pub auth_token: String,
}

pub struct Message {
    pub from: String,
    pub to: String,
    pub message_data: String,
}
