use std::io::Write;

use anyhow::Result;

use crate::compilation::expression_compiler::ExpressionCompiler;
use crate::compilation::statements_compiler::StatementsCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord::While;

/// whileStatement = ’while’ ’(’ expression ’)’ ’{’ statements ’}’
pub struct WhileStatementCompiler {}

impl WhileStatementCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        written: &mut impl Write,
    ) -> Result<()> {
        // <whileStatement>
        writer.write_start_tag("whileStatement", written)?;
        // while
        writer.write_key_word(tokenizer, vec![While], written)?;
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
        // </whileStatement>
        writer.write_end_tag("whileStatement", written)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::compilation::while_statement_compiler::WhileStatementCompiler;
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation::xml_writer::XmlWriter;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile_while_statement() {
        let expected = "\
<whileStatement>
  <keyword> while </keyword>
  <symbol> ( </symbol>
  <expression>
    <term>
      <identifier> i </identifier>
    </term>
    <symbol> &lt; </symbol>
    <term>
      <identifier> length </identifier>
    </term>
  </expression>
  <symbol> ) </symbol>
  <symbol> { </symbol>
  <statements>
    <letStatement>
      <keyword> let </keyword>
      <identifier> a </identifier>
      <symbol> [ </symbol>
      <expression>
        <term>
          <identifier> i </identifier>
        </term>
      </expression>
      <symbol> ] </symbol>
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
                <stringConstant> ENTER THE NEXT NUMBER:  </stringConstant>
              </term>
            </expression>
          </expressionList>
          <symbol> ) </symbol>
        </term>
      </expression>
      <symbol> ; </symbol>
    </letStatement>
    <letStatement>
      <keyword> let </keyword>
      <identifier> i </identifier>
      <symbol> = </symbol>
      <expression>
        <term>
          <identifier> i </identifier>
        </term>
        <symbol> + </symbol>
        <term>
          <integerConstant> 1 </integerConstant>
        </term>
      </expression>
      <symbol> ; </symbol>
    </letStatement>
  </statements>
  <symbol> } </symbol>
</whileStatement>
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "while (i < length) {{").unwrap();
        writeln!(
            src_file,
            "let a[i] = Keyboard.readInt(\"ENTER THE NEXT NUMBER: \");"
        )
        .unwrap();
        writeln!(src_file, "let i = i + 1;").unwrap();
        writeln!(src_file, "}}").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();

        let result = WhileStatementCompiler::compile(&mut tokenizer, &mut writer, &mut output);
        let actual = String::from_utf8(output).unwrap();

        assert_eq!(expected, actual);
        assert!(result.is_ok());
    }
}
