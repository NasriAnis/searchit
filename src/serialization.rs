use serde::{Deserialize, Serialize};
use std::io::Read;

use crate::config::TFIDF_TO_WORD_PATH;

use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufWriter, Write},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DocTfIdf {
    pub path: String,
    pub page: u32,
    pub extension: String,
    pub terms: HashMap<String, f64>,
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
    let serialized = serde_json::to_string(&data)?;
    writeln!(writer, "{}", serialized)?;
    Ok(())
}

pub fn deserialize_tfidf_to_word() -> Result<Vec<DocTfIdf>, io::Error> {
    let mut vec_deserialized: Vec<DocTfIdf> = Vec::new();
    let path = TFIDF_TO_WORD_PATH;
    let entries = std::fs::read_dir(path)?;
    for entry in entries {
        let e = entry?;
        let mut f = std::fs::File::open(e.path())?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        vec_deserialized.push(serde_json::from_str(&buffer)?);
    }
    Ok(vec_deserialized)
}
