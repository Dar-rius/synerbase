//Bind limit for mysql and postgres
pub const BIND_LIMIT: usize = 65535;

pub enum TypeSGBD {
    Mysql,
    Postgres
}

//Data Structure for database
pub struct Database {
    pub name_db: String
}

pub struct Column{
    pub column_name: Vec<String>,
    pub type_column: Vec<String>
}

pub struct Table {
    pub name_tb: String,
    pub column: Column,
    pub row: u32
}

impl Database {
    pub fn new(name_db: String) -> Database {
        Database{name_db}
    }
}