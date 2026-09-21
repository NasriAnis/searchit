use crate::config::{MAX_WORKERS, TFIDF_TO_WORD_PATH};
use crate::{serialization::serialize_tfidf_to_word, tf_idf::compute, token};
use pdf_extract::extract_text_by_pages;
use std::fmt::Write;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, mpsc},
    thread, vec,
};

enum ScanJob {
    Pdf(PathBuf),
}

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
    let files_path = match recursive_read_directory(path) {
        Ok(t) => t,
        Err(e) => {
            eprint!("ERROR: recursive_read_directory() {}", e);
            println!("EXITTING NOW!");
            return;
        }
    };

    let file_objects_mutex: Arc<Mutex<Vec<Doc>>> = Arc::new(Mutex::new(vec![]));
    let (tx, handles) = workers(file_objects_mutex.clone());

    // feed jobs into the queue
    for path in files_path {
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("unknown")
            .to_string();

        match extension.as_str() {
            "pdf" => {
                match tx.send(ScanJob::Pdf(path)) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("ERROR: scan() at tx.send() {:?}", e);
                        continue;
                    }
                };
            }
            _ => {
                println!("(x) Skipping not scannable type {}", extension);
            }
        }
    }
    drop(tx); // closes the channel — workers exit their loop once queue drains

    for handle in handles {
        match handle.join() {
            Ok(_) => (),
            Err(e) => println!("ERROR: scan() with worker error {:?}", e),
        };
    }

    let file_objects: Vec<Doc> = match Arc::try_unwrap(file_objects_mutex)
        .expect("Arc still has multiple owners")
        .into_inner()
    {
        Ok(fo) => fo,
        Err(e) => {
            eprint!(
                "ERROR: scan() at collecting file objects from arc mutex {:?}",
                e
            );
            panic!()
        }
    };

    match compute_tf_idf_wrapper(file_objects) {
        Ok(()) => println!("INFO: succesfully saved term to tf mappings"),
        Err(e) => eprintln!("ERROR: scan() at compute_tf_idf_wrapper() {e}"),
    };
}

fn extract_pdf_data(path: &Path) -> Vec<Doc> {
    let mut file_objects: Vec<Doc> = vec![];

    let words_vector = match extract_text_by_pages(path) {
        Ok(t) => t,
        Err(e) => {
            eprint!("ERROR: scan() at extract_text_by_pages() {:?} {e}", path);
            return vec![];
        }
    };

    for (page, w) in (0_u32..).zip(words_vector) {
        let tokens_hashmap = get_tokens(w);
        file_objects.push(Doc {
            loc: Loc {
                path: path.to_path_buf(),
                page,
            },
            extension: "pdf".to_string(),
            words: tokens_hashmap,
        });
    }
    file_objects
}

fn recursive_read_directory(path: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
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
        let mut tf_to_word: HashMap<String, f64> = HashMap::new();

        for w in words_hashmap {
            let n_t_in_d = w.1; // count of term in document
            let term = w.0; // the term
            let d_with_t_value = match d_with_t.get(&term) {
                Some(t) => t,
                None => {
                    panic!()
                }
            };
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

        serialize_tfidf_to_word(
            doc.loc.path.as_path().display().to_string(),
            doc.loc.page,
            doc.extension,
            tf_to_word,
            file_name,
        )?; // fix
    }
    Ok(())
}

fn get_tokens(text: String) -> HashMap<String, usize> {
    token::count_individual_token(token::tokenize(text))
}

fn workers<T: Extend<Doc> + Send + 'static>(objects: Arc<Mutex<T>>) -> (Sender<ScanJob>, Vec<JoinHandle<()>>) {
    let (tx, rx) = mpsc::channel::<ScanJob>();
    let rx = Arc::new(Mutex::new(rx));

    let mut handles = vec![];

    for _ in 0..MAX_WORKERS {
        let rx = Arc::clone(&rx);
        let objects_mutex = Arc::clone(&objects);

        let handle = thread::spawn(move || {
            loop {
                // lock just long enough to grab one job, then release
                let job = {
                    match rx.lock() {
                        Ok(j) => j.recv(),
                        Err(e) => {
                            eprintln!("ERROR: scan() at rx.lock() {:?}", e);
                            continue;
                        }
                    }
                };
                match job {
                    Ok(ScanJob::Pdf(path)) => {
                        println!("INFO: scanning PDF {:?}", path);
                        let data = extract_pdf_data(path.as_path());
                        let mut file_obj = match objects_mutex.lock() {
                            Ok(fo) => fo,
                            Err(e) => {
                                eprintln!("ERROR: scan() at file_objects_mutex.lock() {:?}", e);
                                continue;
                            }
                        };
                        file_obj.extend(data);
                    }
                    Err(_) => break, // channel closed, no more jobs
                }
            }
        });
        handles.push(handle);
    }
    (tx, handles)
}

fn help() {
    println!(
        "\
help for scan:
    - scan <path>           Scan a directory and compute TF-IDF for all PDFs found
    - scan <path> --help    Show this message"
    );
}
