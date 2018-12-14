#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate rocket_contrib;
extern crate mylib;

use mylib::list_slides;

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
    let s = list_slides();
    let mut slides = HashMap::new();
    slides.insert("slides".to_string(), to_json(&s));
    Template::render("index", &slides)
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[post("/presentation", data = "<slide>")]
fn presentation(slide: Form<Slide>) -> Template {
    let input: Slide = slide.into_inner();
    let id: usize = input.slide_id - 1;
    let s = list_slides();
    let slide = &s[id];
    let mut slides = HashMap::new();
    slides.insert("title", &slide.title);
    slides.insert("description", &slide.description);
    slides.insert("style", &slide.style);
    slides.insert("file", &slide.file);
    Template::render("presentation", &slides)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, files, presentation])
    .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
