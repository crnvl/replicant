use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RegisterData {
    pub username: String,
    pub public_key: String,
}