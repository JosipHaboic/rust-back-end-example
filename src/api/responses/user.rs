use serde_derive::{Serialize, Deserialize};
use crate::models::User;


#[derive(Serialize, Deserialize)]
pub struct UsersResponse<'a> {
	pub status: u16,
	pub message: &'a str,
	pub result: Vec<User>,
}

