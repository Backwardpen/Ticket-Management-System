use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    pub email: String,
    pub password: String,
    pub role: String,
}