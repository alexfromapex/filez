#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_include_static_resources;
extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;
use rocket_include_static_resources::StaticResponse;

mod error_handlers;
mod favicon;
mod filez;

fn main() {
    rocket::ignite()
    .attach(StaticResponse::fairing(|resources| {
        static_resources_initialize!(
            resources,
            "favicon", "/Users/macbook/Code/Rust/filez/static/favicon/favicon.ico",
            "favicon-png", "/Users/macbook/Code/Rust/filez/static/favicon/favicon-16x16.png"
        );
    }))
    .mount("/", routes![
        filez::index,
        filez::filez,
        favicon::favicon,
        favicon::favicon_png,
    ])
    .mount("/static", StaticFiles::from("../static"))
    .register(catchers![error_handlers::not_found])
    .launch();
}
