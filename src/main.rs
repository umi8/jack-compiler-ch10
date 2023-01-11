use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::Instant;

use anyhow::{Context, Result};
use clap::Parser;

use tokenizer::jack_tokenizer::JackTokenizer;

use crate::tokenizer::token_type::TokenType;

mod tokenizer;

/// Jack Compiler
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Sets a source to be compiled. The source is a jack file or directory.
    #[arg(value_name = "SOURCE")]
    path: PathBuf,
}

fn main() -> Result<()> {
    // let args = Args::parse();
    // let file = File::open(args.path.as_path())?;
    let mut jack_tokenizer =
        JackTokenizer::new(Path::new("tests/resources/ExpressionLessSquare/Main.jack"))?;

    Ok(())
}
