#[macro_use] extern crate rocket;

use simple_rest::{
    Db,
    routes::{
        index::index, 
        user::{get_user, save_user}
    }
};

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().expect(".env not loaded");
    rocket::build()
        .attach(Db::fairing())
        .mount("/", routes![index])
        .mount("/user", routes![get_user, save_user])
}