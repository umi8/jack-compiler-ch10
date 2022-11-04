use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

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
        let mut ch = self.current_line.peek();
        if ch == '/' {
            self.current_line.next();
            ch = self.current_line.peek();
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
        } else {
            println!("{}", self.current_line.next());
        }
        Ok(())
    }
}

struct Line {
    line: String,
    current_index: usize,
    max_len: usize,
}

impl Line {
    fn new(line: String) -> Self {
        let len = line.len();
        Line {
            line,
            current_index: 0,
            max_len: len,
        }
    }

    fn next(&mut self) -> char {
        if !self.has_next() {
            return '\0';
        }
        let index = self.current_index;
        self.current_index += 1;
        self.line.chars().collect::<Vec<char>>()[index]
    }

    fn peek(&mut self) -> char {
        if !self.has_next() {
            return '\0';
        }
        self.line.chars().collect::<Vec<char>>()[self.current_index]
    }

    fn has_next(&mut self) -> bool {
        self.current_index < self.max_len
    }

    fn skip_whitespace(&mut self) -> () {
        if !self.has_next() {
            return;
        }
        let mut ch = self.peek();
        while ch.is_whitespace() {
            self.next();
            if !self.has_next() {
                break;
            }
            ch = self.peek();
        }
    }

    fn move_cursor_to_end_of_line(&mut self) -> () {
        self.current_index = self.max_len;
    }
}
