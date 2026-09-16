use std::{collections::HashMap, path::{Path, PathBuf}, vec};
use pdf_extract::extract_text_by_pages;
use crate::token;

#[derive(Debug)]
struct Doc {
    loc: Loc,
    extension: String,
    words: HashMap<String, usize>,
}

#[derive(Debug)]
struct Loc {
    path: PathBuf,
    page: u32,
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

    let mut file_objects: Vec<Doc> = vec![];

    for path in files_path {
        println!("INFO: scanning file {:?} {}", path.as_path(), "#".repeat(5));
        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("unknown").to_string();

        match extension.as_str() {
            "pdf" => { file_objects.append(handle_pdf(path.as_path()).as_mut()); },
            _ => {
                println!("(x) Skipping not scannable type {}", extension);
            },
        }
    }

    println!("{:?}", file_objects)
}

fn get_tokens(text: String) -> HashMap<String, usize> {
    token::count_individual_token(
        token::tokenize(text)
    )
}

fn handle_pdf(path: &Path) -> Vec<Doc> {
    let mut file_objects: Vec<Doc> = vec![];

    let words_vector = match extract_text_by_pages(path){
        Ok(t) => t,
        Err(e) => {
            eprint!("ERROR in extracting text from pdf {:?}: {e}", path);
            return vec![];
        }
    };

    let mut page: u32 = 0;
    for w in words_vector {
        page = page + 1;
        let tokens_hashmap = get_tokens(w);
        file_objects.push(
            Doc {
                loc: Loc { path: path.to_path_buf(), page: page },
                extension: "pdf".to_string(),
                words: tokens_hashmap,
            }
        );
    }
    return file_objects;
}

fn recursive_read_directory(path: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let dire_files = path.read_dir()?;
    let mut files_path: Vec<PathBuf> = vec![];

    for file in dire_files{
        let f = match file {
            Ok(f) => { f },
            Err(e) => {
                eprintln!("ERROR could not read file : {}", e);
                println!("SKIPING IT!");
                continue;
            },
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
                eprintln!("ERROR could not determine file type for {:?}: {}", f.path(), e);
                println!("SKIPING IT!");
            }
        }
    }
    Ok(files_path)
}

fn help(){
    println!("help for scan:
    - scan <path>
    ")
}
