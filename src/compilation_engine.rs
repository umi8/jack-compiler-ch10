use std::io::Write;

use anyhow::{bail, Error, Result};

use crate::{JackTokenizer, TokenType};

trait CompilationEngine {
    fn new(tokenizer: JackTokenizer) -> Self;
    fn compile_class(&mut self, writer: &mut impl Write) -> Result<()>;
    fn compile_class_var_dec(&mut self, writer: &mut impl Write) -> Result<()>;
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

    fn compile_class_var_dec(&mut self, writer: &mut impl Write) -> Result<()> {
        writeln!(writer, "<classVarDec>")?;
        // static or field
        self.write_key_word(writer)?;
        // type
        self.write_key_word(writer)?;
        // varName
        self.write_identifier(writer)?;
        // ;
        self.write_symbol(writer)?;
        writeln!(writer, "</classVarDec>")?;
        Ok(())
    }
}

impl XmlCompilationEngine {
    fn write_key_word(&mut self, writer: &mut impl Write) -> Result<()> {
        self.tokenizer.advance()?;
        match self.tokenizer.token_type() {
            TokenType::Keyword => writeln!(
                writer,
                "<keyword> {} </keyword>",
                self.tokenizer.key_word()?.to_string().to_lowercase()
            )?,
            _ => bail!(Error::msg("Illegal token")),
        }
        Ok(())
    }

    fn write_identifier(&mut self, writer: &mut impl Write) -> Result<()> {
        self.tokenizer.advance()?;
        match self.tokenizer.token_type() {
            TokenType::Identifier => writeln!(
                writer,
                "<identifier> {} </identifier>",
                self.tokenizer.identifier()
            )?,
            _ => bail!(Error::msg("Illegal token")),
        }
        Ok(())
    }

    fn write_symbol(&mut self, writer: &mut impl Write) -> Result<()> {
        self.tokenizer.advance()?;
        match self.tokenizer.token_type() {
            TokenType::Symbol => {
                writeln!(writer, "<symbol> {} </symbol>", self.tokenizer.symbol())?
            }
            _ => bail!(Error::msg("Illegal token")),
        }
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
        let expected = "<class>\n\
        <keyword> class </keyword>\n\
        <identifier> Main </identifier>\n\
        <symbol> { </symbol>\n\
        <symbol> } </symbol>\n\
        </class>\n"
            .to_string();

        let mut src_file = tempfile::tempfile().unwrap();
        writeln!(src_file, "class Main {{").unwrap();
        writeln!(src_file, "}}").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let mut output = Vec::<u8>::new();

        let tokenizer = JackTokenizer::new(src_file).unwrap();
        let mut engine = XmlCompilationEngine::new(tokenizer);

        let result = engine.compile_class(&mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_compile_class_var_dec() {
        let expected = "<classVarDec>\n\
        <keyword> static </keyword>\n\
        <keyword> boolean </keyword>\n\
        <identifier> test </identifier>\n\
        <symbol> ; </symbol>\n\
        </classVarDec>\n"
            .to_string();

        let mut src_file = tempfile::tempfile().unwrap();
        writeln!(src_file, "static boolean test;").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let mut output = Vec::<u8>::new();

        let tokenizer = JackTokenizer::new(src_file).unwrap();
        let mut engine = XmlCompilationEngine::new(tokenizer);

        let result = engine.compile_class_var_dec(&mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }
}
