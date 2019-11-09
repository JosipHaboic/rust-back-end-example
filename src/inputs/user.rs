use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserInput {
    pub username: String,
    pub password: String,
}
