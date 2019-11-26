#![allow(dead_code)]

use crate::core::traits::domain_logic::TransactionScript;
use crate::core::types::sqlite3::Params;
use crate::models::User;
use rusqlite::Connection;

pub struct FindByUsername<'a> {
    pub username: &'a str,
}

impl<'a> FindByUsername<'a> {
    pub fn new(username: &'a str) -> FindByUsername {
        FindByUsername { username }
    }
}

impl<'a> TransactionScript for FindByUsername<'a> {
    type Output = Option<User>;
    type Connection = Connection;
    type Params = Params;

    fn execute(
        self: &Self,
        connection: &Self::Connection,
        params: &Self::Params,
    ) -> Self::Output {
        let mut statement = connection
            .prepare(
                "
				SELECT * FROM users WHERE username = ?1
			",
            )
            .unwrap();

        match statement.query_row(&[params.get("username")], |row| {
            Ok(User {
                uuid: row.get(0).unwrap(),
                username: row.get(1).unwrap(),
                password: row.get(2).unwrap(),
                inserted_at: row.get(3).unwrap(),
            })
        }) {
            Ok(result) => Some(result),
            Err(_) => None,
        }
    }
}

#[test]
fn test_find_user_by_username() {
    use crate::core::traits::domain_logic::TransactionScript;
    use crate::core::types::sqlite3::{Params, Value};
    use rusqlite::Connection;

    let connection = Connection::open_in_memory().unwrap();

    connection
        .execute_batch(
            "
		CREATE TABLE users (
			uuid TEXT PRIMARY KEY,
			username TEXT,
			password TEXT,
			inserted_at TEXT DEFAULT(datetime('now'))
		);

		INSERT INTO users (uuid, username, password)
		VALUES ('98ds-9s8ds9d-98ds-9d98s', 'Josip', '0d9s0d909ds');
	",
        )
        .unwrap();

    let user_find_by_username = FindByUsername::new("Josip");

    let mut search_params = Params::new();
    search_params
        .insert("username".to_owned(), Value::Text("Josip".to_owned()));

    if let Some(user) =
        user_find_by_username.execute(&connection, &search_params)
    {
        assert_eq!(user.username, "Josip".to_owned());
    }
}
