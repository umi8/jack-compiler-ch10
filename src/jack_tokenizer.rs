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
        if self.current_line.has_next() {
            return Ok(true);
        }

        loop {
            let mut buf = String::new();
            return match self.reader.read_line(&mut buf) {
                Ok(0) => Ok(false),
                Ok(_) => {
                    self.current_line = Line::new(buf);
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
        if self.current_line.has_next() {
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
        let index = self.current_index;
        self.current_index += 1;
        self.line.chars().collect::<Vec<char>>()[index]
    }

    fn has_next(&mut self) -> bool {
        self.current_index < self.max_len
    }
}
