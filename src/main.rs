use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use crate::jack_tokenizer::JackTokenizer;

mod jack_tokenizer;

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
    let mut jack_tokenizer = JackTokenizer::new(args.path.as_path())?;

    while jack_tokenizer.has_more_tokens()? {
        jack_tokenizer.advance()?;
    }

    Ok(())
}
