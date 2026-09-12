use std::{path::{Path, PathBuf}};

struct FileInfo{
    path: PathBuf,
    extension: String,
}

pub fn run(args: Vec<String>) {
    if args.len() < 3 {
        help();
    }
    let path = Path::new(&args[2]);

    let files = match recursive_read_directory(path){
        Ok(t) => t,
        Err(e) => {
            eprint!("ERROR at recursive_read_directory() : {}", e);
            println!("EXITTING NOW!");
            return
        }
    };

    for file in files {
        println!("File : {:?} of type {:?}", file.path, file.extension);
    }
}

fn recursive_read_directory(path: &Path) -> Result<Vec<FileInfo>, std::io::Error> {
    let dire_files = path.read_dir()?;
    let mut files: Vec<FileInfo> = vec![];
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
                            files.append(&mut rf);
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


    for path in files_path {
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("unknown").to_string();
        files.push(
            FileInfo{
                path,
                extension,
            }
        )
    }

    Ok(files)
}

fn help(){
    println!("help for scan:
    - scan <path>
    ")
}
