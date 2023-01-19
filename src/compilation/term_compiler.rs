use std::io::Write;

use anyhow::Result;

use crate::compilation::expression_compiler::ExpressionCompiler;
use crate::compilation::subroutine_call_compiler::SubroutineCallCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord::{False, Null, This, True};
use crate::tokenizer::token_type::TokenType;

/// term = integerConstant | stringConstant | keywordConstant | varName | varName ’[’ expression ’]’ | subroutineCall | ’(’ expression ’)’ | unaryOp term
pub struct TermCompiler {}

impl TermCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        written: &mut impl Write,
    ) -> Result<()> {
        // <term>
        writer.write_start_tag("term", written)?;

        match tokenizer.peek()?.token_type() {
            TokenType::Keyword => {
                if tokenizer.peek()?.is_keyword_constant()? {
                    writer.write_key_word(
                        tokenizer,
                        vec![True, False, Null, This],
                        written,
                    )?;
                }
            }
            TokenType::Symbol => match tokenizer.peek()?.value().as_str() {
                "(" => {
                    // '('
                    writer.write_symbol(tokenizer, written)?;
                    // expression
                    ExpressionCompiler::compile(tokenizer, writer, written)?;
                    // ')'
                    writer.write_symbol(tokenizer, written)?;
                }
                "-" | "~" => {
                    // unaryOp
                    writer.write_symbol(tokenizer, written)?;
                    // term
                    TermCompiler::compile(tokenizer, writer, written)?;
                }
                _ => {}
            },
            TokenType::Identifier => {
                match tokenizer.peek_second()?.value().as_str() {
                    "[" => {
                        // varName
                        writer.write_identifier(tokenizer, written)?;
                        // '['
                        writer.write_symbol(tokenizer, written)?;
                        // expression
                        ExpressionCompiler::compile(tokenizer, writer, written)?;
                        // ']'
                        writer.write_symbol(tokenizer, written)?;
                    }
                    "." | "(" => SubroutineCallCompiler::compile(tokenizer, writer, written)?,
                    _ => writer.write_identifier(tokenizer, written)?,
                }
            }
            TokenType::IntConst =>
                writer
                    .write_integer_constant(tokenizer, written)?,
            TokenType::StringConst =>
                writer
                    .write_string_constant(tokenizer, written)?,
        }

        // </term>
        writer.write_end_tag("term", written)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};
    use crate::compilation::term_compiler::TermCompiler;

    use crate::compilation::xml_writer::XmlWriter;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile_term() {
        let expected = "\
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
"
            .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "Keyboard.readInt(\"HOW MANY NUMBERS? \")").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();

        let result = TermCompiler::compile(&mut tokenizer, &mut writer, &mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }
}
