use super::Lexer;

pub struct StringLexer {
    input: Vec<u8>,
    index: usize,
    line:  u32,
    chr:   u32
}


impl StringLexer {
    pub fn new(input: String) -> StringLexer {
        StringLexer { input: input.into_bytes(), index: 0, line: 1, chr: 1 }
    }
}

impl Lexer for StringLexer {
    fn get(&mut self) -> Option<char> {
        match self.peek() {
            Some(c) => {
                self.index = self.index + 1;
                self.count(c);
                Some(c)
            },
            None => None
        }
    }

    fn peek(&mut self) -> Option<char> {
        if self.index < self.input.len() {
            Some(self.input[self.index] as char)
        } else {
            None
        }
    }

    fn set_line(&mut self, line: u32) -> () {
        self.line = line
    }

    fn set_chr(&mut self, chr: u32) -> () {
        self.chr = chr
    }

    fn line(&self) -> u32 {
        self.line
    }

    fn chr(&self) -> u32 {
        self.chr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::Lexer;

    static TEST_STRING : &'static str = "ab\ncd";

    #[test]
    fn new() {
        let lexer = StringLexer::new(TEST_STRING.to_string());
        assert_eq!(lexer.line, 1);
        assert_eq!(lexer.chr, 1);
    }

    #[test]
    fn peek() {
        let mut lexer = StringLexer::new(TEST_STRING.to_string());
        assert_eq!(Some('a'), lexer.peek());
        assert_eq!(lexer.chr, 1);
        assert_eq!(lexer.line, 1);
        assert_eq!(Some('a'), lexer.peek());
        assert_eq!(lexer.chr, 1);
        assert_eq!(lexer.line, 1);
        lexer.get();
        assert_eq!(Some('b'), lexer.peek());
        assert_eq!(lexer.chr, 2);
        assert_eq!(lexer.line, 1);
        assert_eq!(Some('b'), lexer.peek());
        assert_eq!(lexer.chr, 2);
        assert_eq!(lexer.line, 1);
    }

    #[test]
    fn get() {
        let mut lexer = StringLexer::new(TEST_STRING.to_string());
        let result = lexer.get();
        assert_eq!(Some('a'), result);
        assert_eq!(lexer.chr, 2);
        assert_eq!(lexer.line, 1);
        assert_eq!(Some('b'), lexer.get());
        assert_eq!(lexer.chr, 3);
        assert_eq!(lexer.line, 1);
        assert_eq!(Some('\n'), lexer.get());
        assert_eq!(lexer.chr, 1);
        assert_eq!(lexer.line, 2);
        assert_eq!(Some('c'), lexer.get());
        assert_eq!(lexer.chr, 2);
        assert_eq!(lexer.line, 2);
        assert_eq!(Some('d'), lexer.get());
        assert_eq!(lexer.chr, 3);
        assert_eq!(lexer.line, 2);
        assert_eq!(None, lexer.get());
    }

    #[test]
    fn line() {
        let mut lexer = StringLexer::new(TEST_STRING.to_string());
        assert_eq!(lexer.line(), 1);
        lexer.line = 43;
        assert_eq!(lexer.line(), 43);
    }

    #[test]
    fn set_line() {
        let mut lexer = StringLexer::new(TEST_STRING.to_string());
        assert_eq!(lexer.line(), 1);
        lexer.set_line(43);
        assert_eq!(lexer.line(), 43);
    }

    #[test]
    fn chr() {
        let mut lexer = StringLexer::new(TEST_STRING.to_string());
        assert_eq!(lexer.chr(), 1);
        lexer.chr = 43;
        assert_eq!(lexer.chr(), 43);
    }

    #[test]
    fn set_chr() {
        let mut lexer = StringLexer::new(TEST_STRING.to_string());
        assert_eq!(lexer.chr(), 1);
        lexer.chr = 43;
        assert_eq!(lexer.chr(), 43);
    }
}
