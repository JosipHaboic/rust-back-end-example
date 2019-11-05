use std::collections::HashMap;

pub mod sqlite3 {
    use super::HashMap;
    pub use rusqlite::types::Value;

    pub type Params<'a> = HashMap<&'a str, Value>;
}
