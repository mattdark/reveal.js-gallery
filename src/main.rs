#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate rocket_contrib;
extern crate mylib;

use mylib::get_slides;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use handlebars::{to_json};

use rocket::response::NamedFile;
use rocket_contrib::templates::{Template, handlebars};
use rocket::request::{Form};

#[derive(FromForm)]
struct Slide{
    slide_id: usize,
}

#[get("/")]
fn index() -> Template {
    let s = get_slides();
    let mut slides = HashMap::new();
    slides.insert("slides".to_string(), to_json(&s));
    Template::render("index", &slides)
}

#[get("/<file..>", rank=3)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[post("/presentation", data = "<slide>")]
fn presentation(slide: Form<Slide>) -> Template {
    let input: Slide = slide.into_inner();
    let id: usize = input.slide_id - 1;
    let s = get_slides();
    let s = &s[id];
    let mut slide = HashMap::new();
    slide.insert("title", &s.title);
    slide.insert("description", &s.description);
    slide.insert("style", &s.style);
    slide.insert("file", &s.file);
    Template::render("presentation", &slide)
}

#[get("/presentation/<slide_id>", rank=2)]
fn get_presentation(slide_id: usize) -> Template {
    let id: usize = slide_id - 1;
    let s = get_slides();
    let s = &s[id];
    let mut slide = HashMap::new();
    slide.insert("title", &s.title);
    slide.insert("description", &s.description);
    slide.insert("style", &s.style);
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
