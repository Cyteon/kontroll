#[macro_use] use rocket::*;

pub mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/v1", routes![routes::v1::index])
}
