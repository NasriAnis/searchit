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
            "pdf" => { file_objects.append(extract_pdf_data(path.as_path()).as_mut()); },
            _ => {
                println!("(x) Skipping not scannable type {}", extension);
            },
        }
    }

    match compute_tf_idf_wrapper(file_objects) {
        Ok(()) => { println!("INFO: succesfully saved term to tf mappings") },
        Err(e) => { println!("ERROR: error in tfidf computing {e}") }
    };
}

fn extract_pdf_data(path: &Path) -> Vec<Doc> {
    let mut file_objects: Vec<Doc> = vec![];

    let words_vector = match extract_text_by_pages(path){
        Ok(t) => t,
        Err(e) => {
            eprint!("ERROR in extracting text from pdf {:?}: {e}", path);
            return vec![];
        }
    };

    for (page, w) in (0_u32..).zip(words_vector) {
        let tokens_hashmap = get_tokens(w);
        file_objects.push(
            Doc {
                loc: Loc { path: path.to_path_buf(), page },
                extension: "pdf".to_string(),
                words: tokens_hashmap,
            }
        );
    }
    file_objects
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

fn compute_tf_idf_wrapper(file_objects: Vec<Doc>) -> Result<(), io::Error> {
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
        let mut tf_to_word: Vec<(String, f64)> = Vec::new();

        for w in words_hashmap{
            let n_t_in_d = w.1; // count of term in document
            let term = w.0; // the term
            let d_with_t_value = match d_with_t.get(&term){
                Some(t) => t,
                None => { panic!() }
            };
            let tf_idf = compute(n_t_in_d as f64, w_count_in_d as f64, d_count as f64, d_with_t_value.to_owned() as f64);
            tf_to_word.push((term, tf_idf))
        }
        let file_stem = doc.loc.path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown");
        let file_name = format!("{}{}:{}.txt",TFIDF_TO_WORD_PATH, file_stem, doc.loc.page);
        let file = File::create(file_name)?;
        let mut writer = BufWriter::new(file);

        let _ = writeln!(writer, "{}", doc.loc.path.as_path().display());
        for item in &tf_to_word { writeln!(writer, "{} : {}", item.0, item.1)?; }
    }
    Ok(())
}

fn get_tokens(text: String) -> HashMap<String, usize> {
    token::count_individual_token(
        token::tokenize(text)
    )
}

fn help(){
    println!("help for scan:
    - scan <path>
    ")
}
