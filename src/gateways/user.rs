#![allow(dead_code)]
use crate::core::traits::base::{Entity, Gateway};
use crate::core::traits::data_source::TableGateway;
use crate::core::traits::object_relational::structural::IdentityField;
use crate::core::types::sqlite3::Params;
use crate::models::User;
use rusqlite::{Connection, Error as DBError, NO_PARAMS};
use std::include_str;

/// implement layer super-type marker trait
impl Entity for User {}

impl IdentityField for User {
    type IdType = String;

    fn id(self: &Self) -> &Self::IdType {
        &self.uuid
    }
}

pub struct UserTableGateway<'a> {
    connection: &'a Connection,
}

impl<'a> Gateway<'a> for UserTableGateway<'a> {
    type Connection = Connection;

    fn init(connection: &'a Self::Connection) -> UserTableGateway {
        UserTableGateway { connection }
    }
}

impl<'a> TableGateway<'a> for UserTableGateway<'a> {
    type Model = User;
    type Params = Params;
    type Error = DBError;

    fn insert(self: &Self, params: &Self::Params) -> Result<(), Self::Error> {
        let mut sql_statement = self
            .connection
            .prepare(include_str!("../sql/user/insert.sql"))
            .unwrap();

        match sql_statement.execute(&[
            params.get("uuid").unwrap(),
            params.get("username").unwrap(),
            params.get("password").unwrap(),
        ]) {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    fn find(self: &Self, id: Option<&str>) -> Result<Vec<Self::Model>, Self::Error> {
        match id {
            Some(uuid) => {
                let mut sql_statement = self
                    .connection
                    .prepare(include_str!("../sql/user/find.sql"))
                    .unwrap();
                match sql_statement.query_row(&[uuid], |row| {
                    Ok(User {
                        uuid: row.get(0).unwrap(),
                        username: row.get(1).unwrap(),
                        password: row.get(2).unwrap(),
                        inserted_at: Some(row.get(3).unwrap()),
                    })
                }) {
                    Ok(user) => Ok(vec![user]),
                    Err(error) => Err(error),
                }
            }
            None => {
                match self
                    .connection
                    .prepare(include_str!("../sql/user/find_all.sql"))
                    .unwrap()
                    .query_map(NO_PARAMS, |row| {
                        Ok(User {
                            uuid: row.get(0).unwrap(),
                            username: row.get(1).unwrap(),
                            password: row.get(2).unwrap(),
                            inserted_at: Some(row.get(3).unwrap()),
                        })
                    }) {
                    Ok(result) => {
                        let mut users: Vec<User> = Vec::new();
                        for i in result {
                            if let Ok(user) = i {
                                users.push(user)
                            }
                        }
                        Ok(users)
                    }
                    Err(error) => Err(error),
                }
            }
        }
    }

    fn update(self: &Self, params: &Self::Params) -> Result<(), Self::Error> {
        match self.connection.execute(
            include_str!("../sql/user/update.sql"),
            &[
                &params.get("username").unwrap(),
                &params.get("password").unwrap(),
                &params.get("uuid").unwrap(),
            ],
        ) {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    fn delete(self: &Self, id: &str) -> Result<(), Self::Error> {
        match self
            .connection
            .execute(include_str!("../sql/user/delete.sql"), &[id])
        {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::types::Value;

    #[test]
    fn test_user_gateway() {
        let connection = Connection::open_in_memory().unwrap();
        assert!(connection
            .execute_batch(include_str!("../sql/user/__create__.sql"))
            .is_ok());

        let user_gateway = UserTableGateway::init(&connection);

        let mut insert_params = Params::new();

        let user = User::new("Josip".to_owned(), "1q2w3e4r".to_owned());

        insert_params.insert("username".to_owned(), Value::Text(user.username));
        insert_params.insert("password".to_owned(), Value::Text(user.password));
        insert_params.insert("uuid".to_owned(), Value::Text(user.uuid));

        assert!(user_gateway.insert(&insert_params).is_ok());

        let users = user_gateway.find(None);

        if let Ok(user_list) = users {
            assert_eq!(user_list.capacity(), 1);
            let user = &user_gateway.find(Some(&user_list[0].uuid)).unwrap()[0];
            assert_eq!(user.uuid, user_list[0].uuid);
            assert_eq!(user.username, user_list[0].username);
            assert_eq!(user.password, user_list[0].password);
        } else {
            assert!(false);
        }
    }
}
