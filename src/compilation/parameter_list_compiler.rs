use std::io::Write;

use anyhow::Result;

use crate::compilation::type_compiler::TypeCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::tokenizer::jack_tokenizer::JackTokenizer;

pub struct ParameterListCompiler {}

impl ParameterListCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        written: &mut impl Write,
    ) -> Result<()> {
        // <parameterList>
        writer.write_start_tag("parameterList", written)?;
        // ((type varName) (’,’ type varName)*)?
        if tokenizer.peek()?.is_type()? {
            // type
            TypeCompiler::compile(tokenizer, writer, written)?;
            // varName
            writer.write_identifier(tokenizer, written)?;
            // (’,’ type varName)*
            while tokenizer.peek()?.value() == "," {
                // ’,’
                writer.write_symbol(tokenizer, written)?;
                // type
                TypeCompiler::compile(tokenizer, writer, written)?;
                // varName
                writer.write_identifier(tokenizer, written)?;
            }
        }
        // </parameterList>
        writer.write_end_tag("parameterList", written)?;
        Ok(())
    }
}
