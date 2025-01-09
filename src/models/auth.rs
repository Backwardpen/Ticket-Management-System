use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    pub email: String,
    pub username: String,
    pub password: String,
}