use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{bail, Context, Error, Result};

use crate::tokenizer::key_word::{KeyWord, KEYWORDS};
use crate::tokenizer::line::Line;
use crate::tokenizer::token::Token;
use crate::tokenizer::token_type::TokenType;

pub struct JackTokenizer {}

impl JackTokenizer {
    pub fn new(path: &Path) -> Result<Self> {
        let code = fs::read_to_string(path)?;
        let code_without_comments = Self::remove_comments(code)?;
        let tokens = Self::tokenize(code_without_comments)?;

        for t in tokens {
            println!("{:?}", t);
        }

        Ok(JackTokenizer {})
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
                                .find("\n")
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

    fn tokenize(source: String) -> Result<Vec<Token>> {
        let mut tokens: Vec<Token> = Vec::new();

        let mut index = 0;
        let chars: Vec<char> = source.chars().collect();
        while index < chars.len() {
            let current = chars[index];
            match current {
                '\"' => {
                    index += 1;
                    let mut value = String::new();
                    while index < chars.len() && chars[index] != '\"' {
                        value.push(chars[index]);
                        index += 1;
                    }
                    tokens.push(Token::new(TokenType::StringConst, value));
                }
                current if SYMBOLS.contains(&current) => {
                    tokens.push(Token::new(TokenType::Symbol, String::from(current)));
                    index += 1;
                }
                current if current.is_alphabetic() => {
                    let mut value = String::new();
                    while index < chars.len() && chars[index].is_alphabetic() {
                        value.push(chars[index]);
                        if KEYWORDS.contains(&value.as_str()) {
                            tokens.push(Token::new(TokenType::Keyword, value));
                            break;
                        }
                        index += 1;
                    }
                    // tokens.push(Token::new(TokenType::Keyword, value));
                }
                current if current.is_numeric() => {
                    let mut value = String::new();
                    while index < chars.len() && chars[index].is_numeric() {
                        value.push(chars[index]);
                        index += 1;
                    }
                    tokens.push(Token::new(TokenType::IntConst, value));
                }
                _ => {}
            }
            index += 1;
        }
        Ok(tokens)
    }
}

const SYMBOLS: [char; 19] = [
    '{', '}', '(', ')', '[', ']', '.', ',', ';', '+', '-', '*', '/', '&', '|', '<', '>', '=', '~',
];
