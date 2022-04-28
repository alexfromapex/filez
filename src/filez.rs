extern crate askama;
extern crate chrono;

use std::fs::{self};
use std::path::{PathBuf};

use askama_rocket::Template;

use rocket::response::content::Html;
use rocket::fs::NamedFile;
use rocket::response::status::{NotFound};

use chrono::offset::Utc;
use chrono::DateTime;

struct File {
    name: String,
    shortname: String,
    size: String,
    modified: String,
    // isdir: bool
}

#[derive(Responder)]
pub enum FileReturnType {
    LIST(Html<String>),
    FILE(NamedFile)
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

async fn show_filez(location: String) -> Option<NamedFile> {
    let file = PathBuf::from(&location);
    NamedFile::open(PathBuf::from(".").join(&file)).await.ok()
}

fn list_filez(location: String) -> Html<String> {
    let base_dir = fs::canonicalize(PathBuf::from(".")).unwrap();
    let fully_qualified = fs::canonicalize(&location).unwrap().display().to_string();
    let paths = fs::read_dir(&fully_qualified).unwrap();
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
            shortname: format!("/{}", PathBuf::from(&_p).strip_prefix(&base_dir).unwrap().display().to_string()),
            size: bytes_to_readable(_file_size),
            modified: format!("{} UTC", _datetime.format("%m/%d/%Y %T")),
            // isdir: PathBuf::from(&_p).is_dir()
        })
    }

    let t = FileListTemplate{
        title: "Filez",
        language: "en",
        current_directory: &fully_qualified,
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

async fn render_from_path(p: PathBuf) -> Result<FileReturnType, NotFound<&'static str>> {
    let err_msg = format!("Cannot expand path {}", &p.to_str().unwrap());
    let relative_string = fs::canonicalize(&p).expect(&err_msg);

    let result: FileReturnType;
    
    match &p.exists() & &p.is_dir() {
        true => {
            let file_list = list_filez(String::from(relative_string.to_str().unwrap()));
            result = FileReturnType::LIST(file_list);
            Ok(result)
        },
        false => {
            match &p.exists() {
                true => {
                    let file = show_filez(String::from(relative_string.to_str().unwrap())).await.unwrap();
                    result = FileReturnType::FILE(file);
                    Ok(result)
                },
                false => {
                    let not_found = NotFound("File or directory not found");
                    Err(not_found)
                }
            }
        }
    }
}

#[get("/")]
pub async fn index() -> Result<FileReturnType, NotFound<&'static str>> {
    let root_dir = PathBuf::from(".");

    let rendered = render_from_path(root_dir).await;

    match rendered {
        Ok(value) => Ok(value),
        Err(value) => Err(value)
    }
}

#[get("/<relative_path..>")]
pub async fn filez(relative_path: PathBuf) -> Result<FileReturnType, NotFound<&'static str>> {
    let root_dir = PathBuf::from(".").canonicalize().unwrap();
    let full_path = root_dir.join(&relative_path.to_str().unwrap());

    let rendered = render_from_path(full_path).await;

    match rendered {
        Ok(result) => Ok(result),
        Err(not_found) => Err(not_found)
    }
}