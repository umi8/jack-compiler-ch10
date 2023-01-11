use std::collections::vec_deque::VecDeque;
use std::fs;
use std::path::Path;

use anyhow::{bail, Context, Error, Result};

use crate::tokenizer::key_word::KEYWORDS;
use crate::tokenizer::token::Token;
use crate::tokenizer::token_type::TokenType;

pub struct JackTokenizer {
    tokens: VecDeque<Token>,
    current_token: Token,
}

impl JackTokenizer {
    pub fn new(path: &Path) -> Result<Self> {
        let code = fs::read_to_string(path)?;
        let code_without_comments = Self::remove_comments(code)?;
        let tokens = Self::tokenize(code_without_comments)?;

        Ok(JackTokenizer {
            tokens,
            current_token: Default::default(),
        })
    }

    pub fn has_more_tokens(&mut self) -> Result<bool> {
        Ok(!self.tokens.is_empty())
    }

    pub fn advance(&mut self) -> Result<()> {
        if self.has_more_tokens()? {
            self.current_token = self.tokens.pop_front().context("pop failed.")?
        } else {
            bail!(Error::msg("pop failed."))
        }
        Ok(())
    }

    pub fn peek(&mut self) -> Result<&Token> {
        if self.has_more_tokens()? {
            self.tokens.get(0).context("get failed.")
        } else {
            bail!(Error::msg("get failed."))
        }
    }

    pub fn token_type(&mut self) -> Result<&TokenType> {
        Ok(self.current_token.token_type())
    }

    pub fn value(&mut self) -> Result<&String> {
        Ok(self.current_token.value())
    }

    fn remove_comments(code: String) -> Result<String> {
        let mut code_without_comments = String::new();
        let mut current_index = 0;

        while current_index < code.len() {
            let current_char = &code[current_index..current_index + 1];

            match current_char {
                "/" => {
                    let next_char = &code[current_index + 1..current_index + 2];
                    match next_char {
                        "/" => {
                            let end_index = code[current_index + 1..]
                                .find('\n')
                                .map(|i| i + current_index + 1)
                                .context("map failed.")?;
                            current_index = end_index + 1;
                        }
                        "*" => {
                            let end_index = code[current_index + 1..]
                                .find("*/")
                                .map(|i| i + current_index + 1)
                                .context("map failed.")?;
                            current_index = end_index + 2;
                        }
                        _ => {
                            code_without_comments += &*String::from(current_char);
                            current_index += 1;
                        }
                    }
                }
                "\r" | "\n" => current_index += 1,
                _ => {
                    code_without_comments += &*String::from(current_char);
                    current_index += 1;
                }
            }
        }
        Ok(code_without_comments)
    }

    fn tokenize(source: String) -> Result<VecDeque<Token>> {
        let mut tokens: VecDeque<Token> = VecDeque::new();

        let mut index = 0;
        let chars: Vec<char> = source.chars().collect();
        while index < chars.len() {
            let current = chars[index];
            match current {
                '\"' => {
                    index += 1;
                    let (token, index_after_tokenize) = Self::tokenize_string_const(index, &chars)?;
                    tokens.push_back(token);
                    index = index_after_tokenize;
                }
                current if SYMBOLS.contains(&current) => {
                    tokens.push_back(Token::new(TokenType::Symbol, String::from(current)));
                    index += 1;
                }
                current if current.is_alphabetic() => {
                    let (token, index_after_tokenize) =
                        Self::tokenize_keyword_and_identifier(index, &chars)?;
                    tokens.push_back(token);
                    index = index_after_tokenize;
                }
                current if current.is_numeric() => {
                    let (token, index_after_tokenize) = Self::tokenize_int_const(index, &chars)?;
                    tokens.push_back(token);
                    index = index_after_tokenize;
                }
                _ => index += 1,
            }
        }
        Ok(tokens)
    }

    fn tokenize_string_const(mut index: usize, chars: &Vec<char>) -> Result<(Token, usize)> {
        let mut value = String::new();
        while index < chars.len() && chars[index] != '\"' {
            value.push(chars[index]);
            index += 1;
        }
        index += 1;
        Ok((Token::new(TokenType::StringConst, value), index))
    }

    fn tokenize_keyword_and_identifier(
        mut index: usize,
        chars: &Vec<char>,
    ) -> Result<(Token, usize)> {
        let mut value = String::new();
        while index < chars.len() && chars[index].is_alphabetic() {
            value.push(chars[index]);
            index += 1;
            if KEYWORDS.contains(&value.as_str()) {
                return Ok((Token::new(TokenType::Keyword, value), index));
            }
        }
        Ok((Token::new(TokenType::Identifier, value), index))
    }

    fn tokenize_int_const(mut index: usize, chars: &Vec<char>) -> Result<(Token, usize)> {
        let mut value = String::new();
        while index < chars.len() && chars[index].is_numeric() {
            value.push(chars[index]);
            index += 1;
        }
        Ok((Token::new(TokenType::IntConst, value), index))
    }
}

const SYMBOLS: [char; 19] = [
    '{', '}', '(', ')', '[', ']', '.', ',', ';', '+', '-', '*', '/', '&', '|', '<', '>', '=', '~',
];
