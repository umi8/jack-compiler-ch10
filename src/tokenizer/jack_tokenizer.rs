use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{bail, Error, Result};

use crate::tokenizer::line::Line;

pub struct JackTokenizer {
    reader: BufReader<File>,
    current_line: Line,
    token: Token,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Keyword,
    Symbol,
    Identifier,
    IntConst,
}

struct Token {
    token_type: TokenType,
    value: String,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            token_type: TokenType::Keyword,
            value: "".to_string(),
        }
    }
}

impl JackTokenizer {
    pub fn new(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        Ok(JackTokenizer {
            reader: BufReader::new(file),
            current_line: Default::default(),
            token: Default::default(),
        })
    }

    pub fn has_more_tokens(&mut self) -> Result<bool> {
        self.current_line.skip_whitespace();
        if self.current_line.has_next() {
            return Ok(true);
        }

        loop {
            let mut buf = String::new();
            return match self.reader.read_line(&mut buf) {
                Ok(0) => Ok(false),
                Ok(_) => {
                    self.current_line = Line::new(buf.trim().to_string());
                    if !self.current_line.has_next() {
                        continue;
                    } else {
                        return Ok(true);
                    }
                }
                Err(_) => Ok(false),
            };
        }
    }

    pub fn advance(&mut self) -> Result<()> {
        let ch = self.current_line.peek();
        if ch == '/' {
            self.ignore_comments()?;
            if self.has_more_tokens()? {
                self.advance()?;
            }
        } else if SYMBOLS.contains(&ch) {
            self.token = Token {
                token_type: TokenType::Symbol,
                value: self.current_line.next().to_string(),
            }
        } else if ch.is_numeric() {
            self.analyze_integer_constant();
        } else if ch.is_alphabetic() {
            self.analyze_alphabetic();
        }
        Ok(())
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token.token_type
    }

    pub fn key_word(&self) -> Result<KeyWord> {
        KeyWord::from(self.token.value.as_str())
    }

    pub fn symbol(&self) -> char {
        self.token.value.parse().unwrap()
    }

    pub fn identifier(&self) -> &String {
        &self.token.value
    }

    pub fn int_val(&self) -> Result<usize> {
        Ok(self.token.value.parse::<usize>()?)
    }

    fn ignore_comments(&mut self) -> Result<()> {
        self.current_line.next();
        let mut ch = self.current_line.peek();

        if ch == '/' {
            self.current_line.move_cursor_to_end_of_line();
        } else if ch == '*' {
            loop {
                self.current_line.next();
                ch = self.current_line.peek();

                while ch != '*' {
                    if !self.current_line.has_next() {
                        let mut buf = String::new();
                        match self.reader.read_line(&mut buf) {
                            Ok(0) => bail!(Error::msg("Format Error.")),
                            Ok(_) => {
                                self.current_line = Line::new(buf.trim().to_string());
                            }
                            Err(_) => bail!(Error::msg("Format Error.")),
                        };
                        ch = self.current_line.peek();
                        continue;
                    }

                    self.current_line.next();
                    ch = self.current_line.peek();
                }

                self.current_line.next();
                if self.current_line.peek() == '/' {
                    break;
                }
            }
        }
        Ok(())
    }

    fn analyze_alphabetic(&mut self) {
        let mut ch = self.current_line.peek();
        let mut value = String::new();
        while self.current_line.has_next() && ch.is_alphabetic() {
            value.push(self.current_line.next());
            if KEYWORDS.contains(&value.as_str()) {
                self.token = Token {
                    token_type: TokenType::Keyword,
                    value,
                };
                return;
            }
            ch = self.current_line.peek();
        }
        self.token = Token {
            token_type: TokenType::Identifier,
            value,
        }
    }

    fn analyze_integer_constant(&mut self) {
        let mut ch = self.current_line.peek();
        let mut value = String::new();
        while self.current_line.has_next() && ch.is_numeric() {
            value.push(self.current_line.next());
            ch = self.current_line.peek();
        }
        self.token = Token {
            token_type: TokenType::IntConst,
            value,
        }
    }
}

#[derive(Debug)]
pub enum KeyWord {
    Class,
    Method,
    Function,
    Constructor,
    Int,
    Boolean,
    Char,
    Void,
    Var,
    Static,
    Field,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Null,
    This,
}

impl fmt::Display for KeyWord {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl KeyWord {
    fn from(key_word: &str) -> Result<KeyWord> {
        match key_word {
            "class" => Ok(KeyWord::Class),
            "constructor" => Ok(KeyWord::Constructor),
            "function" => Ok(KeyWord::Function),
            "method" => Ok(KeyWord::Method),
            "field" => Ok(KeyWord::Field),
            "static" => Ok(KeyWord::Static),
            "var" => Ok(KeyWord::Var),
            "int" => Ok(KeyWord::Int),
            "char" => Ok(KeyWord::Char),
            "boolean" => Ok(KeyWord::Boolean),
            "void" => Ok(KeyWord::Void),
            "true" => Ok(KeyWord::True),
            "false" => Ok(KeyWord::False),
            "null" => Ok(KeyWord::Null),
            "this" => Ok(KeyWord::This),
            "let" => Ok(KeyWord::Let),
            "do" => Ok(KeyWord::Do),
            "if" => Ok(KeyWord::If),
            "else" => Ok(KeyWord::Else),
            "while" => Ok(KeyWord::While),
            "return" => Ok(KeyWord::Return),
            _ => bail!(Error::msg(format!("Illegal Argument Error: {}", key_word))),
        }
    }
}

const KEYWORDS: [&str; 21] = [
    "class",
    "constructor",
    "function",
    "method",
    "field",
    "static",
    "var",
    "int",
    "char",
    "boolean",
    "void",
    "true",
    "false",
    "null",
    "this",
    "let",
    "do",
    "if",
    "else",
    "while",
    "return",
];

const SYMBOLS: [char; 19] = [
    '{', '}', '(', ')', '[', ']', '.', ',', ';', '+', '-', '*', '/', '&', '|', '<', '>', '=', '~',
];

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::io::{BufRead, BufReader, Seek, SeekFrom};

    use tempfile::tempfile;

    use crate::tokenizer::jack_tokenizer::{JackTokenizer, TokenType};
    use crate::tokenizer::line::Line;

    #[test]
    fn advance_if_keyword() {
        let mut tokenizer = JackTokenizer {
            reader: BufReader::new(tempfile::tempfile().unwrap()),
            current_line: Line::new("class Square {".to_string()),
            token: Default::default(),
        };

        tokenizer.advance().unwrap();
        assert_eq!("class", tokenizer.token.value);
        assert_eq!(TokenType::Keyword, tokenizer.token.token_type);
    }

    #[test]
    fn advance_if_identifier() {
        let mut tokenizer = JackTokenizer {
            reader: BufReader::new(tempfile::tempfile().unwrap()),
            current_line: Line::new("Square {".to_string()),
            token: Default::default(),
        };

        tokenizer.advance().unwrap();
        assert_eq!("Square", tokenizer.token.value);
        assert_eq!(TokenType::Identifier, tokenizer.token.token_type);
    }

    #[test]
    fn advance_if_symbol() {
        let mut tokenizer = JackTokenizer {
            reader: BufReader::new(tempfile::tempfile().unwrap()),
            current_line: Line::new("{ code; }".to_string()),
            token: Default::default(),
        };

        tokenizer.advance().unwrap();
        assert_eq!("{", tokenizer.token.value);
        assert_eq!(TokenType::Symbol, tokenizer.token.token_type);
    }

    #[test]
    fn advance_if_numeric() {
        let mut tokenizer = JackTokenizer {
            reader: BufReader::new(tempfile::tempfile().unwrap()),
            current_line: Line::new("123 + 24".to_string()),
            token: Default::default(),
        };

        tokenizer.advance().unwrap();
        assert_eq!("123", tokenizer.token.value);
        assert_eq!(TokenType::IntConst, tokenizer.token.token_type);
    }

    #[test]
    fn can_ignore_line_if_double_slash_comment() {
        let mut tokenizer = JackTokenizer {
            reader: BufReader::new(tempfile::tempfile().unwrap()),
            current_line: Line::new("// comments".to_string()),
            token: Default::default(),
        };

        tokenizer.ignore_comments().unwrap();
        assert!(!tokenizer.current_line.has_next());
    }

    #[test]
    fn can_ignore_until_end_of_comment() {
        let mut tokenizer = JackTokenizer {
            reader: BufReader::new(tempfile::tempfile().unwrap()),
            current_line: Line::new("/* comments */ code;".to_string()),
            token: Default::default(),
        };

        tokenizer.current_line.next();
        tokenizer.ignore_comments().unwrap();
        assert!(tokenizer.current_line.has_next());
        assert_eq!(' ', tokenizer.current_line.peek());
    }

    #[test]
    fn can_ignore_until_end_of_comment_if_api_doc_comment() {
        let mut file = tempfile().unwrap();
        writeln!(file, "/**").unwrap();
        writeln!(file, " * ").unwrap();
        writeln!(file, " * multiline comments").unwrap();
        writeln!(file, " * multiline comments").unwrap();
        writeln!(file, " */").unwrap();
        writeln!(file, "code;").unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();

        let mut buf = String::new();
        let mut reader = BufReader::new(file);
        reader.read_line(&mut buf).unwrap();

        let mut tokenizer = JackTokenizer {
            reader,
            current_line: Line::new(buf),
            token: Default::default(),
        };

        tokenizer.current_line.next();
        tokenizer.ignore_comments().unwrap();

        assert!(tokenizer.current_line.has_next());
        assert_eq!('/', tokenizer.current_line.peek());
    }
}
