use std::fmt::Write;
use std::fs::read_to_string;
use std::path::Path;
use std::string::String;

use jack_compiler::tokenizer::jack_tokenizer::{JackTokenizer, TokenType};

#[test]
fn square() {
    let expected_file_path = Path::new("tests/resources/Square/SquareT.xml");
    let expected = read_to_string(expected_file_path).unwrap();
    let src_path = Path::new("tests/resources/Square/Square.jack");
    let mut jack_tokenizer = JackTokenizer::new(src_path).unwrap();

    let mut actual = String::new();
    writeln!(actual, "<tokens>").unwrap();
    while jack_tokenizer.has_more_tokens().unwrap() {
        jack_tokenizer.advance().unwrap();
        match jack_tokenizer.token_type() {
            TokenType::Keyword => writeln!(
                actual,
                "<keyword> {} </keyword>",
                jack_tokenizer
                    .key_word()
                    .unwrap()
                    .to_string()
                    .to_lowercase()
            )
            .unwrap(),
            TokenType::Symbol => {
                let symbol = match jack_tokenizer.symbol() {
                    '<' => "&lt;",
                    '>' => "&gt;",
                    '&' => "&amp;",
                    _ => "",
                };

                if symbol == "" {
                    writeln!(actual, "<symbol> {} </symbol>", jack_tokenizer.symbol()).unwrap()
                } else {
                    writeln!(actual, "<symbol> {} </symbol>", symbol).unwrap()
                }
            }
            TokenType::Identifier => writeln!(
                actual,
                "<identifier> {} </identifier>",
                jack_tokenizer.identifier()
            )
            .unwrap(),
            TokenType::IntConst => writeln!(
                actual,
                "<integerConstant> {} </integerConstant>",
                jack_tokenizer.int_val().unwrap()
            )
            .unwrap(),
        }
    }
    writeln!(actual, "</tokens>").unwrap();

    assert_eq!(expected, actual)
}
