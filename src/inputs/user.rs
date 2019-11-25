use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInput {
    pub username: String,
    pub password: String,
}
