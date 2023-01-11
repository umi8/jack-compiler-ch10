use crate::tokenizer::token_type::TokenType;
use std::fmt::{Debug, Formatter};

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
}
