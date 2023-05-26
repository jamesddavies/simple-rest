use diesel::{Insertable, Queryable};
use serde::{Serialize, Deserialize};
use crate::schema::users;

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct StatusCheck {
    pub status: String,
    pub message: String
}