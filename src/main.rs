use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

/// Jack Compiler
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Sets a source to be compiled. The source is a jack file or directory.
    #[arg(value_name = "SOURCE")]
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file = File::open(args.path.as_path())?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
