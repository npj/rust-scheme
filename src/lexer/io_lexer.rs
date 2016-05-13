use super::Lexer;
use std::io::Read;
use std::io::BufReader;

pub struct IOLexer<T: Read> {
    input: BufReader<T>,
    buf:   [u8; 1],
    eof:   bool,
    line:  u32,
    chr:   u32
}

impl<T: Read> IOLexer<T> {
    pub fn new(input: T) -> IOLexer<T> {
        let mut lexer = IOLexer { input: BufReader::new(input), buf: [0], eof: false, line: 1, chr: 1 };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        match self.input.read(&mut self.buf) {
            Ok(0) | Err(_) => {
                self.eof = true;
            },
            Ok(_) => ()
        }
    }
}

impl<T: Read> Lexer for IOLexer<T> {
    fn get(&mut self) -> Option<char> {
        match self.peek() {
            None    => None,
            Some(c) => {
                self.read_char();
                self.count(c);
                Some(c)
            }
        }
    }

    fn peek(&self) -> Option<char> {
        if self.eof {
            None
        } else {
            Some(self.buf[0] as char)
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
    use std::io::Read;
    use std::io::Result;

    static TEST_STRING : &'static str = "ab\ncd";

    struct FakeFile {
        cursor: usize,
        data: Vec<u8>
    }

    impl FakeFile {
        fn new() -> FakeFile {
            FakeFile { cursor : 0, data: TEST_STRING.to_string().into_bytes() }
        }

        fn at(&self, i: usize) -> u8 {
            self.data[i]
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn move_cursor(&mut self, by: usize) {
            self.cursor = self.cursor + by;
        }

        fn cursor(&self) -> usize {
            self.cursor
        }
    }

    impl Read for FakeFile {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let mut count : usize = 0;

            for i in 0..(buf.len() - 1) {
                let offset = self.cursor() + i;
                if offset < self.len() {
                    buf[i] = self.at(offset);
                    count = count + 1;
                } else {
                    break;
                }
            }

            self.move_cursor(count);
            Ok(count)
        }
    }

    #[test]
    fn new() {
        let lexer = IOLexer::new(FakeFile::new());
        assert_eq!(lexer.eof, false);
        assert_eq!(lexer.line, 1);
        assert_eq!(lexer.chr, 1);
    }

    #[test]
    fn peek() {
        let mut lexer = IOLexer::new(FakeFile::new());
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
        let mut lexer = IOLexer::new(FakeFile::new());
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
        let mut lexer = IOLexer::new(FakeFile::new());
        assert_eq!(lexer.line(), 1);
        lexer.line = 43;
        assert_eq!(lexer.line(), 43);
    }

    #[test]
    fn set_line() {
        let mut lexer = IOLexer::new(FakeFile::new());
        assert_eq!(lexer.line(), 1);
        lexer.set_line(43);
        assert_eq!(lexer.line(), 43);
    }

    #[test]
    fn chr() {
        let mut lexer = IOLexer::new(FakeFile::new());
        assert_eq!(lexer.chr(), 1);
        lexer.chr = 43;
        assert_eq!(lexer.chr(), 43);
    }

    #[test]
    fn set_chr() {
        let mut lexer = IOLexer::new(FakeFile::new());
        assert_eq!(lexer.chr(), 1);
        lexer.chr = 43;
        assert_eq!(lexer.chr(), 43);
    }
}
