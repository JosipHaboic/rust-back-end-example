use crate::core::traits::base::{Entity, Gateway};
use crate::core::traits::data_source::TableGateway;
use crate::core::traits::object_relational::structural::IdentityField;
use crate::core::types::sqlite3::Params;
use crate::models::User;
use rusqlite::{Connection, NO_PARAMS};
use std::include_str;
use std::sync::Mutex;

/// implement layer super-type marker trait
impl Entity for User {}

impl IdentityField for User {
    type IdType = String;

    fn id(self: &Self) -> &Self::IdType {
        &self.uuid
    }
}

pub struct UserTableGateway<'a> {
    connection: &'a Mutex<Connection>,
}

impl<'a> Gateway<'a> for UserTableGateway<'a> {
    type Connection = Mutex<Connection>;

    fn init(connection: &'a Self::Connection) -> UserTableGateway<'a> {
        UserTableGateway { connection }
    }
}

impl<'a> TableGateway<'a> for UserTableGateway<'a> {
    type Model = User;
    type Params = Params<'a>;

    fn create_table(self: &Self) -> bool {
        self.connection
            .lock()
            .unwrap()
            .execute(include_str!("../sql/user/create_table.sql"), NO_PARAMS)
            .is_ok()
    }

    fn drop_table(self: &Self) -> bool {
        self.connection
            .lock()
            .unwrap()
            .execute(include_str!("../sql/user/drop_table.sql"), NO_PARAMS)
            .is_ok()
    }

    fn insert(self: &Self, params: &Self::Params) -> bool {
        let connection = self.connection.lock().unwrap();
        let mut sql_statement = connection
            .prepare(include_str!("../sql/user/insert.sql"))
            .unwrap();

        sql_statement
            .execute(&[
                params.get("uuid").unwrap(),
                params.get("username").unwrap(),
                params.get("password").unwrap(),
            ])
            .is_ok()
    }

    fn find(self: &Self, id: &str) -> Option<Self::Model> {
        let connection = self.connection.lock().unwrap();
        let mut sql_statement = connection
            .prepare(include_str!("../sql/user/find.sql"))
            .unwrap();
        match sql_statement.query_row(&[id], |row| {
            Ok(User {
                uuid: row.get(0).unwrap(),
                username: row.get(1).unwrap(),
                password: row.get(2).unwrap(),
                created_at: row.get(3).unwrap(),
            })
        })
        {
            Ok(user) => Some(user),
            Err(_) => None,
        }
    }

    fn update(self: &Self, params: &Self::Params) -> bool {
        self.connection
            .lock()
            .unwrap()
            .execute(
                include_str!("../sql/user/update.sql"),
                &[
                    &params.get("username").unwrap(),
                    &params.get("password").unwrap(),
                    &params.get("uuid").unwrap(),
                ],
            )
            .is_ok()
    }

    fn delete(self: &Self, id: &str) -> bool {
        self.connection
            .lock()
            .unwrap()
            .execute(include_str!("../sql/user/delete.sql"), &[id])
            .is_ok()
    }
}

// this should be done in a different way, by making more smart "find" method
impl <'a> UserTableGateway <'a> {
    pub fn find_all(self: &Self) -> Option<Vec<User>> {
        let connection = self.connection.lock().unwrap();
        let mut sql_statement = connection
            .prepare(include_str!("../sql/user/find_all.sql"))
            .unwrap();

        match sql_statement.query_map(NO_PARAMS, |row| {
            Ok(User {
                uuid: row.get(0).unwrap(),
                username: row.get(1).unwrap(),
                password: row.get(2).unwrap(),
                created_at: row.get(3).unwrap(),
            })
        }) {
            Ok(result) =>  {
                let mut users: Vec<User> = Vec::new();

                for i in result {
                    if let Ok(user) = i {
                        users.push(user)
                    }
                }

                Some(users)
            },
            Err(_) => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::types::Value;

    #[test]
    fn test_user_gateway() {
        let connection = Mutex::new(Connection::open_in_memory().unwrap());

        let user_gateway = UserTableGateway::init(&connection);
        assert!(user_gateway.create_table());

        let mut insert_params = Params::new();

        let user = User::new("Josip".to_owned(), "1q2w3e4r".to_owned());

        insert_params.insert("username", Value::Text(user.username));
        insert_params.insert("password", Value::Text(user.password));
        insert_params.insert("uuid", Value::Text(user.uuid));

        assert!(user_gateway.insert(&insert_params));

        let id: String = connection
            .lock()
            .unwrap()
            .query_row("SELECT uuid FROM users;", NO_PARAMS, |row| {
                Ok(row.get(0).unwrap())
            })
            .unwrap();

        assert!(id.len() > 0);

        match user_gateway.find(&id) {
            Some(user) => {
                assert_eq!(user.id(), &id);
                assert_eq!(user.username, "Josip".to_owned());
                assert_eq!(user.password, "1q2w3e4r".to_owned());
            }
            None => assert!(false),
        }
    }
}
