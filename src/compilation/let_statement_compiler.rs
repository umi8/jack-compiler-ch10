use std::io::Write;

use anyhow::Result;

use crate::compilation::expression_compiler::ExpressionCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord::Let;

/// letStatement = ’let’ varName (’[’ expression ’]’)? ’=’ expression ’;’
pub struct LetStatementCompiler {}

impl LetStatementCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        written: &mut impl Write,
    ) -> Result<()> {
        // <letStatement>
        writer.write_start_tag("letStatement", written)?;
        // let
        writer
            .write_key_word(tokenizer, vec![Let], written)?;
        // varName
        writer.write_identifier(tokenizer, written)?;
        // (’[’ expression ’]’)?
        if tokenizer.peek()?.value() == "[" {
            // ’[’
            writer.write_symbol(tokenizer, written)?;
            // expression
            ExpressionCompiler::compile(tokenizer, writer, written)?;
            // ’]’
            writer.write_symbol(tokenizer, written)?;
        }
        // ’=’
        writer.write_symbol(tokenizer, written)?;
        // expression
        ExpressionCompiler::compile(tokenizer, writer, written)?;
        // ’;’
        writer.write_symbol(tokenizer, written)?;
        // </letStatement>
        writer.write_end_tag("letStatement", written)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation::let_statement_compiler::LetStatementCompiler;
    use crate::compilation::xml_writer::XmlWriter;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile_let_statement() {
        let expected = "\
<letStatement>
  <keyword> let </keyword>
  <identifier> length </identifier>
  <symbol> = </symbol>
  <expression>
    <term>
      <identifier> Keyboard </identifier>
      <symbol> . </symbol>
      <identifier> readInt </identifier>
      <symbol> ( </symbol>
      <expressionList>
        <expression>
          <term>
            <stringConstant> HOW MANY NUMBERS?  </stringConstant>
          </term>
        </expression>
      </expressionList>
      <symbol> ) </symbol>
    </term>
  </expression>
  <symbol> ; </symbol>
</letStatement>
"
            .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(
            src_file,
            "let length = Keyboard.readInt(\"HOW MANY NUMBERS? \");"
        )
            .unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();

        let result = LetStatementCompiler::compile(&mut tokenizer, &mut writer, &mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }
}
