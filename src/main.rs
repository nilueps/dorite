use std::path::PathBuf;

use clap::Parser;

// Define the command line arguments
#[derive(Parser)]
struct Args {
    #[arg()]
    file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    if let Some(file_path) = args.file {
        // Read file content and display it
        let content = std::fs::read_to_string(file_path).unwrap();
        println!("{}", content);
    }
}
