use anyhow::{bail, Error, Result};

use crate::{JackTokenizer, TokenType};

trait CompilationEngine {
    fn new(tokenizer: JackTokenizer) -> Self;
    fn compile_class(&mut self) -> Result<()>;
}

struct XmlCompilationEngine {
    tokenizer: JackTokenizer,
}

impl CompilationEngine for XmlCompilationEngine {
    fn new(tokenizer: JackTokenizer) -> Self {
        XmlCompilationEngine { tokenizer }
    }

    fn compile_class(&mut self) -> Result<()> {
        println!("<class>");

        self.tokenizer.advance()?;
        match self.tokenizer.token_type() {
            TokenType::Keyword => {
                println!(
                    "<keyword> {} </keyword>",
                    self.tokenizer.key_word()?.to_string().to_lowercase()
                )
            }
            _ => bail!(Error::msg("Illegal token")),
        }

        self.tokenizer.advance()?;
        match self.tokenizer.token_type() {
            TokenType::Identifier => {
                println!("<identifier> {} </identifier>", self.tokenizer.identifier())
            }
            _ => bail!(Error::msg("Illegal token")),
        }

        self.tokenizer.advance()?;
        match self.tokenizer.token_type() {
            TokenType::Symbol => {
                println!("<symbol> {} </symbol>", self.tokenizer.symbol())
            }
            _ => bail!(Error::msg("Illegal token")),
        }

        self.tokenizer.advance()?;
        match self.tokenizer.token_type() {
            TokenType::Symbol => {
                println!("<symbol> {} </symbol>", self.tokenizer.symbol())
            }
            _ => bail!(Error::msg("Illegal token")),
        }

        println!("</class>");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation_engine::{CompilationEngine, XmlCompilationEngine};
    use crate::JackTokenizer;

    #[test]
    fn test() {
        let mut file = tempfile::tempfile().unwrap();
        writeln!(file, "class Main {{").unwrap();
        writeln!(file, "}}").unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        let tokenizer = JackTokenizer::new(file).unwrap();
        let mut engine = XmlCompilationEngine::new(tokenizer);

        engine.compile_class().unwrap();

        assert!(true);
    }
}
