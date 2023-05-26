#[macro_use] extern crate rocket;

use simple_rest::{
    Db,
    routes::{
        index, 
        user
    }
};

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().expect(".env not loaded");
    rocket::build()
        .attach(Db::fairing())
        .mount("/", routes![index::index])
        .mount("/user", routes![user::get_user, user::save_user])
}