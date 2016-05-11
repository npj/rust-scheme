use std::io::Read;
use std::io::BufReader;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;

#[derive(PartialEq, Debug)]
pub enum Token {
    LPAR,
    RPAR,
    COMMENT(String),
    STRING(String),
    INTEGER(String),
    IDENT(String)
}

#[derive(Debug)]
pub enum LexError {
    INVALID(char, u32, u32),
    UNTERMINATED(String, u32, u32),
    END(u32, u32)
}

pub trait Lexer {
    fn get(&mut self) -> Option<char>;
    fn peek(&mut self) -> Option<char>;
    fn set_line(&mut self, line: u32) -> ();
    fn set_chr(&mut self, chr: u32) -> ();
    fn line(&self) -> u32;
    fn chr(&self) -> u32;

    fn count(&mut self, c: char) -> () {
        let line = self.line();
        let chr  = self.chr();

        match c {
            '\n' => {
                self.set_line(line + 1);
                self.set_chr(1);
            },
            _ => {
                self.set_chr(chr + 1);
            }
        }
    }

    fn next(&mut self) -> Result<Token, LexError> {
        self.consume_whitespace();
        match self.peek() {
            None    => Err(LexError::END),
            Some(_) => self.read_token()
        }
    }

    fn read_token(&mut self) -> Result<Token, LexError> {
        match self.peek() {
            Some(c) => match c {
                '('         => self.lpar(),
                ')'         => self.rpar(),
                ';'         => self.comment(),
                '"'         => self.string(),
                '0' ... '9' => self.integer(),
                'A' ... 'z' => self.ident(),
                _           => Err(LexError::INVALID(c))
            },
            None => Err(LexError::END)
        }
    }

    fn consume_whitespace(&mut self) -> () {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            } else {
                self.get();
            }
        }
    }

    fn lpar(&mut self) -> Result<Token, LexError> {
        self.get();
        Ok(Token::LPAR)
    }

    fn rpar(&mut self) -> Result<Token, LexError> {
        self.get();
        Ok(Token::RPAR)
    }

    // consume until end of line
    fn comment(&mut self) -> Result<Token, LexError> {
        let mut comment = String::new();
        while let Some(c) = self.get() {
            if c != '\n' {
                comment.push(c);
            } else {
                break;
            }
        }
        Ok(Token::COMMENT(comment.trim().to_string()))
    }

    fn string(&mut self) -> Result<Token, LexError> {
        let mut string = String::new();

        /* consume first quotation mark */
        self.get();

        while let Some(c) = self.get() {
            match c {
                /* if we get a '\', the next character, unconditionally take the next character */
                '\\' => match self.get() {
                    Some(next) => string.push(next),
                    None       => break
                },
                '\n' => break,
                '\"' => return Ok(Token::STRING(string)),
                _    => string.push(c)
            };
        }
        Err(LexError::UNTERMINATED(string))
    }

    fn integer(&mut self) -> Result<Token, LexError> {
        Ok(Token::LPAR)
    }

    fn ident(&mut self) -> Result<Token, LexError> {
        Ok(Token::RPAR)
    }
}

pub struct IOLexer<T: Read> {
    input: BufReader<T>,
    full:  bool,
    buf:   [u8; 1],
    line:  u32,
    chr:   u32
}

impl<T: Read> IOLexer<T> {
    pub fn new(input: T) -> IOLexer<T> {
        IOLexer { input: BufReader::new(input), buf: [0], full: false, line: 1, chr: 1 }
    }
}

impl<T: Read> Lexer for IOLexer<T> {
    fn get(&mut self) -> Option<char> {
        let c;
        if self.full {
            c = self.buf[0] as char;
            self.peek();
            self.count(c);
            Some(c)
        } else if let Some(c) = self.get() {
            Some(c)
        } else {
            None
        }
    }

    fn peek(&mut self) -> Option<char> {
        if self.full {
            Some(self.buf[0] as char)
        } else {
            match self.input.read(&mut self.buf) {
                Ok(0)  => None,
                Ok(_)  => {
                    self.full = true;
                    Some(self.buf[0] as char)
                }
                Err(_) => None
            }
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

impl<T: Read> Debug for IOLexer<T> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "IOLexer {{ line: { }, chr: { } }}", self.line, self.chr)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_lpar() {
        let mut lexer = StringLexer::new("(".to_string());
        let token = lexer.next().unwrap();
        assert_eq!(token, Token::LPAR);
    }

    #[test]
    fn read_rpar() {
        let mut lexer = StringLexer::new(")".to_string());
        let token = lexer.next().unwrap();
        assert_eq!(token, Token::RPAR);
    }

    #[test]
    fn read_string() {
        let mut lexer = StringLexer::new("\"\\\"Hello\\\", world!\\\n\"".to_string());
        let token = lexer.next().unwrap();
        assert_eq!(token, Token::STRING("\"Hello\", world!\n".to_string()));
    }

    #[test]
    fn read_comment() {
        let mut lexer = StringLexer::new("; this is some code that does some stuff".to_string());
        let token = lexer.next().unwrap();
        assert_eq!(token, Token::COMMENT("; this is some code that does some stuff".to_string()));
    }

    #[test]
    fn read_all() {
        let mut lexer = StringLexer::new("\
            ; hello, this is a comment \n\
            (\"this is a \\\"string\\\" with some escape chars\") \n\
            (   ) ; this is a comment after something on a line \n\
                            ( \"s p a c e\" ) ; space \n\
            ".to_string());

        let expected = vec![
            Token::COMMENT("; hello, this is a comment".to_string()),
            Token::LPAR, Token::STRING("this is a \"string\" with some escape chars".to_string()), Token::RPAR,
            Token::LPAR, Token::RPAR, Token::COMMENT("; this is a comment after something on a line".to_string()),
            Token::LPAR, Token::STRING("s p a c e".to_string()), Token::RPAR, Token::COMMENT("; space".to_string())
        ];

        let mut tokens = vec![];
        while let Ok(token) = lexer.next() {
            tokens.push(token)
        }
        assert_eq!(tokens, expected);
    }
}
