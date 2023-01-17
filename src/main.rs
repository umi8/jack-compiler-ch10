use std::fs::File;
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::Parser;
use walkdir::{DirEntry, WalkDir};

use crate::compilation::compilation_engine::CompilationEngine;
use crate::compilation::compilation_engine::XmlCompilationEngine;
use crate::tokenizer::jack_tokenizer::JackTokenizer;

mod compilation;
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
    let args = Args::parse();
    let files: Vec<DirEntry> = extract_files_from(args.path.as_path());

    if files.is_empty() {
        println!(
            "The compilation target doesn't exist. Set a jack file or directory with jack files."
        );
        return Ok(());
    }

    for file in files {
        let mut output_file = File::create(create_output_file_name(file.path()))?;
        let mut engine = XmlCompilationEngine::new(JackTokenizer::new(file.path())?);
        engine.compile_class(&mut output_file)?;
    }

    Ok(())
}

fn extract_files_from(path: &Path) -> Vec<DirEntry> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(is_jack_file)
        .collect()
}

fn is_jack_file(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".jack"))
        .unwrap_or(false)
}

fn create_output_file_name(path: &Path) -> String {
    if path.is_file() && path.extension().unwrap() == "jack" {
        return String::from(path.with_extension("xml").to_string_lossy());
    }

    let dir = path.to_string_lossy();
    let file_name = path.file_name().unwrap().to_string_lossy();
    format!("{}/{}.xml", dir, file_name)
}
