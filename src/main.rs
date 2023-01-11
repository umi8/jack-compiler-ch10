use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use crate::tokenizer::token_type::TokenType;
use tokenizer::jack_tokenizer::JackTokenizer;

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
    let mut jack_tokenizer = JackTokenizer::new(args.path.as_path())?;

    while jack_tokenizer.has_more_tokens()? {
        jack_tokenizer.advance()?;
        match jack_tokenizer.token_type()? {
            TokenType::Keyword => {
                println!("<keyword> {} </keyword>", jack_tokenizer.value()?)
            }
            TokenType::Symbol => {
                println!("<symbol> {} </symbol>", jack_tokenizer.value()?)
            }
            TokenType::Identifier => {
                println!("<identifier> {} </identifier>", jack_tokenizer.value()?)
            }
            TokenType::IntConst => {
                println!(
                    "<integerConstant> {} </integerConstant>",
                    jack_tokenizer.value()?
                )
            }
            TokenType::StringConst => {
                println!(
                    "<stringConstant> {} </stringConstant>",
                    jack_tokenizer.value()?
                )
            }
        }
    }
    Ok(())
}
