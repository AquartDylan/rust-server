#[macro_use] extern crate rocket;

mod routes;
mod cors;

#[launch]
fn rocket() -> _ {
  rocket::build().attach(cors::cors::CORS).mount("/", routes![routes::endpoints::index])
}