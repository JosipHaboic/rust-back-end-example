#![allow(dead_code, unused_variables)]
use std::time::SystemTime;

pub fn timestamp() -> String {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(time) => format!("{}", time.as_secs()),
        Err(error) => panic!(error),
    }
}
