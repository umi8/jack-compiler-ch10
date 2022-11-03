use anyhow::Result;

pub struct JackTokenizer {}

impl JackTokenizer {
    pub fn has_more_tokens() -> Result<bool> {
        Ok(false)
    }

    pub fn token_type() -> Result<TokenType> {
        Ok(TokenType::Keyword)
    }

    pub fn key_word() -> Result<KeyWord> {
        Ok(KeyWord::Class)
    }

    pub fn symbol() -> Result<char> {
        Ok('{')
    }

    pub fn identifier() -> Result<String> {
        Ok(String::new())
    }

    pub fn int_val() -> Result<usize> {
        Ok(0)
    }

    pub fn string_val() -> Result<String> {
        Ok(String::new())
    }
}

enum TokenType {
    Keyword,
    Symbol,
    Identifier,
    IntConst,
    StringConst,
}

enum KeyWord {
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
