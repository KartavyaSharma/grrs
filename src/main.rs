#![allow(unused)]

use clap::Parser;
use std::fs::File;
use std::io::{BufReader, Read, BufRead};

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
    let file = File::open(&args.path).expect("Could not read file!");
    let mut buf_reader = BufReader::new(file);
    
    for line in buf_reader.lines() {
        if line.as_ref().unwrap().contains(&args.pattern) {
            println!("{}", line.unwrap());
            return;
        }
    }
    eprintln!("No match found! Reached EOD.");
}
