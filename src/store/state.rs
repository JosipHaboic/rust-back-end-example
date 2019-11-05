#![allow(dead_code)]
use crate::core::traits::base::Gateway;
use crate::gateways::user::UserTableGateway;
use rusqlite::Connection;
use std::sync::Mutex;

pub struct DatabaseState<'a> {
    pub user_table_gateway: UserTableGateway<'a>,
}

impl<'a> DatabaseState<'a> {
    pub fn new(connection: Mutex<Connection>) -> DatabaseState<'a> {
        DatabaseState {
            user_table_gateway: UserTableGateway::init(&connection),
        }
    }
}

pub struct AppState<'a> {
    pub db: DatabaseState<'a>,
}

impl<'a> AppState<'a> {
    pub fn new() -> AppState<'a> {
        AppState {
            db: DatabaseState::new(Mutex::new(Connection::open_in_memory().unwrap())),
        }
    }
}
