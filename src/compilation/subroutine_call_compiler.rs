use std::io::Write;

use crate::compilation::expression_list_compiler::ExpressionListCompiler;
use anyhow::Result;

use crate::compilation::xml_writer::XmlWriter;
use crate::tokenizer::jack_tokenizer::JackTokenizer;

/// subroutineCall = subroutineName ’(’ expressionList ’)’ | (className | varName) ’.’ subroutineName ’(’ expressionList ’)’
pub struct SubroutineCallCompiler {}

impl SubroutineCallCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        written: &mut impl Write,
    ) -> Result<()> {
        // subroutineName | (className | varName)
        writer.write_identifier(tokenizer, written)?;
        if tokenizer.peek()?.value() == "." {
            // ’.’
            writer.write_symbol(tokenizer, written)?;
            // subroutineName
            writer.write_identifier(tokenizer, written)?;
        }
        // ’(’
        writer.write_symbol(tokenizer, written)?;
        // expressionList
        ExpressionListCompiler::compile(tokenizer, writer, written)?;
        // ’)’
        writer.write_symbol(tokenizer, written)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::compilation::subroutine_call_compiler::SubroutineCallCompiler;
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation::xml_writer::XmlWriter;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile_subroutine_call() {
        let expected = "\
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
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "Keyboard.readInt(\"HOW MANY NUMBERS? \")").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();

        let result = SubroutineCallCompiler::compile(&mut tokenizer, &mut writer, &mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }
}
