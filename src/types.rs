pub const BIND_LIMIT: usize = 65535;

//Data Structure for database
pub struct Database {
    pub name_db: String
}

impl Database {
    pub fn new(name_db: String) -> Database {
        Database{name_db}
    }
}

