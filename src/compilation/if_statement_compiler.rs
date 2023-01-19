use std::io::Write;

use anyhow::Result;
use crate::compilation::expression_compiler::ExpressionCompiler;
use crate::compilation::statements_compiler::StatementsCompiler;

use crate::compilation::xml_writer::XmlWriter;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord;
use crate::tokenizer::key_word::KeyWord::{Else, If};
use crate::tokenizer::token_type::TokenType::Keyword;

/// ifStatement = ’if’ ’(’ expression ’)’ ’{’ statements ’}’ (’else’ ’{’ statements ’}’)?
pub struct IfStatementCompiler {}

impl IfStatementCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        written: &mut impl Write,
    ) -> Result<()> {
        // <ifStatement>
        writer.write_start_tag("ifStatement", written)?;
        // if
        writer
            .write_key_word(tokenizer, vec![If], written)?;
        // ’(’
        writer.write_symbol(tokenizer, written)?;
        // expression
        ExpressionCompiler::compile(tokenizer, writer, written)?;
        // ’)’
        writer.write_symbol(tokenizer, written)?;
        // ’{’
        writer.write_symbol(tokenizer, written)?;
        // statements
        StatementsCompiler::compile(tokenizer, writer, written)?;
        // ’}’
        writer.write_symbol(tokenizer, written)?;
        // (’else’ ’{’ statements ’}’)?
        if tokenizer.peek()?.token_type() == &Keyword
            && KeyWord::from(tokenizer.peek()?.value())? == KeyWord::Else
        {
            // else
            writer
                .write_key_word(tokenizer, vec![Else], written)?;
            // ’{’
            writer.write_symbol(tokenizer, written)?;
            // statements
            StatementsCompiler::compile(tokenizer, writer, written)?;
            // ’}’
            writer.write_symbol(tokenizer, written)?;
        }
        // </ifStatement>
        writer.write_end_tag("ifStatement", written)?;
        Ok(())
    }
}
