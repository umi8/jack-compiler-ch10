use crate::tokenizer::key_word::KeyWord;
use crate::tokenizer::token_type::TokenType;
use anyhow::Result;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Token {
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

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Self {
        Token { token_type, value }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn is_type(&self) -> Result<bool> {
        match self.token_type {
            TokenType::Keyword => match KeyWord::from(&self.value)? {
                KeyWord::Int | KeyWord::Char | KeyWord::Boolean => Ok(true),
                _ => Ok(false),
            },
            TokenType::Symbol | TokenType::IntConst | TokenType::StringConst => Ok(false),
            TokenType::Identifier => Ok(true),
        }
    }

    pub fn is_op(&self) -> bool {
        matches!(
            self.value.as_str(),
            "+" | "-" | "*" | "/" | "&" | "|" | "<" | ">" | "="
        )
    }

    pub fn is_keyword_constant(&self) -> Result<bool> {
        if !KeyWord::exists(&self.value) {
            return Ok(false);
        }
        match KeyWord::from(&self.value)? {
            KeyWord::True | KeyWord::False | KeyWord::Null | KeyWord::This => Ok(true),
            _ => Ok(false),
        }
    }
}
