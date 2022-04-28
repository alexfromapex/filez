#[macro_use] extern crate rocket;

use rocket::fs::FileServer;

mod error_handlers;
mod filez;

#[launch]
fn rocket() -> _ {

    rocket::build()
    .mount("/", routes![
        filez::index,
        filez::filez,
    ])
    .mount("/static", FileServer::from("static"))
    .register("/", catchers![error_handlers::internal_error,error_handlers::not_found])
}
