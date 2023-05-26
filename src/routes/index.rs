use rocket::{serde::json::Json};
use crate::models::{StatusCheck};

#[get("/")]
pub fn index() -> Json<StatusCheck> {
    Json(
        StatusCheck { 
            status: "OK".to_string(),
            message: "App is healthy.".to_string()
        }
    )
}