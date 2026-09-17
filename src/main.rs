use std::env;

pub mod cli;
pub mod token;
pub mod tf_idf;
pub mod config;
pub mod serialization;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help()
    }
    else {
        match args[1].as_str() {
            "help" => { help() }
            "scan" => { cli::scan::run(args) },
            _ => {
                eprintln!("ERROR: this command doesnt exit {}", args[1]);
                eprintln!("EXITING!")
            }
        }
    }
}

fn help(){
    println!("Help section :
    - help <command> : get help for a specific command
    - scan <path> : scan a directory to prepare for searching
    ")
}
