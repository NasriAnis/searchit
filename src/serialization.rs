use serde::{Deserialize, Serialize};
use std::io::Read;

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

impl DocTfIdf {
    pub fn serialize(self, file_name: String) -> Result<(), io::Error> {
        let file = File::create(file_name)?;
        let mut writer = BufWriter::new(file);
        // impl: check if files already exit and handle that
        let serialized = serde_json::to_string(&self)?;
        writeln!(writer, "{}", serialized)?;
        Ok(())
    }

    pub fn deserialize(path: &str) -> Result<Vec<DocTfIdf>, io::Error> {
        let mut vec_deserialized: Vec<DocTfIdf> = Vec::new();
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
}
