// #![allow(dead_code)]
use rusqlite::Connection;
use std::sync::Mutex;

pub struct DatabaseState {
    pub connection: Mutex<Connection>,
}

impl<'a> DatabaseState {
    pub fn new(connection: Mutex<Connection>) -> DatabaseState {
        DatabaseState { connection }
    }
}

pub struct AppState {
    pub version: u8,
    pub db: DatabaseState,
}

impl AppState {
    pub fn new(version: u8) -> AppState {
        AppState {
            version,
            db: DatabaseState::new(Mutex::new(
                Connection::open("./database.db").unwrap(),
            )),
        }
    }
}
