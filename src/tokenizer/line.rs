pub(crate) struct Line {
    line: String,
    current_index: usize,
    max_len: usize,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            line: "".to_string(),
            current_index: 0,
            max_len: 0,
        }
    }
}

impl Line {
    pub(crate) fn new(line: String) -> Self {
        let len = line.len();
        Line {
            line,
            current_index: 0,
            max_len: len,
        }
    }

    pub(crate) fn next(&mut self) -> char {
        if !self.has_next() {
            return '\0';
        }
        let index = self.current_index;
        self.current_index += 1;
        self.line.chars().collect::<Vec<char>>()[index]
    }

    pub(crate) fn peek(&mut self) -> char {
        if !self.has_next() {
            return '\0';
        }
        self.line.chars().collect::<Vec<char>>()[self.current_index]
    }

    pub(crate) fn has_next(&mut self) -> bool {
        self.current_index < self.max_len
    }

    pub(crate) fn skip_whitespace(&mut self) {
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

    pub(crate) fn move_cursor_to_end_of_line(&mut self) {
        self.current_index = self.max_len;
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::line::Line;

    #[test]
    fn next_return_current_char_and_increment_index_if_has_next() {
        let mut line = Line {
            line: "apple".to_string(),
            current_index: 0,
            max_len: 5,
        };
        assert_eq!('a', line.next());
        assert_eq!(1, line.current_index);
    }

    #[test]
    fn next_return_empty_char_if_not_have_next() {
        let mut line = Line {
            line: "apple".to_string(),
            current_index: 5,
            max_len: 5,
        };
        assert_eq!('\0', line.next());
        assert_eq!(5, line.current_index);
    }

    #[test]
    fn peek_return_current_char_and_increment_index_if_has_next() {
        let mut line = Line {
            line: "apple".to_string(),
            current_index: 0,
            max_len: 5,
        };
        assert_eq!('a', line.peek());
        assert_eq!(0, line.current_index);
    }

    #[test]
    fn peek_return_empty_char_if_not_have_next() {
        let mut line = Line {
            line: "apple".to_string(),
            current_index: 5,
            max_len: 5,
        };
        assert_eq!('\0', line.peek());
        assert_eq!(5, line.current_index);
    }

    #[test]
    fn can_skip_whitespace() {
        let mut line = Line {
            line: "space    Line".to_string(),
            current_index: 5,
            max_len: 13,
        };
        line.skip_whitespace();
        assert_eq!(9, line.current_index);
        assert_eq!('L', line.peek());
    }
}
