#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate rocket_contrib;
extern crate mylib;

use mylib::get_slides;
use mylib::get_profile;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::handlebars::{to_json};

use rocket::response::NamedFile;
use rocket::response::Redirect;
use rocket_contrib::templates::{Template, handlebars};
use rocket::request::{Form};

#[derive(FromForm)]
struct Slide{
    path: String,
}

#[get("/")]
fn index() -> Template {
    let s = get_slides();
    let p = get_profile();
    let mut data = HashMap::new();
    data.insert("slides".to_string(), to_json(&s));
    data.insert("profile".to_string(), to_json(&p));
    Template::render("index", &data)
}

#[get("/<file..>", rank=3)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[post("/presentation", data = "<slide>")]
fn presentation(slide: Form<Slide>) -> Redirect {
    let path = slide.into_inner().path;
    Redirect::to(format!("/presentation/{}", path))
}

#[get("/presentation/<url>", rank=2)]
fn get_presentation(url: String) -> Template {
    let s = get_slides();
    let mut id: usize = 0;
    while id < s.len() {
        let u = &s[id].url.to_string();
        if u == url.trim() {
            break;
        }
        id += 1;
    }
    let s = &s[id];
    let mut slide = HashMap::new();
    slide.insert("title", &s.title);
    slide.insert("description", &s.description);
    slide.insert("theme", &s.theme);
    slide.insert("file", &s.file);
    Template::render("presentation", &slide)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, presentation, get_presentation, files])
    .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
