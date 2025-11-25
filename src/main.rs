use std::process::exit;
use clap::Parser;
use relic::count_dir;

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
    let final_count: u64 = match count_dir(args.path, &args.ext) {
        Ok(f) => {f},
        Err(e) => {eprintln!("Error while counting: {e}");exit(1);},
    };
    
    println!("Lines counted: {:?}", final_count);
}


