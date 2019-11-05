#![allow(dead_code)]
use crate::utils::timestamp;
use rs_uuid::uuid16;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub created_at: String,
}

impl User {
    pub fn new(username: String, password: String) -> User {
        User {
            uuid: uuid16(),
            username,
            password,
            created_at: timestamp(),
        }
    }
}
