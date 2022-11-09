use std::fmt;
use std::fmt::Formatter;

use anyhow::{bail, Error, Result};

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
    pub fn from(key_word: &str) -> Result<KeyWord> {
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

pub const KEYWORDS: [&str; 21] = [
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
