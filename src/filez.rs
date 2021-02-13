extern crate askama;
extern crate chrono;

use std::fs::{self};
use std::path::{PathBuf};
use askama::Template;
use rocket::response::content::Html;
use rocket::response::{NamedFile};
use rocket::response::status::{NotFound};
use rocket::http::RawStr;
use chrono::offset::Utc;
use chrono::DateTime;

struct File {
    name: String,
    shortname: String,
    size: String,
    modified: String,
    // isdir: bool
}

#[derive(Template)]
#[template(path = "files.html")]
struct FileListTemplate<'a> {
    title: &'a str,
    language: &'a str,
    current_directory: &'a String,
    filez: &'a Vec<File>
}

fn bytes_to_readable(size: u64) -> String {
    let sz_float = size as f64;
    let kilobyte: f64 = 1000.0;
    let megabyte: f64 = kilobyte * 1000.0;
    let gigabyte: f64 = megabyte * 1000.0;
    let terabyte: f64 = gigabyte * 1000.0;
    let petabyte: f64 = terabyte * 1000.0;
    match sz_float {
        sz if sz < kilobyte => format!("{} bytes", sz),
        sz if sz < megabyte => format!("{:.2} kb", sz / kilobyte),
        sz if sz < gigabyte => format!("{:.2} mb", sz / megabyte),
        sz if sz < terabyte => format!("{:.2} gb", sz / gigabyte),
        sz if sz < petabyte => format!("{:.2} tb", sz / terabyte),
        _ => format!("gigantic")
    }
}

fn show_filez(location: String) -> NamedFile {
    let file = PathBuf::from(&location);
    NamedFile::open(&file).unwrap()
}

fn list_filez(location: String) -> Html<String> {
    let base_dir = PathBuf::from(".");
    let relative = &base_dir.join(&location);
    let paths = fs::read_dir(&relative).unwrap();
    let canonical_dir = fs::canonicalize(&relative).unwrap().display().to_string();
    let mut file_list: Vec<File> = Vec::new();
    let mut _p;
    let mut _metadata;
    let mut _file_size;
    let mut _datetime: DateTime<Utc>;

    for path in paths {
        _p = PathBuf::from(path.unwrap().path());
        _metadata = fs::metadata(&_p).unwrap();
        _file_size = _metadata.len();
        _datetime = _metadata.modified().unwrap().into();
        file_list.push(File {
            name: PathBuf::from(&_p).display().to_string(),
            shortname: String::from(PathBuf::from(&_p).strip_prefix(&canonical_dir).unwrap().to_str().unwrap()),
            size: bytes_to_readable(_file_size),
            modified: format!("{} UTC", _datetime.format("%m/%d/%Y %T")),
            // isdir: PathBuf::from(&_p).is_dir()
        })
    }

    let t = FileListTemplate{
        title: "Filez",
        language: "en",
        current_directory: &canonical_dir,
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

fn render_from_path(p: PathBuf) -> Result<Html<String>, Result<NamedFile, NotFound<&'static str>>> {
    let relative_string = fs::canonicalize(&p).expect("Cannot expand path");
    
    match &p.exists() & &p.is_dir() {
        true => {
            let file_list = list_filez(String::from(relative_string.to_str().unwrap()));
            Ok(file_list)
        },
        false => {
            match &p.exists() {
                true => {
                    let file_view = show_filez(String::from(relative_string.to_str().unwrap()));
                    Err(Ok(file_view))
                },
                false => Err(Err(NotFound("File or directory not found")))
            }
        }
    }
}

#[get("/")]
pub fn index() -> Result<Html<String>, Result<NamedFile, NotFound<&'static str>>> {
    let root_dir = PathBuf::from("./");
    render_from_path(root_dir)
}

#[get("/<relative_path>")]
pub fn filez(relative_path: &RawStr) -> Result<Html<String>, Result<NamedFile, NotFound<&'static str>>> {
    let root_dir = PathBuf::from("./");
    let url = String::from(&relative_path.to_string());
    let full_path = root_dir.join(&url);
    render_from_path(full_path)
}