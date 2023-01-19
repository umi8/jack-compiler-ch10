use std::io::Write;

use anyhow::Result;

use crate::compilation::statement_compiler::StatementCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord;

/// statements = statement*
pub struct StatementsCompiler {}

impl StatementsCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        written: &mut impl Write,
    ) -> Result<()> {
        // <statements>
        writer.write_start_tag("statements", written)?;
        loop {
            if !KeyWord::exists(tokenizer.peek()?.value()) {
                break;
            }
            match KeyWord::from(tokenizer.peek()?.value())? {
                KeyWord::Let | KeyWord::If | KeyWord::While | KeyWord::Do | KeyWord::Return => {
                    StatementCompiler::compile(tokenizer, writer, written)?;
                }
                _ => break,
            }
        }
        // </statements>
        writer.write_end_tag("statements", written)?;
        Ok(())
    }
}
