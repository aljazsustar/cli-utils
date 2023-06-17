use std::{path::Path, fs};

use lopdf::Document;

pub fn get_docs_from_dir(path: &Path) -> Vec<Document> {
    let mut docs = Vec::new();
    println!("{}", path.exists());
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            if entry.as_ref().unwrap().file_type().unwrap().is_file() {
                println!("{}", entry.as_ref().unwrap().file_name().to_str().unwrap());
                docs.push(Document::load(entry.unwrap().path()).unwrap());
            }
        }
    }
    docs
}