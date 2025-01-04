use rocket::*;

#[get("/")]
pub fn index() -> &'static str {
    "API v1 is up and operational!"
}