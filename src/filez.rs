extern crate askama;

use std::fs::{self};
use std::path::{PathBuf};
use askama::Template;
use rocket::response::content::Html;
use rocket::response::{NamedFile};
use rocket::http::RawStr;

struct File {
    name: String,
    shortname: String,
    size: u64,
    isdir: bool
}

#[derive(Template)]
#[template(path = "files.html")]
struct FileListTemplate<'a> {
    title: &'a str,
    language: &'a str,
    current_directory: &'a String,
    filez: &'a Vec<File>
}

fn show_filez(location: String) -> NamedFile {
    let file = PathBuf::from(&location);
    NamedFile::open(&file).unwrap()
}

fn list_filez(location: String) -> Html<String> {
    let base_dir = PathBuf::from(".");
    let relative = &base_dir.join(&location);
    let paths = fs::read_dir(&relative).unwrap();
    let dir = fs::canonicalize(&relative).unwrap().display().to_string();
    let mut file_list: Vec<File> = Vec::new();
    let mut _p;

    for path in paths {
        _p = PathBuf::from(path.unwrap().path());
        file_list.push(File {
            name: PathBuf::from(&_p).display().to_string(),
            shortname: String::from(PathBuf::from(&_p).strip_prefix(&dir).unwrap().to_str().unwrap()),
            size: fs::metadata(&_p).unwrap().len(),
            isdir: PathBuf::from(&_p).is_dir()
        })
    }

    let t = FileListTemplate{
        title: "Filez",
        language: "en",
        current_directory: &dir,
        filez: &file_list
    };
    let rendered_page = t.render();
    match rendered_page {
        Ok(s) => Html(s),
        Err(_) => {
            Html(String::from(""))
        }
    }
}

// TODO: Need to accept both return types
fn render_from_path(p: PathBuf) -> Result<Html<String>, NamedFile> {
    let relative_string = fs::canonicalize(&p).expect("Cannot expand path");
    
    match &p.is_dir() {
        true => {
            let file_list = list_filez(String::from(relative_string.to_str().unwrap()));
            Ok(file_list)
        },
        false => {
            let file_view = show_filez(String::from(relative_string.to_str().unwrap()));
            Err(file_view)
        }
    }
}

#[get("/")]
pub fn index() -> Result<Html<String>, NamedFile> {
    let root_dir = PathBuf::from("./");
    render_from_path(root_dir)
}

#[get("/<relative_path>")]
pub fn filez(relative_path: &RawStr) -> Result<Html<String>, NamedFile> {
    let root_dir = PathBuf::from("./");
    let url = String::from(&relative_path.to_string());
    let full_path = root_dir.join(&url);
    render_from_path(full_path)
}