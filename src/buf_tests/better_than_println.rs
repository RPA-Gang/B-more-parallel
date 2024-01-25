use anyhow::{Context, Result};
use clap::Parser;

use std::io::{self, Write};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()> {
    let args = Cli::parse();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("\nCouldn't read file `{}`", &args.path.display()))?;
    // println!("file content: {}", content);

    println!("Println macro, but before we start writeln buffer");

    writeln!(handle, "Content in writeln is: \n\n{}", content).unwrap();

    writeln!(handle, "Some other text").unwrap();
    writeln!(handle, "Some other text").unwrap();
    writeln!(handle, "Some other text").unwrap();
    writeln!(handle, "Some other text").unwrap();
    writeln!(handle, "Some other text").unwrap();

    println!("Println macro, before FLISH but after buffer write in");

    handle.flush().unwrap();

    Ok(())
}
