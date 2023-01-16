use std::io::Write;

use anyhow::{bail, Error, Result};

use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord;
use crate::tokenizer::key_word::KeyWord::{False, Null, This, True};
use crate::tokenizer::token_type::TokenType;

pub trait CompilationEngine {
    fn new(tokenizer: JackTokenizer) -> Self;
    fn compile_class(&mut self, writer: &mut impl Write) -> Result<()>;
    fn compile_class_var_dec(&mut self, writer: &mut impl Write) -> Result<()>;
    fn compile_type(&mut self, writer: &mut impl Write) -> Result<()>;
    fn compile_subroutine_dec(&mut self, writer: &mut impl Write) -> Result<()>;
    fn compile_subroutine_body(&mut self, writer: &mut impl Write) -> Result<()>;
    fn compile_var_dec(&mut self, writer: &mut impl Write) -> Result<()>;
    fn compile_statements(&mut self, writer: &mut impl Write) -> Result<()>;
    fn compile_statement(&mut self, writer: &mut impl Write) -> Result<()>;
    fn compile_let_statement(&mut self, writer: &mut impl Write) -> Result<()>;
    fn compile_if_statement(&mut self, writer: &mut impl Write) -> Result<()>;
    fn compile_while_statement(&mut self, writer: &mut impl Write) -> Result<()>;
    fn compile_do_statement(&mut self, writer: &mut impl Write) -> Result<()>;
    fn compile_return_statement(&mut self, writer: &mut impl Write) -> Result<()>;
    fn compile_expression(&mut self, writer: &mut impl Write) -> Result<()>;
    fn compile_term(&mut self, writer: &mut impl Write) -> Result<()>;
    fn compile_subroutine_call(&mut self, writer: &mut impl Write) -> Result<()>;
    fn compile_expression_list(&mut self, writer: &mut impl Write) -> Result<()>;
}

pub struct XmlCompilationEngine {
    tokenizer: JackTokenizer,
    indent: String,
}

const INDENT_COUNT: usize = 2;

impl CompilationEngine for XmlCompilationEngine {
    fn new(tokenizer: JackTokenizer) -> Self {
        XmlCompilationEngine {
            tokenizer,
            indent: String::new(),
        }
    }

    /// class = ’class’ className ’{’ classVarDec* subroutineDec* ’}’
    fn compile_class(&mut self, writer: &mut impl Write) -> Result<()> {
        // <class>
        self.write_start_tag("class", writer)?;
        // ’class’
        self.write_key_word(vec![KeyWord::Class], writer)?;
        // className
        self.write_identifier(writer)?;
        // {
        self.write_symbol(writer)?;
        // classVarDec*
        loop {
            if !KeyWord::exists(self.tokenizer.peek()?.value()) {
                break;
            }
            match KeyWord::from(self.tokenizer.peek()?.value())? {
                KeyWord::Static | KeyWord::Field => self.compile_class_var_dec(writer)?,
                _ => break,
            }
        }
        // subroutineDec*
        loop {
            if !KeyWord::exists(self.tokenizer.peek()?.value()) {
                break;
            }
            match KeyWord::from(self.tokenizer.peek()?.value())? {
                KeyWord::Constructor | KeyWord::Function | KeyWord::Method => {
                    self.compile_subroutine_dec(writer)?
                }
                _ => break,
            }
        }
        // }
        self.write_symbol(writer)?;
        // </class>
        self.write_end_tag("class", writer)?;
        Ok(())
    }

    /// classVarDec = (’static’ | ’field’) type varName (’,’ varName)* ’;’
    fn compile_class_var_dec(&mut self, writer: &mut impl Write) -> Result<()> {
        // <classVarDec>
        self.write_start_tag("classVarDec", writer)?;
        // static or field
        self.write_key_word(vec![KeyWord::Static, KeyWord::Field], writer)?;
        // type
        self.compile_type(writer)?;
        // varName
        self.write_identifier(writer)?;
        // TODO: (’,’ varName)*
        // ;
        self.write_symbol(writer)?;
        // </classVarDec>
        self.write_end_tag("classVarDec", writer)?;
        Ok(())
    }

    /// type = ’int’ | ’char’ | ’boolean’ | className
    fn compile_type(&mut self, writer: &mut impl Write) -> Result<()> {
        match self.tokenizer.peek()?.token_type() {
            TokenType::Keyword => {
                self.write_key_word(vec![KeyWord::Int, KeyWord::Boolean, KeyWord::Char], writer)?
            }
            TokenType::Identifier => self.write_identifier(writer)?,
            _ => bail!(Error::msg("Illegal token")),
        }
        Ok(())
    }

    /// subroutineDec =(’constructor’ | ’function’ | ’method’) (’void’ | type) subroutineName ’(’ parameterList ’)’ subroutineBody
    fn compile_subroutine_dec(&mut self, writer: &mut impl Write) -> Result<()> {
        // <subroutineDec>
        self.write_start_tag("subroutineDec", writer)?;
        // ’constructor’ | ’function’ | ’method’
        self.write_key_word(
            vec![KeyWord::Constructor, KeyWord::Function, KeyWord::Method],
            writer,
        )?;
        // ’void’ | type
        if self.tokenizer.peek()?.token_type() == &TokenType::Keyword
            && KeyWord::from(self.tokenizer.peek()?.value())? == KeyWord::Void
        {
            self.write_key_word(vec![KeyWord::Void], writer)?
        } else {
            self.compile_type(writer)?
        }
        // subroutineName
        self.write_identifier(writer)?;
        // ’(’
        self.write_symbol(writer)?;
        // TODO: parameterList
        self.write_start_tag("parameterList", writer)?;
        self.write_end_tag("parameterList", writer)?;
        // ’)’
        self.write_symbol(writer)?;
        // subroutineBody
        self.compile_subroutine_body(writer)?;
        // </subroutineDec>
        self.write_end_tag("subroutineDec", writer)?;
        Ok(())
    }

    /// subroutineBody = ’{’ varDec* statements ’}’
    fn compile_subroutine_body(&mut self, writer: &mut impl Write) -> Result<()> {
        // <subroutineBody>
        self.write_start_tag("subroutineBody", writer)?;
        // ’{’
        self.write_symbol(writer)?;
        // varDec*
        loop {
            if !KeyWord::exists(self.tokenizer.peek()?.value()) {
                break;
            }
            match KeyWord::from(self.tokenizer.peek()?.value())? {
                KeyWord::Var => self.compile_var_dec(writer)?,
                _ => break,
            }
        }
        // statements
        self.compile_statements(writer)?;
        // ’}’
        self.write_symbol(writer)?;
        // </subroutineBody>
        self.write_end_tag("subroutineBody", writer)?;
        Ok(())
    }

    /// varDec = ’var’ type varName (’,’ varName)* ’;’
    fn compile_var_dec(&mut self, writer: &mut impl Write) -> Result<()> {
        // <varDec>
        self.write_start_tag("varDec", writer)?;
        // ’var’
        self.write_key_word(vec![KeyWord::Var], writer)?;
        // type
        self.compile_type(writer)?;
        // varName
        self.write_identifier(writer)?;
        // (’,’ varName)*
        loop {
            if self.tokenizer.peek()?.token_type() == &TokenType::Symbol
                && self.tokenizer.peek()?.value() == ","
            {
                // ','
                self.write_symbol(writer)?;
                // varName
                self.write_identifier(writer)?;
            } else {
                break;
            }
        }
        // ’;’
        self.write_symbol(writer)?;
        // </varDec>
        self.write_end_tag("varDec", writer)?;
        Ok(())
    }

    /// statements = statement*
    fn compile_statements(&mut self, writer: &mut impl Write) -> Result<()> {
        // <statements>
        self.write_start_tag("statements", writer)?;
        loop {
            if !KeyWord::exists(self.tokenizer.peek()?.value()) {
                break;
            }
            match KeyWord::from(self.tokenizer.peek()?.value())? {
                KeyWord::Let | KeyWord::If | KeyWord::While | KeyWord::Do | KeyWord::Return => {
                    self.compile_statement(writer)?;
                }
                _ => break,
            }
        }
        // </statements>
        self.write_end_tag("statements", writer)?;
        Ok(())
    }

    /// statement = letStatement | ifStatement | whileStatement | doStatement | returnStatement
    fn compile_statement(&mut self, writer: &mut impl Write) -> Result<()> {
        match KeyWord::from(self.tokenizer.peek()?.value())? {
            KeyWord::Let => self.compile_let_statement(writer)?,
            KeyWord::If => self.compile_if_statement(writer)?,
            KeyWord::While => self.compile_while_statement(writer)?,
            KeyWord::Do => self.compile_do_statement(writer)?,
            KeyWord::Return => self.compile_return_statement(writer)?,
            _ => {}
        }
        Ok(())
    }

    /// letStatement = ’let’ varName (’[’ expression ’]’)? ’=’ expression ’;’
    fn compile_let_statement(&mut self, writer: &mut impl Write) -> Result<()> {
        // <letStatement>
        self.write_start_tag("letStatement", writer)?;
        // let
        self.write_key_word(vec![KeyWord::Let], writer)?;
        // varName
        self.write_identifier(writer)?;
        // (’[’ expression ’]’)?
        if self.tokenizer.peek()?.value() == "[" {
            // ’[’
            self.write_symbol(writer)?;
            // expression
            self.compile_expression(writer)?;
            // ’]’
            self.write_symbol(writer)?;
        }
        // ’=’
        self.write_symbol(writer)?;
        // expression
        self.compile_expression(writer)?;
        // ’;’
        self.write_symbol(writer)?;
        // </letStatement>
        self.write_end_tag("letStatement", writer)?;
        Ok(())
    }

    /// ifStatement = ’if’ ’(’ expression ’)’ ’{’ statements ’}’ (’else’ ’{’ statements ’}’)?
    fn compile_if_statement(&mut self, writer: &mut impl Write) -> Result<()> {
        // <ifStatement>
        self.write_start_tag("ifStatement", writer)?;
        // if
        self.write_key_word(vec![KeyWord::If], writer)?;
        // ’(’
        self.write_symbol(writer)?;
        // expression
        self.compile_expression(writer)?;
        // ’)’
        self.write_symbol(writer)?;
        // ’{’
        self.write_symbol(writer)?;
        // statements
        self.compile_statements(writer)?;
        // ’}’
        self.write_symbol(writer)?;
        // (’else’ ’{’ statements ’}’)?
        match self.tokenizer.peek()?.token_type() {
            TokenType::Keyword => match KeyWord::from(self.tokenizer.peek()?.value())? {
                KeyWord::Else => {
                    // else
                    self.write_key_word(vec![KeyWord::Else], writer)?;
                    // ’{’
                    self.write_symbol(writer)?;
                    // statements
                    self.compile_statements(writer)?;
                    // ’}’
                    self.write_symbol(writer)?;
                }
                _ => {}
            },
            _ => {}
        }
        // </ifStatement>
        self.write_end_tag("ifStatement", writer)?;
        Ok(())
    }

    /// whileStatement = ’while’ ’(’ expression ’)’ ’{’ statements ’}’
    fn compile_while_statement(&mut self, writer: &mut impl Write) -> Result<()> {
        // <whileStatement>
        self.write_start_tag("whileStatement", writer)?;
        // while
        self.write_key_word(vec![KeyWord::While], writer)?;
        // ’(’
        self.write_symbol(writer)?;
        // expression
        self.compile_expression(writer)?;
        // ’)’
        self.write_symbol(writer)?;
        // ’{’
        self.write_symbol(writer)?;
        // statements
        self.compile_statements(writer)?;
        // ’}’
        self.write_symbol(writer)?;
        // </whileStatement>
        self.write_end_tag("whileStatement", writer)?;
        Ok(())
    }

    /// doStatement = ’do’ subroutineCall ’;’
    fn compile_do_statement(&mut self, writer: &mut impl Write) -> Result<()> {
        // <doStatement>
        self.write_start_tag("doStatement", writer)?;
        // do
        self.write_key_word(vec![KeyWord::Do], writer)?;
        // subroutineCall
        self.compile_subroutine_call(writer)?;
        // ’;’
        self.write_symbol(writer)?;
        // </doStatement>
        self.write_end_tag("doStatement", writer)?;
        Ok(())
    }

    /// returnStatement = ’return’ expression? ’;’
    fn compile_return_statement(&mut self, writer: &mut impl Write) -> Result<()> {
        // <returnStatement>
        self.write_start_tag("returnStatement", writer)?;
        // return
        self.write_key_word(vec![KeyWord::Return], writer)?;
        // expression?
        if self.tokenizer.peek()?.value() != ";" {
            self.compile_expression(writer)?;
        }
        // ’;’
        self.write_symbol(writer)?;
        // </returnStatement>
        self.write_end_tag("returnStatement", writer)?;
        Ok(())
    }

    /// expression = term (op term)*
    fn compile_expression(&mut self, writer: &mut impl Write) -> Result<()> {
        // <expression>
        self.write_start_tag("expression", writer)?;
        // term
        self.compile_term(writer)?;
        // (op term)*
        loop {
            if self.tokenizer.peek()?.is_op() {
                // op
                self.write_symbol(writer)?;
                // term
                self.compile_term(writer)?;
            } else {
                break;
            }
        }
        // </expression>
        self.write_end_tag("expression", writer)?;
        Ok(())
    }

    /// term = integerConstant | stringConstant | keywordConstant | varName | varName ’[’ expression ’]’ | subroutineCall | ’(’ expression ’)’ | unaryOp term
    fn compile_term(&mut self, writer: &mut impl Write) -> Result<()> {
        // <term>
        self.write_start_tag("term", writer)?;

        match self.tokenizer.peek()?.token_type() {
            TokenType::Keyword => {
                if self.tokenizer.peek()?.is_keyword_constant()? {
                    self.write_key_word(vec![True, False, Null, This], writer)?;
                }
            }
            TokenType::Symbol => match self.tokenizer.peek()?.value().as_str() {
                "(" => {
                    // '('
                    self.write_symbol(writer)?;
                    // expression
                    self.compile_expression(writer)?;
                    // ')'
                    self.write_symbol(writer)?;
                }
                "-" | "~" => {
                    // unaryOp
                    self.write_symbol(writer)?;
                    // term
                    self.compile_term(writer)?;
                }
                _ => {}
            },
            TokenType::Identifier => {
                match self.tokenizer.peek_second()?.value().as_str() {
                    "[" => {
                        // varName
                        self.write_identifier(writer)?;
                        // '['
                        self.write_symbol(writer)?;
                        // expression
                        self.compile_expression(writer)?;
                        // ']'
                        self.write_symbol(writer)?;
                    }
                    "." | "(" => self.compile_subroutine_call(writer)?,
                    _ => self.write_identifier(writer)?,
                }
            }
            TokenType::IntConst => self.write_integer_constant(writer)?,
            TokenType::StringConst => self.write_string_constant(writer)?,
        }

        // </term>
        self.write_end_tag("term", writer)?;
        Ok(())
    }

    /// subroutineCall = subroutineName ’(’ expressionList ’)’ | (className | varName) ’.’ subroutineName ’(’ expressionList ’)’
    fn compile_subroutine_call(&mut self, writer: &mut impl Write) -> Result<()> {
        // subroutineName | (className | varName)
        self.write_identifier(writer)?;
        if self.tokenizer.peek()?.value() == "." {
            // ’.’
            self.write_symbol(writer)?;
            // subroutineName
            self.write_identifier(writer)?;
        }
        // ’(’
        self.write_symbol(writer)?;
        // expressionList
        self.compile_expression_list(writer)?;
        // ’)’
        self.write_symbol(writer)?;
        Ok(())
    }

    /// expressionList = (expression (’,’ expression)* )?
    fn compile_expression_list(&mut self, writer: &mut impl Write) -> Result<()> {
        // <expressionList>
        self.write_start_tag("expressionList", writer)?;
        // (expression)?
        if self.tokenizer.is_term()? {
            self.compile_expression(writer)?;
            // TODO: (’,’ expression)*
        }
        // </expressionList>
        self.write_end_tag("expressionList", writer)?;
        Ok(())
    }
}

impl XmlCompilationEngine {
    fn write_key_word(&mut self, targets: Vec<KeyWord>, writer: &mut impl Write) -> Result<()> {
        self.tokenizer.advance()?;
        match self.tokenizer.token_type()? {
            TokenType::Keyword => {
                let keyword = self.tokenizer.key_word()?;
                match keyword {
                    keyword if targets.contains(&keyword) => writeln!(
                        writer,
                        "{}<keyword> {} </keyword>",
                        self.indent,
                        self.tokenizer.key_word()?.to_string().to_lowercase()
                    )?,
                    _ => bail!(Error::msg("Illegal token")),
                }
            }
            _ => bail!(Error::msg("Illegal token")),
        }
        Ok(())
    }

    fn write_identifier(&mut self, writer: &mut impl Write) -> Result<()> {
        self.tokenizer.advance()?;
        match self.tokenizer.token_type()? {
            TokenType::Identifier => writeln!(
                writer,
                "{}<identifier> {} </identifier>",
                self.indent,
                self.tokenizer.identifier()
            )?,
            _ => bail!(Error::msg("Illegal token")),
        }
        Ok(())
    }

    fn write_symbol(&mut self, writer: &mut impl Write) -> Result<()> {
        self.tokenizer.advance()?;
        match self.tokenizer.token_type()? {
            TokenType::Symbol => {
                let symbol = match self.tokenizer.symbol() {
                    '<' => "&lt;",
                    '>' => "&gt;",
                    '&' => "&amp;",
                    _ => "",
                };

                if symbol.is_empty() {
                    writeln!(
                        writer,
                        "{}<symbol> {} </symbol>",
                        self.indent,
                        self.tokenizer.symbol()
                    )?
                } else {
                    writeln!(writer, "{}<symbol> {} </symbol>", self.indent, symbol)?
                }
            }
            _ => bail!(Error::msg("Illegal token")),
        }
        Ok(())
    }

    fn write_string_constant(&mut self, writer: &mut impl Write) -> Result<()> {
        self.tokenizer.advance()?;
        match self.tokenizer.token_type()? {
            TokenType::StringConst => writeln!(
                writer,
                "{}<stringConstant> {} </stringConstant>",
                self.indent,
                self.tokenizer.string_val()
            )?,
            _ => bail!(Error::msg("Illegal token")),
        }
        Ok(())
    }

    fn write_integer_constant(&mut self, writer: &mut impl Write) -> Result<()> {
        self.tokenizer.advance()?;
        match self.tokenizer.token_type()? {
            TokenType::IntConst => writeln!(
                writer,
                "{}<integerConstant> {} </integerConstant>",
                self.indent,
                self.tokenizer.int_val()?
            )?,
            _ => bail!(Error::msg("Illegal token")),
        }
        Ok(())
    }

    fn write_start_tag(&mut self, element: &str, writer: &mut impl Write) -> Result<()> {
        writeln!(writer, "{}<{}>", self.indent, element)?;
        self.increase_indent();
        Ok(())
    }

    fn write_end_tag(&mut self, element: &str, writer: &mut impl Write) -> Result<()> {
        self.decrease_indent();
        writeln!(writer, "{}</{}>", self.indent, element)?;
        Ok(())
    }

    fn increase_indent(&mut self) {
        self.indent += &" ".repeat(INDENT_COUNT);
    }

    fn decrease_indent(&mut self) {
        let count_after_decrease = self.indent.len() - INDENT_COUNT;
        self.indent = self.indent[..count_after_decrease].parse().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation::compilation_engine::{CompilationEngine, XmlCompilationEngine};
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile_class() {
        let expected = "\
<class>
  <keyword> class </keyword>
  <identifier> Main </identifier>
  <symbol> { </symbol>
  <symbol> } </symbol>
</class>
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "class Main {{").unwrap();
        writeln!(src_file, "}}").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let tokenizer = JackTokenizer::new(path).unwrap();
        let mut engine = XmlCompilationEngine::new(tokenizer);

        let result = engine.compile_class(&mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_compile_class_with_classvardec() {
        let expected = "\
<class>
  <keyword> class </keyword>
  <identifier> Main </identifier>
  <symbol> { </symbol>
  <classVarDec>
    <keyword> static </keyword>
    <keyword> boolean </keyword>
    <identifier> test </identifier>
    <symbol> ; </symbol>
  </classVarDec>
  <classVarDec>
    <keyword> static </keyword>
    <keyword> boolean </keyword>
    <identifier> test </identifier>
    <symbol> ; </symbol>
  </classVarDec>
  <symbol> } </symbol>
</class>
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "class Main {{").unwrap();
        writeln!(src_file, "static boolean test;").unwrap();
        writeln!(src_file, "static boolean test;").unwrap();
        writeln!(src_file, "}}").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let tokenizer = JackTokenizer::new(path).unwrap();
        let mut engine = XmlCompilationEngine::new(tokenizer);

        let result = engine.compile_class(&mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_compile_class_with_subroutinedec() {
        let expected = "\
<class>
  <keyword> class </keyword>
  <identifier> Main </identifier>
  <symbol> { </symbol>
  <subroutineDec>
    <keyword> function </keyword>
    <keyword> void </keyword>
    <identifier> main </identifier>
    <symbol> ( </symbol>
    <parameterList>
    </parameterList>
    <symbol> ) </symbol>
    <subroutineBody>
      <symbol> { </symbol>
      <statements>
        <returnStatement>
          <keyword> return </keyword>
          <symbol> ; </symbol>
        </returnStatement>
      </statements>
      <symbol> } </symbol>
    </subroutineBody>
  </subroutineDec>
  <subroutineDec>
    <keyword> function </keyword>
    <keyword> boolean </keyword>
    <identifier> isSomething </identifier>
    <symbol> ( </symbol>
    <parameterList>
    </parameterList>
    <symbol> ) </symbol>
    <subroutineBody>
      <symbol> { </symbol>
      <statements>
        <returnStatement>
          <keyword> return </keyword>
          <symbol> ; </symbol>
        </returnStatement>
      </statements>
      <symbol> } </symbol>
    </subroutineBody>
  </subroutineDec>
  <symbol> } </symbol>
</class>
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "class Main {{").unwrap();
        writeln!(src_file, "function void main() {{ return; }}").unwrap();
        writeln!(src_file, "function boolean isSomething() {{ return; }}").unwrap();
        writeln!(src_file, "}}").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let tokenizer = JackTokenizer::new(path).unwrap();
        let mut engine = XmlCompilationEngine::new(tokenizer);

        let result = engine.compile_class(&mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }

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

        let tokenizer = JackTokenizer::new(path).unwrap();
        let mut engine = XmlCompilationEngine::new(tokenizer);

        let result = engine.compile_class_var_dec(&mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_compile_subroutine_dec() {
        let expected = "\
<subroutineDec>
  <keyword> function </keyword>
  <keyword> void </keyword>
  <identifier> main </identifier>
  <symbol> ( </symbol>
  <parameterList>
  </parameterList>
  <symbol> ) </symbol>
  <subroutineBody>
    <symbol> { </symbol>
    <statements>
      <returnStatement>
        <keyword> return </keyword>
        <symbol> ; </symbol>
      </returnStatement>
    </statements>
    <symbol> } </symbol>
  </subroutineBody>
</subroutineDec>
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "function void main() {{").unwrap();
        writeln!(src_file, "return;").unwrap();
        writeln!(src_file, "}}").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let tokenizer = JackTokenizer::new(path).unwrap();
        let mut engine = XmlCompilationEngine::new(tokenizer);

        let result = engine.compile_subroutine_dec(&mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_compile_subroutine_body() {
        let expected = "\
<subroutineBody>
  <symbol> { </symbol>
  <varDec>
    <keyword> var </keyword>
    <identifier> Array </identifier>
    <identifier> a </identifier>
    <symbol> ; </symbol>
  </varDec>
  <varDec>
    <keyword> var </keyword>
    <keyword> int </keyword>
    <identifier> length </identifier>
    <symbol> ; </symbol>
  </varDec>
  <statements>
    <returnStatement>
      <keyword> return </keyword>
      <symbol> ; </symbol>
    </returnStatement>
  </statements>
  <symbol> } </symbol>
</subroutineBody>
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "{{").unwrap();
        writeln!(src_file, "var Array a;").unwrap();
        writeln!(src_file, "var int length;").unwrap();
        writeln!(src_file, "return;").unwrap();
        writeln!(src_file, "}}").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let tokenizer = JackTokenizer::new(path).unwrap();
        let mut engine = XmlCompilationEngine::new(tokenizer);

        let result = engine.compile_subroutine_body(&mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }

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

        let tokenizer = JackTokenizer::new(path).unwrap();
        let mut engine = XmlCompilationEngine::new(tokenizer);

        let result = engine.compile_var_dec(&mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }

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

        let tokenizer = JackTokenizer::new(path).unwrap();
        let mut engine = XmlCompilationEngine::new(tokenizer);

        let result = engine.compile_let_statement(&mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }

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

        let tokenizer = JackTokenizer::new(path).unwrap();
        let mut engine = XmlCompilationEngine::new(tokenizer);

        let result = engine.compile_while_statement(&mut output);
        let actual = String::from_utf8(output).unwrap();

        assert_eq!(expected, actual);
        assert!(result.is_ok());
    }

    #[test]
    fn can_compile_do_statement() {
        let expected = "\
<doStatement>
  <keyword> do </keyword>
  <identifier> Output </identifier>
  <symbol> . </symbol>
  <identifier> printString </identifier>
  <symbol> ( </symbol>
  <expressionList>
    <expression>
      <term>
        <stringConstant> THE AVERAGE IS:  </stringConstant>
      </term>
    </expression>
  </expressionList>
  <symbol> ) </symbol>
  <symbol> ; </symbol>
</doStatement>
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "do Output.printString(\"THE AVERAGE IS: \");").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let tokenizer = JackTokenizer::new(path).unwrap();
        let mut engine = XmlCompilationEngine::new(tokenizer);

        let result = engine.compile_do_statement(&mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }

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

        let tokenizer = JackTokenizer::new(path).unwrap();
        let mut engine = XmlCompilationEngine::new(tokenizer);

        let result = engine.compile_return_statement(&mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }

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

        let tokenizer = JackTokenizer::new(path).unwrap();
        let mut engine = XmlCompilationEngine::new(tokenizer);

        let result = engine.compile_term(&mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }

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

        let tokenizer = JackTokenizer::new(path).unwrap();
        let mut engine = XmlCompilationEngine::new(tokenizer);

        let result = engine.compile_subroutine_call(&mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }
}
