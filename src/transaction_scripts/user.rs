#![allow(dead_code)]

use crate::core::traits::domain_logic::TransactionScript;
use crate::core::types::sqlite3::Params;
use crate::models::User;
use rusqlite::{Connection, Error as DBError};

pub struct FindByField<'a> {
    pub field: &'a str,
}

impl<'a> FindByField<'a> {
    pub fn new(field: &'a str) -> Self {
        FindByField {
            field,
        }
    }
}

impl<'a> TransactionScript for FindByField<'a> {
    type Output = Result<Vec<User>, DBError>;
    type Connection = Connection;
    type Params = Params;

    fn execute(
        self: &Self,
        connection: &Self::Connection,
        params: &Self::Params,
    ) -> Self::Output {
        let mut statement = connection
            .prepare(&format!("SELECT * FROM users WHERE {} = ?1", self.field))
            .unwrap();

        let rows = statement.query_map(&[params.get("username")], |row| {
            Ok(User {
                uuid: row.get(0).unwrap(),
                username: row.get(1).unwrap(),
                password: row.get(2).unwrap(),
                inserted_at: row.get(3).unwrap(),
            })
        }).unwrap();

        rows.collect()
    }
}

#[test]
fn test_find_by_field() {
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

    let find_user_by_username = FindByField::new("username");
    let mut search_params = Params::new();
    search_params.insert("username".to_owned(), Value::Text("Josip".to_owned()));

    if let Ok(users) = find_user_by_username.execute(&connection, &search_params) {
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].username, "Josip".to_owned());
    } else {
        assert!(false);
    }

    let find_user_by_id = FindByField::new("uuid");
    search_params = Params::new();
    search_params.insert(
        "uuid".to_owned(),
        Value::Text("98ds-9s8ds9d-98ds-9d98s".to_owned()),
    );

    if let Ok(users) = find_user_by_id.execute(&connection, &search_params) {
        assert_eq!(users.len(), 1);
        assert_eq!(
            users[0].uuid,
            "98ds-9s8ds9d-98ds-9d98s".to_owned(),
            "Did not find the uuid"
        );
    } else {
        assert!(false);
    }
}
