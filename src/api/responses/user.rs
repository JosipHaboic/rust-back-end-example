use crate::models::User;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UsersResponse<'a> {
    pub status: u16,
    pub message: &'a str,
    pub result: Vec<User>,
}
