pub mod commands;
pub mod settings;
pub mod api;
pub mod state;
pub mod model;
pub mod schema;
pub mod db;

// // Establish database connection
// use diesel::prelude::*;
// use dotenvy::dotenv;
// use std::env;

// pub fn establish_connection() -> SqliteConnection {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     SqliteConnection::establish(&database_url)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
// }
