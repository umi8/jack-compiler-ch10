use std::io::Write;

use anyhow::{bail, Error, Result};

use crate::{JackTokenizer, TokenType};

trait CompilationEngine {
    fn new(tokenizer: JackTokenizer, writer: Box<dyn Write>) -> Self;
    fn compile_class(&mut self) -> Result<()>;
}

struct XmlCompilationEngine {
    tokenizer: JackTokenizer,
    writer: Box<dyn Write>,
}

impl CompilationEngine for XmlCompilationEngine {
    fn new(tokenizer: JackTokenizer, writer: Box<dyn Write>) -> Self {
        XmlCompilationEngine { tokenizer, writer }
    }

    fn compile_class(&mut self) -> Result<()> {
        writeln!(self.writer, "<class>")?;

        self.tokenizer.advance()?;
        match self.tokenizer.token_type() {
            TokenType::Keyword => writeln!(
                self.writer,
                "<keyword> {} </keyword>",
                self.tokenizer.key_word()?.to_string().to_lowercase()
            )?,
            _ => bail!(Error::msg("Illegal token")),
        }

        self.tokenizer.advance()?;
        match self.tokenizer.token_type() {
            TokenType::Identifier => writeln!(
                self.writer,
                "<identifier> {} </identifier>",
                self.tokenizer.identifier()
            )?,
            _ => bail!(Error::msg("Illegal token")),
        }

        self.tokenizer.advance()?;
        match self.tokenizer.token_type() {
            TokenType::Symbol => writeln!(
                self.writer,
                "<symbol> {} </symbol>",
                self.tokenizer.symbol()
            )?,
            _ => bail!(Error::msg("Illegal token")),
        }

        self.tokenizer.advance()?;
        match self.tokenizer.token_type() {
            TokenType::Symbol => writeln!(
                self.writer,
                "<symbol> {} </symbol>",
                self.tokenizer.symbol()
            )?,
            _ => bail!(Error::msg("Illegal token")),
        }

        writeln!(self.writer, "</class>")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{stdout, Seek, SeekFrom, Write};
    use std::path::Path;

    use crate::compilation_engine::{CompilationEngine, XmlCompilationEngine};
    use crate::JackTokenizer;

    #[test]
    fn test() {
        let mut file = tempfile::tempfile().unwrap();
        writeln!(file, "class Main {{").unwrap();
        writeln!(file, "}}").unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        let tokenizer = JackTokenizer::new(file).unwrap();
        let buf = Vec::<u8>::new();
        let buf = stdout().lock();
        let buf = File::create(Path::new("sample.xml")).unwrap();
        let mut engine = XmlCompilationEngine::new(tokenizer, Box::new(buf));

        engine.compile_class().unwrap();

        assert!(true);
    }
}
