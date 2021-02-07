use rocket_include_static_resources::StaticResponse;

#[get("/favicon.ico")]
pub fn favicon() -> StaticResponse {
    static_response!("favicon")
}

#[get("/favicon-16.png")]
pub fn favicon_png() -> StaticResponse {
    static_response!("favicon-png")
}