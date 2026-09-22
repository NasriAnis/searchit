use crate::config::{MAX_WORKERS, TFIDF_TO_WORD_PATH};
use crate::workers::{PoolError, spawn_worker_pool};
use crate::{serialization::serialize_tfidf_to_word, tf_idf::compute, token};
use pdf_extract::{OutputError, extract_text_by_pages};
use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
    vec,
};

#[derive(Debug)]
enum ScanError {
    Io(std::io::Error),
    PdfExtract(pdf_extract::OutputError),
    LockPoisoned,
}

impl From<std::io::Error> for ScanError {
    fn from(e: std::io::Error) -> Self {
        ScanError::Io(e)
    }
}
impl From<OutputError> for ScanError {
    fn from(e: OutputError) -> Self {
        ScanError::PdfExtract(e)
    }
}
impl From<PoolError> for ScanError {
    fn from(e: PoolError) -> Self {
        match e {
            PoolError::LockPoisoned => ScanError::LockPoisoned,
        }
    }
}

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
        return;
    }
    let path = Path::new(&args[2]);

    println!("INFO: reading directory");
    let files_path = match recursive_read_directory(path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("ERROR: recursive_read_directory() {}", e);
            println!("EXITTING NOW!");
            return;
        }
    };

    let (job_tx, results_rx, handles) = spawn_worker_pool(
        MAX_WORKERS,
        |job: ScanJob| -> Result<Vec<Doc>, ScanError> {
            match job {
                ScanJob::Pdf(path) => extract_pdf_data(&path),
            }
        },
    );

    for path in files_path {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("unknown");
        println!("INFO: incoming scan of {path:?}");
        match ext {
            "pdf" => {
                if job_tx.send(ScanJob::Pdf(path)).is_err() {
                    eprintln!("FATAL: worker pool is dead, aborting scan");
                    break; // no point sending more, nobody will receive them
                }
            }
            _ => println!("(x) Skipping not scannable type {}", ext),
        }
    }
    drop(job_tx); // no more jobs — lets workers exit once the queue drains

    let mut file_objects: Vec<Doc> = vec![];
    let mut failures: Vec<ScanError> = vec![];
    for result in results_rx {
        match result {
            Ok(mut docs) => file_objects.append(&mut docs),
            Err(e) => {
                eprintln!("ERROR: worker failed: {:?}", e);
                failures.push(e);
            }
        }
    }
    if !failures.is_empty() {
        eprintln!(
            "WARN: {} job(s) failed during scan (see above for details)",
            failures.len()
        );
    }

    for handle in handles {
        if let Err(panic) = handle.join() {
            eprintln!("ERROR: worker thread panicked: {:?}", panic);
        }
    }

    match compute_tf_idf_wrapper(file_objects) {
        Ok(()) => println!("INFO: succesfully saved term to tf mappings"),
        Err(e) => eprintln!("ERROR: scan() at compute_tf_idf_wrapper() {e}"),
    };
}

fn extract_pdf_data(path: &Path) -> Result<Vec<Doc>, ScanError> {
    let mut file_objects: Vec<Doc> = vec![];
    let words_vector = extract_text_by_pages(path)?;

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
    Ok(file_objects)
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

        serialize_tfidf_to_word(
            doc.loc.path.as_path().display().to_string(),
            doc.loc.page,
            doc.extension,
            tf_to_word,
            file_name,
        )?;
    }
    Ok(())
}

fn get_tokens(text: String) -> HashMap<String, usize> {
    token::count_individual_token(token::tokenize(text))
}

fn help() {
    println!(
        "\
help for scan:
    - scan <path>           Scan a directory and compute TF-IDF for all PDFs found
    - scan <path> --help    Show this message"
    );
}
