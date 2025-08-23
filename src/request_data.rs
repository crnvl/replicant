use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RegisterData {
    pub username: String,
    pub public_key: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegisterResponse {
    pub message: String,
    pub auth_token: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SendData {
    pub to: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SendResponse {
    pub message: String,
}
