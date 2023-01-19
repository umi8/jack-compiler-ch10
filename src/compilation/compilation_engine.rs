use std::io::Write;

use anyhow::Result;

use crate::compilation::class_compiler::ClassCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::tokenizer::jack_tokenizer::JackTokenizer;

pub trait CompilationEngine {
    fn new(tokenizer: JackTokenizer) -> Self;
    fn compile(&mut self, writer: &mut impl Write) -> Result<()>;
}

pub struct XmlCompilationEngine {
    tokenizer: JackTokenizer,
    writer: XmlWriter,
}

impl CompilationEngine for XmlCompilationEngine {
    fn new(tokenizer: JackTokenizer) -> Self {
        XmlCompilationEngine {
            tokenizer,
            writer: XmlWriter::new(),
        }
    }

    /// class = ’class’ className ’{’ classVarDec* subroutineDec* ’}’
    fn compile(&mut self, writer: &mut impl Write) -> Result<()> {
        ClassCompiler::compile(&mut self.tokenizer, &mut self.writer, writer)?;
        Ok(())
    }
}
