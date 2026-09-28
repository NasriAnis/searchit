use crate::config::TFIDF_TO_WORD_PATH;
use crate::{serialization::DocTfIdf, tf_idf::compute, token};
use pdf_extract::extract_text_by_pages;
use html2text::from_read;
use std::io::prelude::*;
use std::fs::File;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    vec,
};

use crate::scanner::error_handling::ScanError;
use crate::scanner::structures::Doc;
use crate::scanner::structures::Loc;

pub fn extract_pdf_data(path: &Path) -> Result<Vec<Doc>, ScanError> {
    let mut file_objects: Vec<Doc> = vec![];
    let words_vector = extract_text_by_pages(path)?;

    for (page, w) in (0_u32..).zip(words_vector) {
        let tokens_hashmap = token::count_individual_token(token::tokenize(w));
        file_objects.push(Doc {
            loc: Loc {
                path: path.to_path_buf(),
                page,
            },
            extension: "pdf".to_string(),
            words: tokens_hashmap,
        });
    }
    Ok(file_objects)
}

pub fn extract_html_data(path: &Path) -> Result<Vec<Doc>, ScanError> {
    let mut f = File::open(path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    let words_string = from_read(buffer.as_bytes(), 20).unwrap(); // result fix
    let words_vec = token::count_individual_token(token::tokenize(words_string));
    let loc = Loc {
        path: path.to_path_buf(),
        page: 1,
    };
    let doc = Doc {
        loc: loc,
        extension: "html".to_string(),
        words: words_vec,
    };
    Ok(vec![doc])
}

pub fn recursive_read_directory(path: &Path) -> Result<Vec<PathBuf>, ScanError> {
    let dire_files = path.read_dir()?;
    let mut files_path: Vec<PathBuf> = vec![];

    for file in dire_files {
        let f = match file {
            Ok(f) => f,
            Err(e) => {
                eprintln!("ERROR could not read file : {}", e);
                println!("SKIPING IT!");
                continue;
            }
        };
        match f.file_type() {
            Ok(ft) => {
                if ft.is_symlink() {
                    continue;
                } else if ft.is_dir() {
                    let mut rf = recursive_read_directory(f.path().as_path())?;
                    files_path.append(&mut rf);
                } else {
                    files_path.push(f.path());
                }
            }
            Err(e) => {
                eprintln!(
                    "ERROR could not determine file type for {:?}: {}",
                    f.path(),
                    e
                );
                println!("SKIPING IT!");
            }
        }
    }
    Ok(files_path)
}

pub fn compute_tf_idf_wrapper(file_objects: Vec<Doc>) -> Result<(), ScanError> {
    let d_count = file_objects.len(); // number of documents

    let mut d_with_t: HashMap<String, usize> = HashMap::new(); // document frequency per term
    for doc in &file_objects {
        for (term, _count) in &doc.words {
            *d_with_t.entry(term.clone()).or_insert(0) += 1;
        }
    }
    for doc in file_objects {
        std::fs::create_dir_all(TFIDF_TO_WORD_PATH)?;
        let words_hashmap = doc.words;
        let w_count_in_d: usize = words_hashmap.values().sum(); // number of words in document
        let mut tf_to_word: HashMap<String, f64> = HashMap::new();

        for w in words_hashmap {
            let n_t_in_d = w.1; // count of term in document
            let term = w.0; // the term
            let d_with_t_value = d_with_t
                .get(&term)
                .expect("term seen in doc.words must be in d_with_t");
            let tf_idf = compute(
                n_t_in_d as f64,
                w_count_in_d as f64,
                d_count as f64,
                d_with_t_value.to_owned() as f64,
            );
            tf_to_word.insert(term, tf_idf);
        }
        let file_stem = doc
            .loc
            .path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        let file_name = format!("{}{}:{}.json", TFIDF_TO_WORD_PATH, file_stem, doc.loc.page);

        DocTfIdf {
            path: doc.loc.path.as_path().display().to_string(),
            page: doc.loc.page,
            extension: doc.extension,
            terms: tf_to_word,
        }.serialize(file_name)?;
    }
    Ok(())
}

pub fn help() {
    println!(
        "\
help for scan:
    - scan <path>           Scan a directory and compute TF-IDF for all PDFs found
    - scan <path> --help    Show this message"
    );
}
