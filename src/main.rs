#![allow(unused)]

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("pattern: {}, path: {}", args.pattern, args.path.display());
    let content = std::fs::read_to_string(&args.path).expect("Could not read file!");
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
