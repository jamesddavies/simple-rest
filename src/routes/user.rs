use rocket::{serde::json::Json};
use diesel::prelude::*;
use crate::Db;
use crate::schema::users;
use crate::models::{User,NewUser};

#[get("/<id>")]
pub async fn get_user(connection: Db, id: i32) -> Json<User> {
    connection.run(move |c| 
        users::table.filter(users::id.eq(id)).first(c)
    )
    .await
    .map(Json)
    .expect("User not found")
}

#[get("/save/<name>/<password>")]
pub async fn save_user(connection: Db, name: String, password: String) -> Json<User> {
    connection.run(move |c| {
        diesel::insert_into(users::table)
            .values(NewUser { username: name, password: password })
            .get_result(c)
    })
    .await
    .map(Json)
    .expect("Failed to add user")
}