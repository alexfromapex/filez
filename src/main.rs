#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_include_static_resources;
extern crate rocket_contrib;
extern crate askama;

use std::fs::{self,Metadata};
use std::path::PathBuf;
use rocket_contrib::serve::StaticFiles;
use askama::Template;
use rocket::response::content::Html;
use rocket_include_static_resources::StaticResponse;

mod error_handlers;
mod favicon;

struct File {
    name: String,
    size: u64
}

#[derive(Template)]
#[template(path = "files.html")]
struct FileListTemplate<'a> {
    title: &'a str,
    language: &'a str,
    current_directory: &'a String,
    filez: &'a Vec<File>
}

#[get("/<url_path>")]
fn index(url_path: String) -> Html<String> {
    let mut relative = PathBuf::from(".");
    if url_path != "" {
        relative = PathBuf::from(".").join(&url_path);
    }
    let paths = fs::read_dir(&relative).unwrap();
    let dir = fs::canonicalize(&relative).unwrap().display().to_string();
    let mut file_list: Vec<File> = Vec::new();
    let mut _p;
    for path in paths {
        _p = path.unwrap();
        file_list.push(File {
            name: _p.path().display().to_string(),
            size: fs::metadata(_p.path()).unwrap().len()
        })
    }

    let t = FileListTemplate{
        title: "Files",
        language: "en",
        current_directory: &dir,
        filez: &file_list
    }.render();
    Html(t.unwrap())
}

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
        favicon::favicon,
        favicon::favicon_png,
        index
    ])
    .mount("/static", StaticFiles::from("../static"))
    .register(catchers![error_handlers::not_found])
    .launch();
}
