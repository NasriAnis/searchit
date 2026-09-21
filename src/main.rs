use std::env;

pub mod cmds;
pub mod config;
pub mod serialization;
pub mod tf_idf;
pub mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help()
    } else {
        match args[1].as_str() {
            "help" => help(),
            "scan" => cmds::scan::run(args),
            "search" => {
                let top_documents = cmds::search::run(args);
                for (doc, score) in &top_documents {
                    println!("{:?} at {:?} with score: {:.4}", doc.path, doc.page, score);
                }
            }
            "serve" => cmds::server::run(),
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
    "
    )
}
