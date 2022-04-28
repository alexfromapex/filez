extern crate rocket;
use rocket::Request;

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Invalid path: '{}'", req.uri())
}

#[catch(500)]
pub fn internal_error() -> &'static str {
    "Internal server error"
}
