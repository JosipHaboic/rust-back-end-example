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
