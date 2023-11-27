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
                        return println!("Test success: Database is found");
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
               return println!("Test success: Database not found");
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
        use std::{process::Command, process::Stdio, env};
        use std::io::{self, Write};

        let output = Command::new("cmd")
            .args([format!("{}\\backup", env::current_dir().unwrap().to_str().unwrap()), "dir"])
            .stdin(Stdio::piped())
            .output()
            .expect("Error: Problem in command");
        if output.status.success() {
            for i in output.stdout {
                let data = String::from
                if data == $nameDB{
                    return println!("Good");
                }
            }
            panic!("Error");
        } else {
            println!("Error in output");
        }
    };
}