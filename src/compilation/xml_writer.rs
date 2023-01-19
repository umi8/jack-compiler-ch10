use std::io::Write;

use anyhow::{bail, Error, Result};

use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord;
use crate::tokenizer::token_type::TokenType;

pub struct XmlWriter {
    indent: String,
}

const INDENT_COUNT: usize = 2;

impl XmlWriter {
    pub fn new() -> Self {
        XmlWriter {
            indent: String::new(),
        }
    }

    pub fn write_key_word(
        &mut self,
        tokenizer: &mut JackTokenizer,
        targets: Vec<KeyWord>,
        written: &mut impl Write,
    ) -> Result<()> {
        tokenizer.advance()?;
        match tokenizer.token_type()? {
            TokenType::Keyword => {
                let keyword = tokenizer.key_word()?;
                match keyword {
                    keyword if targets.contains(&keyword) => writeln!(
                        written,
                        "{}<keyword> {} </keyword>",
                        self.indent,
                        tokenizer.key_word()?.to_string().to_lowercase()
                    )?,
                    _ => bail!(Error::msg("Illegal token")),
                }
            }
            _ => bail!(Error::msg("Illegal token")),
        }
        Ok(())
    }

    pub fn write_identifier(
        &mut self,
        tokenizer: &mut JackTokenizer,
        written: &mut impl Write,
    ) -> Result<()> {
        tokenizer.advance()?;
        match tokenizer.token_type()? {
            TokenType::Identifier => writeln!(
                written,
                "{}<identifier> {} </identifier>",
                self.indent,
                tokenizer.identifier()
            )?,
            _ => bail!(Error::msg("Illegal token")),
        }
        Ok(())
    }

    pub fn write_symbol(
        &mut self,
        tokenizer: &mut JackTokenizer,
        written: &mut impl Write,
    ) -> Result<()> {
        tokenizer.advance()?;
        match tokenizer.token_type()? {
            TokenType::Symbol => {
                let symbol = match tokenizer.symbol() {
                    '<' => "&lt;",
                    '>' => "&gt;",
                    '&' => "&amp;",
                    _ => "",
                };

                if symbol.is_empty() {
                    writeln!(
                        written,
                        "{}<symbol> {} </symbol>",
                        self.indent,
                        tokenizer.symbol()
                    )?
                } else {
                    writeln!(written, "{}<symbol> {} </symbol>", self.indent, symbol)?
                }
            }
            _ => bail!(Error::msg("Illegal token")),
        }
        Ok(())
    }

    pub fn write_string_constant(
        &mut self,
        tokenizer: &mut JackTokenizer,
        written: &mut impl Write,
    ) -> Result<()> {
        tokenizer.advance()?;
        match tokenizer.token_type()? {
            TokenType::StringConst => writeln!(
                written,
                "{}<stringConstant> {} </stringConstant>",
                self.indent,
                tokenizer.string_val()
            )?,
            _ => bail!(Error::msg("Illegal token")),
        }
        Ok(())
    }

    pub fn write_integer_constant(
        &mut self,
        tokenizer: &mut JackTokenizer,
        written: &mut impl Write,
    ) -> Result<()> {
        tokenizer.advance()?;
        match tokenizer.token_type()? {
            TokenType::IntConst => writeln!(
                written,
                "{}<integerConstant> {} </integerConstant>",
                self.indent,
                tokenizer.int_val()?
            )?,
            _ => bail!(Error::msg("Illegal token")),
        }
        Ok(())
    }

    pub fn write_start_tag(&mut self, element: &str, written: &mut impl Write) -> Result<()> {
        writeln!(written, "{}<{}>", self.indent, element)?;
        self.increase_indent();
        Ok(())
    }

    pub fn write_end_tag(&mut self, element: &str, written: &mut impl Write) -> Result<()> {
        self.decrease_indent();
        writeln!(written, "{}</{}>", self.indent, element)?;
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
