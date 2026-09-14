use std::env;

pub mod cli;

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
