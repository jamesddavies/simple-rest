mod routes;
mod models;
mod schema;

use crate::routes::index::index;
use crate::routes::user::{get_user, save_user};
use rocket_sync_db_pools::database;

#[macro_use] extern crate rocket;

#[database("simplerest")]
pub struct Db(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().expect(".env not loaded");
    rocket::build()
        .attach(Db::fairing())
        .mount("/", routes![index])
        .mount("/user", routes![get_user, save_user])
}