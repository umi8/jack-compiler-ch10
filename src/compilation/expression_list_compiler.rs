use std::io::Write;

use crate::compilation::expression_compiler::ExpressionCompiler;
use anyhow::Result;

use crate::compilation::xml_writer::XmlWriter;
use crate::tokenizer::jack_tokenizer::JackTokenizer;

/// expressionList = (expression (’,’ expression)* )?
pub struct ExpressionListCompiler {}

impl ExpressionListCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        written: &mut impl Write,
    ) -> Result<()> {
        // <expressionList>
        writer.write_start_tag("expressionList", written)?;
        // (expression)?
        if tokenizer.is_term()? {
            // expression
            ExpressionCompiler::compile(tokenizer, writer, written)?;
            // (’,’ expression)*
            while tokenizer.peek()?.value() == "," {
                // ’,’
                writer.write_symbol(tokenizer, written)?;
                // expression
                ExpressionCompiler::compile(tokenizer, writer, written)?;
            }
        }
        // </expressionList>
        writer.write_end_tag("expressionList", written)?;
        Ok(())
    }
}
