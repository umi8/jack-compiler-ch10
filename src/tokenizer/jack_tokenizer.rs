use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::tokenizer::line::Line;
use anyhow::Result;

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
