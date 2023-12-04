pub use crate::types::TypeSGBD;

// Macro to check that database exist
#[macro_export]
macro_rules! assert_db_exist {
    ($typeSGBD:expr, $recent_db:expr, $url:expr) => {
        match $typeSGBD {
             TypeSGBD::Mysql => {
                let databases = database::show_db_mysql(&$url)
                .await
                .unwrap();

                for i in databases {
                    if i == $recent_db {
                        println!("Test success: Database is found");
                        return;
                    }
                }
                panic!("Test failed: Database not found");
            },
            TypeSGBD::Postgres => {
                 println!("No Ready");
             }
        }
    };
}

// Macro to check that database is dropped 
#[macro_export]
macro_rules! assert_db_dropped {
    ($typeSGBD:expr, $nameDB:expr, $url:expr) => {
        match $typeSGBD {
           TypeSGBD::Mysql => {
                let databases = database::show_db_mysql(&$url)
                .await
                .unwrap();

               for i in databases {
                    if i == $nameDB {
                        panic!("Test failed: Database is found");
                    }
                }
               println!("Test success: Database not found");
               return;
           },
            TypeSGBD::Postgres => {
               println!("No Ready")
           }
        }
    };
}

// Macro to check that backup's file exist really
#[macro_export]
macro_rules! assert_backup {
    ($nameDB:expr) => {
        use std::{fs, env};
        
        let directory_1 = format!("{}\\backup", env::current_dir().unwrap().to_str().unwrap());
        let directory_2 = format!("{}\\backup\\{}", env::current_dir().unwrap().to_str().unwrap(), $nameDB);

        for entry in fs::read_dir(directory_1).unwrap() {
            let data = entry.unwrap().path();
            if  data.to_str() == Some(&directory_2) {
                println!("Success");
                return;
            }
        }
        panic!("Error: database name don't found");
    };
}
