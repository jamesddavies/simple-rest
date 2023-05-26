pub mod routes;
pub mod models;
pub mod schema;

use rocket_sync_db_pools::database;

#[database("simplerest")]
pub struct Db(diesel::PgConnection);