use std::io::Write;

use anyhow::{bail, Error, Result};

use crate::tokenizer::key_word::KeyWord;
use crate::{JackTokenizer, TokenType};

trait CompilationEngine {
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
    fn compile_expression(&mut self, writer: &mut impl Write) -> Result<()>;
    fn compile_term(&mut self, writer: &mut impl Write) -> Result<()>;
    fn compile_subroutine_call(&mut self, writer: &mut impl Write) -> Result<()>;
    fn compile_expression_list(&mut self, writer: &mut impl Write) -> Result<()>;
}

struct XmlCompilationEngine {
    tokenizer: JackTokenizer,
}

impl CompilationEngine for XmlCompilationEngine {
    fn new(tokenizer: JackTokenizer) -> Self {
        XmlCompilationEngine { tokenizer }
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
        // TODO: subroutineBody
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
        // TODO: statements
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
        self.compile_statement(writer)?;
        Ok(())
    }

    /// statement = letStatement | ifStatement | whileStatement | doStatement | returnStatement
    fn compile_statement(&mut self, writer: &mut impl Write) -> Result<()> {
        self.compile_let_statement(writer)?;
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
        // TODO: (’[’ expression ’]’)?
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

    /// expression = term (op term)*
    fn compile_expression(&mut self, writer: &mut impl Write) -> Result<()> {
        // <expression>
        self.write_start_tag("expression", writer)?;
        // term
        self.compile_term(writer)?;
        // TODO: (op term)*
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
                todo!("keywordConstant")
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
                    todo!("unaryOp");
                    // TODO: call term
                }
                _ => {}
            },
            TokenType::Identifier => {
                match self.tokenizer.peek_second()?.value().as_str() {
                    "[" => {
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

        self.compile_expression(writer)?;

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
                        "<keyword> {} </keyword>",
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
                "<identifier> {} </identifier>",
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
                writeln!(writer, "<symbol> {} </symbol>", self.tokenizer.symbol())?
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
                "<stringConstant> {} </stringConstant>",
                self.tokenizer.string_val()
            )?,
            _ => bail!(Error::msg("Illegal token")),
        }
        Ok(())
    }

    fn write_integer_constant(&mut self, writer: &mut impl Write) -> Result<()> {
        self.tokenizer.advance()?;
        match self.tokenizer.token_type()? {
            TokenType::StringConst => writeln!(
                writer,
                "<integerConstant> {} </integerConstant>",
                self.tokenizer.int_val()?
            )?,
            _ => bail!(Error::msg("Illegal token")),
        }
        Ok(())
    }

    fn write_start_tag(&mut self, element: &str, writer: &mut impl Write) -> Result<()> {
        writeln!(writer, "<{}>", element)?;
        Ok(())
    }

    fn write_end_tag(&mut self, element: &str, writer: &mut impl Write) -> Result<()> {
        writeln!(writer, "</{}>", element)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation::compilation_engine::{CompilationEngine, XmlCompilationEngine};
    use crate::JackTokenizer;

    #[test]
    fn can_compile_class() {
        let expected = "<class>\n\
        <keyword> class </keyword>\n\
        <identifier> Main </identifier>\n\
        <symbol> { </symbol>\n\
        <symbol> } </symbol>\n\
        </class>\n"
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
        let expected = "<class>\n\
        <keyword> class </keyword>\n\
        <identifier> Main </identifier>\n\
        <symbol> { </symbol>\n\
        <classVarDec>\n\
        <keyword> static </keyword>\n\
        <keyword> boolean </keyword>\n\
        <identifier> test </identifier>\n\
        <symbol> ; </symbol>\n\
        </classVarDec>\n\
        <classVarDec>\n\
        <keyword> static </keyword>\n\
        <keyword> boolean </keyword>\n\
        <identifier> test </identifier>\n\
        <symbol> ; </symbol>\n\
        </classVarDec>\n\
        <symbol> } </symbol>\n\
        </class>\n"
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
        let expected = "<class>\n\
        <keyword> class </keyword>\n\
        <identifier> Main </identifier>\n\
        <symbol> { </symbol>\n\
        <subroutineDec>\n\
        <keyword> function </keyword>\n\
        <keyword> void </keyword>\n\
        <identifier> main </identifier>\n\
        <symbol> ( </symbol>\n\
        <parameterList>\n\
        </parameterList>\n\
        <symbol> ) </symbol>\n\
        </subroutineDec>\n\
        <subroutineDec>\n\
        <keyword> function </keyword>\n\
        <keyword> boolean </keyword>\n\
        <identifier> isSomething </identifier>\n\
        <symbol> ( </symbol>\n\
        <parameterList>\n\
        </parameterList>\n\
        <symbol> ) </symbol>\n\
        </subroutineDec>\n\
        <symbol> } </symbol>\n\
        </class>\n"
            .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "class Main {{").unwrap();
        writeln!(src_file, "function void main()").unwrap();
        writeln!(src_file, "function boolean isSomething()").unwrap();
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
        let expected = "<classVarDec>\n\
        <keyword> static </keyword>\n\
        <keyword> boolean </keyword>\n\
        <identifier> test </identifier>\n\
        <symbol> ; </symbol>\n\
        </classVarDec>\n"
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
        let expected = "<subroutineDec>\n\
        <keyword> function </keyword>\n\
        <keyword> void </keyword>\n\
        <identifier> main </identifier>\n\
        <symbol> ( </symbol>\n\
        <parameterList>\n\
        </parameterList>\n\
        <symbol> ) </symbol>\n\
        </subroutineDec>\n"
            .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "function void main()").unwrap();
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
        <subroutineBody>\n\
        <symbol> { </symbol>\n\
        <varDec>\n\
        <keyword> var </keyword>\n\
        <identifier> Array </identifier>\n\
        <identifier> a </identifier>\n\
        <symbol> ; </symbol>\n\
        </varDec>\n\
        <varDec>\n\
        <keyword> var </keyword>\n\
        <keyword> int </keyword>\n\
        <identifier> length </identifier>\n\
        <symbol> ; </symbol>\n\
        </varDec>\n\
        <symbol> } </symbol>\n\
        </subroutineBody>\n"
            .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "{{").unwrap();
        writeln!(src_file, "var Array a;").unwrap();
        writeln!(src_file, "var int length;").unwrap();
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
        let expected = "<varDec>\n\
        <keyword> var </keyword>\n\
        <keyword> int </keyword>\n\
        <identifier> i </identifier>\n\
        <symbol> , </symbol>\n\
        <identifier> j </identifier>\n\
        <symbol> , </symbol>\n\
        <identifier> sum </identifier>\n\
        <symbol> ; </symbol>\n\
        </varDec>\n"
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
        let expected = "<letStatement>\n\
            <keyword> let </keyword>\n\
            <identifier> length </identifier>\n\
            <symbol> = </symbol>\n\
            <expression>\n\
            <term>\n\
            <identifier> Keyboard </identifier>\n\
            <symbol> . </symbol>\n\
            <identifier> readInt </identifier>\n\
            <symbol> ( </symbol>\n\
            <expressionList>\n\
            <expression>\n\
            <term>\n\
            <stringConstant> HOW MANY NUMBERS?  </stringConstant>\n\
            </term>\n\
            </expression>\n\
            </expressionList>\n\
            <symbol> ) </symbol>\n\
            </term>\n\
            </expression>\n\
            <symbol> ; </symbol>\n\
            </letStatement>\n"
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
    fn can_compile_term() {
        let expected = "\
            <term>\n\
            <identifier> Keyboard </identifier>\n\
            <symbol> . </symbol>\n\
            <identifier> readInt </identifier>\n\
            <symbol> ( </symbol>\n\
            <expressionList>\n\
            <expression>\n\
            <term>\n\
            <stringConstant> HOW MANY NUMBERS?  </stringConstant>\n\
            </term>\n\
            </expression>\n\
            </expressionList>\n\
            <symbol> ) </symbol>\n\
            </term>\n"
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
            <identifier> Keyboard </identifier>\n\
            <symbol> . </symbol>\n\
            <identifier> readInt </identifier>\n\
            <symbol> ( </symbol>\n\
            <expressionList>\n\
            <expression>\n\
            <term>\n\
            <stringConstant> HOW MANY NUMBERS?  </stringConstant>\n\
            </term>\n\
            </expression>\n\
            </expressionList>\n\
            <symbol> ) </symbol>\n"
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
