use std::io::Write;

use anyhow::{bail, Error, Result};

use crate::{JackTokenizer, TokenType};

trait CompilationEngine {
    fn new(tokenizer: JackTokenizer) -> Self;
    fn compile_class(&mut self, writer: &mut impl Write) -> Result<()>;
}

struct XmlCompilationEngine {
    tokenizer: JackTokenizer,
}

impl CompilationEngine for XmlCompilationEngine {
    fn new(tokenizer: JackTokenizer) -> Self {
        XmlCompilationEngine { tokenizer }
    }

    fn compile_class(&mut self, writer: &mut impl Write) -> Result<()> {
        writeln!(writer, "<class>")?;

        self.tokenizer.advance()?;
        match self.tokenizer.token_type() {
            TokenType::Keyword => writeln!(
                writer,
                "<keyword> {} </keyword>",
                self.tokenizer.key_word()?.to_string().to_lowercase()
            )?,
            _ => bail!(Error::msg("Illegal token")),
        }

        self.tokenizer.advance()?;
        match self.tokenizer.token_type() {
            TokenType::Identifier => writeln!(
                writer,
                "<identifier> {} </identifier>",
                self.tokenizer.identifier()
            )?,
            _ => bail!(Error::msg("Illegal token")),
        }

        self.tokenizer.advance()?;
        match self.tokenizer.token_type() {
            TokenType::Symbol => {
                writeln!(writer, "<symbol> {} </symbol>", self.tokenizer.symbol())?
            }
            _ => bail!(Error::msg("Illegal token")),
        }

        self.tokenizer.advance()?;
        match self.tokenizer.token_type() {
            TokenType::Symbol => {
                writeln!(writer, "<symbol> {} </symbol>", self.tokenizer.symbol())?
            }
            _ => bail!(Error::msg("Illegal token")),
        }

        writeln!(writer, "</class>")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation_engine::{CompilationEngine, XmlCompilationEngine};
    use crate::JackTokenizer;

    #[test]
    fn can_compile_class() {
        let mut file = tempfile::tempfile().unwrap();
        writeln!(file, "class Main {{").unwrap();
        writeln!(file, "}}").unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        let tokenizer = JackTokenizer::new(file).unwrap();
        let mut output = Vec::<u8>::new();
        let mut engine = XmlCompilationEngine::new(tokenizer);

        engine.compile_class(&mut output).unwrap();

        assert_eq!(&output, b"HELLO, WORLD!\n");
    }
}
