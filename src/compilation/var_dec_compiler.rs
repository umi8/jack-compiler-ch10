use std::io::Write;

use anyhow::Result;

use crate::compilation::type_compiler::TypeCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord::Var;
use crate::tokenizer::token_type::TokenType::Symbol;

/// varDec = ’var’ type varName (’,’ varName)* ’;’
pub struct VarDecCompiler {}

impl VarDecCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        written: &mut impl Write,
    ) -> Result<()> {
        // <varDec>
        writer.write_start_tag("varDec", written)?;
        // ’var’
        writer
            .write_key_word(tokenizer, vec![Var], written)?;
        // type
        TypeCompiler::compile(tokenizer, writer, written)?;
        // varName
        writer.write_identifier(tokenizer, written)?;
        // (’,’ varName)*
        loop {
            if tokenizer.peek()?.token_type() == &Symbol
                && tokenizer.peek()?.value() == ","
            {
                // ','
                writer.write_symbol(tokenizer, written)?;
                // varName
                writer.write_identifier(tokenizer, written)?;
            } else {
                break;
            }
        }
        // ’;’
        writer.write_symbol(tokenizer, written)?;
        // </varDec>
        writer.write_end_tag("varDec", written)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation::var_dec_compiler::VarDecCompiler;
    use crate::compilation::xml_writer::XmlWriter;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile_var_dec() {
        let expected = "\
<varDec>
  <keyword> var </keyword>
  <keyword> int </keyword>
  <identifier> i </identifier>
  <symbol> , </symbol>
  <identifier> j </identifier>
  <symbol> , </symbol>
  <identifier> sum </identifier>
  <symbol> ; </symbol>
</varDec>
"
            .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "var int i, j, sum;").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();

        let result = VarDecCompiler::compile(&mut tokenizer, &mut writer, &mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }
}
