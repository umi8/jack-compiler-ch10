use std::fs::read_to_string;
use std::path::Path;
use std::string::String;

use jack_compiler::compilation::compilation_engine::{CompilationEngine, XmlCompilationEngine};
use jack_compiler::tokenizer::jack_tokenizer::JackTokenizer;

#[test]
fn square_main() {
    let expected_file_path = Path::new("tests/resources/Square/Main.xml");
    let src_path = Path::new("tests/resources/Square/Main.jack");
    test_diff(expected_file_path, src_path);
}

#[test]
fn square_square() {
    let expected_file_path = Path::new("tests/resources/Square/Square.xml");
    let src_path = Path::new("tests/resources/Square/Square.jack");
    test_diff(expected_file_path, src_path);
}

#[test]
fn square_square_game() {
    let expected_file_path = Path::new("tests/resources/Square/SquareGame.xml");
    let src_path = Path::new("tests/resources/Square/SquareGame.jack");
    test_diff(expected_file_path, src_path);
}

#[test]
fn array_test_main() {
    let expected_file_path = Path::new("tests/resources/ArrayTest/Main.xml");
    let src_path = Path::new("tests/resources/ArrayTest/Main.jack");
    test_diff(expected_file_path, src_path);
}

fn test_diff(expected_file_path: &Path, src_file_path: &Path) {
    let expected = read_to_string(expected_file_path).unwrap();
    let jack_tokenizer = JackTokenizer::new(src_file_path).unwrap();
    let mut compilation_engine = XmlCompilationEngine::new(jack_tokenizer);
    let mut output = Vec::<u8>::new();
    let result = compilation_engine.compile_class(&mut output);
    let actual = String::from_utf8(output).unwrap();
    assert!(result.is_ok());
    assert_eq!(expected, actual)
}
