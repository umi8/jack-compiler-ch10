use std::fmt::Write;
use std::fs::read_to_string;
use std::path::Path;
use std::string::String;

use jack_compiler::tokenizer::jack_tokenizer::JackTokenizer;
use jack_compiler::tokenizer::token_type::TokenType;

#[test]
fn square_main() {
    let expected_file_path = Path::new("tests/resources/Square/MainT.xml");
    let src_path = Path::new("tests/resources/Square/Main.jack");
    test_diff(expected_file_path, src_path);
}

#[test]
fn square_square() {
    let expected_file_path = Path::new("tests/resources/Square/SquareT.xml");
    let src_path = Path::new("tests/resources/Square/Square.jack");
    test_diff(expected_file_path, src_path);
}

#[test]
fn square_square_game() {
    let expected_file_path = Path::new("tests/resources/Square/SquareGameT.xml");
    let src_path = Path::new("tests/resources/Square/SquareGame.jack");
    test_diff(expected_file_path, src_path);
}

#[test]
fn array_test_main() {
    let expected_file_path = Path::new("tests/resources/ArrayTest/MainT.xml");
    let src_path = Path::new("tests/resources/ArrayTest/Main.jack");
    test_diff(expected_file_path, src_path);
}

fn test_diff(expected_file_path: &Path, src_file_path: &Path) {
    let expected = read_to_string(expected_file_path).unwrap();
    let mut jack_tokenizer = JackTokenizer::new(src_file_path).unwrap();

    let mut actual = String::new();
    writeln!(actual, "<tokens>").unwrap();
    while jack_tokenizer.has_more_tokens().unwrap() {
        jack_tokenizer.advance().unwrap();
        match jack_tokenizer.token_type().unwrap() {
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

                if symbol.is_empty() {
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
            TokenType::StringConst => writeln!(
                actual,
                "<stringConstant> {} </stringConstant>",
                jack_tokenizer.string_val()
            )
            .unwrap(),
        }
    }
    writeln!(actual, "</tokens>").unwrap();

    assert_eq!(expected, actual)
}
