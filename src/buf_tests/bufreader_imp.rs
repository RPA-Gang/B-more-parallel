use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    // Look for this
    // #[arg(short = 'i', long = "input-pattern")]
    pattern: String,

    // in this file
    // #[arg(short = 'f', long = "file")]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let f = File::open(&args.path).unwrap();

    let reader = BufReader::new(f);

    reader.lines().for_each(|line| {
        let line = line.unwrap_or(String::from("That line was invalid"));
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    });
}
