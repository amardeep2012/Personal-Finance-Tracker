use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

/// Establishes a connection to the Postgres database.
pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
