#[macro_use]
extern crate serde_derive;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Social {
    pub name: String,
    pub facebook: String,
    pub twitter: String,
    pub linkedin: String,
    pub github: String,
}

pub fn get_slides() -> Vec<Slides> {
    let file = File::open("static/slides.json").unwrap(); 
    let reader = BufReader::new(file);
    let slides: Vec<Slides> = serde_json::from_reader(reader).unwrap();
    slides
}

pub fn get_social() -> Vec<Social> {
    let file = File::open("static/data/contact.json").unwrap();
    let reader = BufReader::new(file);
    let social: Vec<Social> = serde_json::from_reader(reader).unwrap();
    social
}
