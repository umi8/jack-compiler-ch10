use std::io::Write;

use anyhow::Result;

use crate::compilation::type_compiler::TypeCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord::{Field, Static};

/// classVarDec = (’static’ | ’field’) type varName (’,’ varName)* ’;’
pub struct ClassVarDecCompiler {}

impl ClassVarDecCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        written: &mut impl Write,
    ) -> Result<()> {
        // <classVarDec>
        writer.write_start_tag("classVarDec", written)?;
        // static or field
        writer
            .write_key_word(tokenizer, vec![Static, Field], written)?;
        // type
        TypeCompiler::compile(tokenizer, writer, written)?;
        // varName
        writer.write_identifier(tokenizer, written)?;
        // (’,’ varName)*
        while tokenizer.peek()?.value() == "," {
            // ,
            writer.write_symbol(tokenizer, written)?;
            // varName
            writer.write_identifier(tokenizer, written)?;
        }
        // ;
        writer.write_symbol(tokenizer, written)?;
        // </classVarDec>
        writer.write_end_tag("classVarDec", written)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation::class_var_dec_compiler::ClassVarDecCompiler;
    use crate::compilation::xml_writer::XmlWriter;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile_class_var_dec() {
        let expected = "\
<classVarDec>
  <keyword> static </keyword>
  <keyword> boolean </keyword>
  <identifier> test </identifier>
  <symbol> ; </symbol>
</classVarDec>
"
            .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "static boolean test;").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();

        let result = ClassVarDecCompiler::compile(&mut tokenizer, &mut writer, &mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }
}
