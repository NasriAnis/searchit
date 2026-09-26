use std::env;

pub mod scanner;
pub mod server;
pub mod search;
pub mod config;
pub mod serialization;
pub mod tf_idf;
pub mod token;
pub mod workers;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help()
    } else {
        match args[1].as_str() {
            "help" => help(),
            "scan" => match scanner::scan::run(args){
                Ok(_) => (),
                Err(e) => eprintln!("ERROR: scan {e}"),
            },
            "search" => {
                match search::run(args) {
                    Ok(td) => {
                        for (doc, score) in &td {
                            println!("{:?} at {:?} with score: {:.4}", doc.path, doc.page, score);
                        }
                    }
                    Err(e) => eprintln!("ERROR: {}", e),
                };
            }
            "serve" => {
                match server::serve::run() {
                    Ok(_) => (),
                    Err(e) => eprintln!("ERROR: {}", e),
                };
            }
            _ => {
                eprintln!("ERROR: this command doesnt exit {}", args[1]);
                eprintln!("EXITING!")
            }
        }
    }
}

fn help() {
    println!(
        "\
help section :
    - help <command> : get help for a specific command
    - scan <path> : scan a directory to prepare for searching
    - search <query> : search for relevent documents
    - serve : serve the web site
    "
    )
}
