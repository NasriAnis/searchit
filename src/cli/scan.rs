use std::{collections::HashMap, path::{Path, PathBuf}, vec};
use pdf_extract::extract_text_by_pages;
use crate::token;

#[derive(Debug)]
struct File {
    path: PathBuf,
    extension: String,
    words: Vec<HashMap<String, usize>>,
}


pub fn run(args: Vec<String>) {
    if args.len() < 3 {
        help();
    }
    let path = Path::new(&args[2]);

    println!("INFO: reading directory");
    let files_path = match recursive_read_directory(path){
        Ok(t) => t,
        Err(e) => {
            eprint!("ERROR at recursive_read_directory() : {}", e);
            println!("EXITTING NOW!");
            return
        }
    };

    let mut file_objects: Vec<File> = vec![];

    for path in files_path {
        println!("INFO: scanning file {:?} {}", path.as_path(), "#".repeat(5));
        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("unknown").to_string();
        let mut tokens_hashmap_vector: Vec<HashMap<String, usize>> = Vec::new();

        match extension.as_str() {
            "pdf" => {
                match handle_pdf(path.as_path()) {
                    Some(words_vector) => {
                        for w in words_vector {
                            tokens_hashmap_vector.push(get_tokens(w));
                        }
                    },
                    None => { continue; },
                };

            },
            _ => {
                println!("(x) Skipping not scannable type {}", extension);
            },
        }
        file_objects.push(
            File {
                path,
                extension,
                words: tokens_hashmap_vector,
            }
        );
    }

    println!("{:?}", file_objects)
}

fn get_tokens(text: String) -> HashMap<String, usize> {
    token::count_individual_token(
        token::tokenize(text)
    )
}

fn handle_pdf(path: &Path) -> Option<Vec<String>> {
    match extract_text_by_pages(path){
        Ok(t) => Some(t),
        Err(e) => {
            eprint!("ERROR in extracting text from pdf {:?}: {e}", path);
            None
        }
    }
}

fn recursive_read_directory(path: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let dire_files = path.read_dir()?;
    let mut files_path: Vec<PathBuf> = vec![];

    for file in dire_files{
        match file {
            Ok(f) => {
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
                        eprintln!("ERROR could not determine file type for {:?}: {}", f.path(), e);
                        println!("SKIPING IT!");
                    }
                }
            },
            Err(e) => {
                eprintln!("ERROR could not read file : {}", e);
                println!("SKIPING IT!")
            },
        }
    }

    Ok(files_path)
}

fn help(){
    println!("help for scan:
    - scan <path>
    ")
}
