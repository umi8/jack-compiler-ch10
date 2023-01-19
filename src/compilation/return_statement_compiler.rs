use std::io::Write;

use anyhow::Result;

use crate::compilation::expression_compiler::ExpressionCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord::Return;

/// returnStatement = ’return’ expression? ’;’
pub struct ReturnStatementCompiler {}

impl ReturnStatementCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        written: &mut impl Write,
    ) -> Result<()> {
        // <returnStatement>
        writer.write_start_tag("returnStatement", written)?;
        // return
        writer.write_key_word(tokenizer, vec![Return], written)?;
        // expression?
        if tokenizer.peek()?.value() != ";" {
            ExpressionCompiler::compile(tokenizer, writer, written)?;
        }
        // ’;’
        writer.write_symbol(tokenizer, written)?;
        // </returnStatement>
        writer.write_end_tag("returnStatement", written)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation::return_statement_compiler::ReturnStatementCompiler;
    use crate::compilation::xml_writer::XmlWriter;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile_return_statement() {
        let expected = "\
<returnStatement>
  <keyword> return </keyword>
  <expression>
    <term>
      <identifier> x </identifier>
    </term>
  </expression>
  <symbol> ; </symbol>
</returnStatement>
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "return x;").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();

        let result = ReturnStatementCompiler::compile(&mut tokenizer, &mut writer, &mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }
}
