use crate::config::MAX_WORKERS;
use crate::workers::spawn_worker_pool;
use std::{path::Path, path::PathBuf, vec};

use crate::cmds::scan::error_handling::ScanError;
use crate::cmds::scan::structures::Doc;
use crate::cmds::scan::wrappers;

enum ScanJob {
    Pdf(PathBuf),
}

pub fn run(args: Vec<String>) {
    if args.len() < 3 {
        wrappers::help();
        return;
    }
    let path = Path::new(&args[2]);

    println!("INFO: reading directory");
    let files_path = match wrappers::recursive_read_directory(path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("ERROR: recursive_read_directory() {}", e);
            println!("EXITTING NOW!");
            return;
        }
    };

    let (job_tx, results_rx, handles) =
        spawn_worker_pool(MAX_WORKERS, |job: ScanJob| -> Result<Vec<Doc>, ScanError> {
            match job {
                ScanJob::Pdf(path) => wrappers::extract_pdf_data(&path),
            }
        });

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

    match wrappers::compute_tf_idf_wrapper(file_objects) {
        Ok(()) => println!("INFO: succesfully saved term to tf mappings"),
        Err(e) => eprintln!("ERROR: scan() at compute_tf_idf_wrapper() {e}"),
    };
}
