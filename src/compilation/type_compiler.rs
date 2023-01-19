use std::io::Write;

use anyhow::{bail, Error, Result};

use crate::compilation::xml_writer::XmlWriter;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord::{Boolean, Char, Int};
use crate::tokenizer::token_type::TokenType;

/// type = ’int’ | ’char’ | ’boolean’ | className
pub struct TypeCompiler {}

impl TypeCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        written: &mut impl Write,
    ) -> Result<()> {
        match tokenizer.peek()?.token_type() {
            TokenType::Keyword => {
                writer.write_key_word(tokenizer, vec![Int, Boolean, Char], written)?
            }
            TokenType::Identifier => writer.write_identifier(tokenizer, written)?,
            _ => bail!(Error::msg("Illegal token")),
        }
        Ok(())
    }
}
