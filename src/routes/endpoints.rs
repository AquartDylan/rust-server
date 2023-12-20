use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

#[get("/")]
pub fn index() -> Json<&'static str> {
    return Json("hello world! This is to prove to joel that I can use rust to make an api")
}