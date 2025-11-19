use std::{fs, io::{self, Read}, process::exit};
use clap::Parser;

// Simple line counter program
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    
    // Path to the file/folder
    #[arg(short, long)]
    path: String,

    // Specifies the file extensions, by default all files are scanned
    #[arg(short, long)]
    ext: Option<String>,
}

fn main(){
    let args = Args::parse();
    let final_count: u64;

    if std::path::Path::new(&args.path).is_dir() {
        final_count = match count_dir(args.path, &args.ext) {
            Ok(f) => {f},
            Err(e) => {
                eprintln!("Error reading directory {}", e.to_string());
                0
            },
        };
    } else {
        final_count = count_lines(args.path, args.ext)
    }
    
    println!("Lines counted: {:?}", final_count);
}

fn count_dir(_path: String, ext: &Option<String>) -> Result<u64, io::Error> {
    let mut count: u64 = 0;

    let dir = std::path::Path::new(&_path);
    if dir.is_dir() { 
       let paths = fs::read_dir(_path)?
           .map(|res| res.map(|e| e.path()))
           .collect::<Result<Vec<_>, io::Error>>()?;
       for path in paths {
            let path_as_str: String = match path.to_str() {
                Some(p) => {p.to_string()},
                None => {todo!()}
            };

           if std::path::Path::new(&path).is_dir() {
               count += match count_dir(path_as_str, &ext) {
                   Ok(c) => {c},
                   Err(e) => {return Err(e);},
               };
           } else {
               count += count_lines(path_as_str, ext.clone());
           }
       }
       Ok(count)
    } else {
        eprintln!("given path is not a directory!");
        exit(1);
    }
}

fn ext_compar(file: &String, ext: String) -> bool {
    let spl: Vec<&str> = file.split('.').collect();
    let last: String = match spl.last() {
        Some(l) => { 
            l.to_string()
        },
        None => {
            return false
        },
    };

    if last == ext {
        return true;
    }
    return false;
}

fn count_lines(_path: String, ext: Option<String>) -> u64 {
    match ext {
        Some(ex) => {
            if !ext_compar(&_path, ex) {
                return 0;
            }
        },
        None => {},
    }

    let mut fp: fs::File = fs::File::open(_path).unwrap();
    let mut contents: String = String::new();
    fp.read_to_string(&mut contents).expect("Unable to read from file");

    let mut count: u64 = 0;
    let mut split_lines: Vec<String> = vec![];
    for line in contents.lines() {
        split_lines.push(String::from(line));
        count += 1;
    }
    count
}

