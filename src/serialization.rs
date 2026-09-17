use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufWriter, Write},
};

#[derive(Serialize, Deserialize, Debug)]
struct DocTfIdf {
    path: String,
    page: u32,
    extension: String,
    terms: HashMap<String, f64>,
}

pub fn serialize_tfidf_to_word(
    path: String,
    page: u32,
    extension: String,
    terms: HashMap<String, f64>,
    file_name: String,
) -> Result<(), io::Error> {
    let file = File::create(file_name)?;
    let mut writer = BufWriter::new(file);
    let data = DocTfIdf {
        path,
        page,
        extension,
        terms,
    };
    // impl: check if files already exit and handle that
    let serialized = serde_json::to_string(&data).unwrap(); // fix
    let _ = writeln!(writer, "{}", serialized); // fix
    Ok(())
}
