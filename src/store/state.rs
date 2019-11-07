#![allow(dead_code)]
use crate::core::traits::base::Gateway;
use crate::gateways::user::UserTableGateway;
use rusqlite::Connection;
use std::sync::Mutex;

pub struct DatabaseState {
    pub user_table_gateway: UserTableGateway,
}

impl<'a> DatabaseState {
    pub fn new(connection: Mutex<Connection>) -> DatabaseState {
        DatabaseState {
            user_table_gateway: UserTableGateway::init(connection),
        }
    }
}

pub struct AppState {
    pub db: DatabaseState,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            db: DatabaseState::new(Mutex::new(
                Connection::open("./database.db").unwrap(),
            )),
        }
    }
}
