use std::{fs, io::Read};

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
    let lines: Vec<String> = open_file(args.path); 
    let line_amt = count_lines(lines);
    println!("Lines counted: {:?}", line_amt);
}

fn open_file(path: String) -> Vec<String> {
    let mut fp: fs::File = fs::File::open(path).expect("Unable to open file");
    let mut contents: String = String::new();
    fp.read_to_string(&mut contents).expect("Unable to read from file");

    let mut split_lines: Vec<String> = vec![];
    for line in contents.lines() {
        split_lines.push(String::from(line));
    }
    split_lines
}

fn count_lines(lines: Vec<String>) -> usize {
    lines.len()
}
