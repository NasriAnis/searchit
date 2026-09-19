use serde::{Deserialize, Serialize};
use std::io::Read;

use crate::config::TFIDF_TO_WORD_PATH;

use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufWriter, Write},
};

#[derive(Serialize, Deserialize, Debug)]
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
    let serialized = serde_json::to_string(&data).unwrap(); // fix
    let _ = writeln!(writer, "{}", serialized); // fix
    Ok(())
}

pub fn deserialize_tfidf_to_word() -> Vec<DocTfIdf> {
    let mut vec_deserialized: Vec<DocTfIdf> = Vec::new();
    let path = TFIDF_TO_WORD_PATH;
    let entries = std::fs::read_dir(path).unwrap(); // fix
    for entry in entries {
        match entry {
            Ok(e) => {
                match std::fs::File::open(e.path()) {
                    Ok(mut file) => {
                        let mut buffer = String::new();
                        match file.read_to_string(&mut buffer) {
                            Ok(_sz) => {
                                vec_deserialized.push(serde_json::from_str(&buffer).unwrap());
                            }
                            Err(_) => {
                                continue;
                            } // fix
                        }
                    }
                    Err(_) => {
                        continue;
                    } // fix
                }
            }
            Err(_) => {
                continue;
            } // fix
        }
    }
    vec_deserialized
}
