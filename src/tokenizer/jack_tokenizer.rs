use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::Result;

use crate::tokenizer::line::Line;

pub struct JackTokenizer {
    reader: BufReader<File>,
    current_line: Line,
}

impl JackTokenizer {
    pub fn new(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        Ok(JackTokenizer {
            reader: BufReader::new(file),
            current_line: Line::new(String::new()),
        })
    }

    pub fn has_more_tokens(&mut self) -> Result<bool> {
        self.current_line.skip_whitespace();
        if self.current_line.has_next() {
            return Ok(true);
        }

        loop {
            let mut buf = String::new();
            return match self.reader.read_line(&mut buf) {
                Ok(0) => Ok(false),
                Ok(_) => {
                    self.current_line = Line::new(buf.trim().to_string());
                    if !self.current_line.has_next() {
                        continue;
                    } else {
                        return Ok(true);
                    }
                }
                Err(_) => Ok(false),
            };
        }
    }

    pub fn advance(&mut self) -> Result<()> {
        let ch = self.current_line.peek();
        if ch == '/' {
            self.ignore_comments();
        } else {
            println!("{}", self.current_line.next());
        }
        Ok(())
    }

    fn ignore_comments(&mut self) -> () {
        self.current_line.next();
        let mut ch = self.current_line.peek();
        if ch == '/' {
            self.current_line.move_cursor_to_end_of_line();
        } else if ch == '*' {
            self.current_line.next();
            let mut is_end = false;
            while self.current_line.has_next() {
                ch = self.current_line.peek();
                if ch != '/' && is_end {
                    is_end = false;
                }
                if ch == '*' {
                    is_end = true;
                }
                if ch == '/' && is_end {
                    self.current_line.next();
                    break;
                }
                self.current_line.next();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::tokenizer::line::Line;
    use crate::JackTokenizer;

    #[test]
    fn can_ignore_line_if_double_slash_comment() {
        let mut tokenizer = JackTokenizer {
            reader: BufReader::new(tempfile::tempfile().unwrap()),
            current_line: Line::new("// comments".to_string()),
        };

        tokenizer.ignore_comments();
        assert!(!tokenizer.current_line.has_next());
    }

    #[test]
    fn can_ignore_until_end_of_comment() {
        let mut tokenizer = JackTokenizer {
            reader: BufReader::new(tempfile::tempfile().unwrap()),
            current_line: Line::new("/* comments */ code;".to_string()),
        };

        tokenizer.ignore_comments();
        assert!(tokenizer.current_line.has_next());
        assert_eq!(' ', tokenizer.current_line.peek());
    }

    #[test]
    fn can_ignore_until_end_of_comment_if_api_doc_comment() {
        let mut tokenizer = JackTokenizer {
            reader: BufReader::new(tempfile::tempfile().unwrap()),
            current_line: Line::new("/** api doc comments */ code;".to_string()),
        };

        tokenizer.ignore_comments();
        assert!(tokenizer.current_line.has_next());
        assert_eq!(' ', tokenizer.current_line.peek());
    }
}
