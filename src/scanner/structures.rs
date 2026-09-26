use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct Doc {
    pub loc: Loc,
    pub extension: String,
    pub words: HashMap<String, usize>,
}

#[derive(Debug)]
pub struct Loc {
    pub path: PathBuf,
    pub page: u32,
}
