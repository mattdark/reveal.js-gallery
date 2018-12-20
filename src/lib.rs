#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
pub struct Slides {
    pub id: String,
    pub file: String,
    pub title: String,
    pub description: String,
    pub style: String,
    pub url: String,
}

pub fn get_slides() -> Vec<Slides> {
    let file = File::open("static/slides.json").unwrap(); 
    let reader = BufReader::new(file);
    let slides: Vec<Slides> = serde_json::from_reader(reader).unwrap();
    slides
}
