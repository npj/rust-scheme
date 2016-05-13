#[derive(PartialEq, Debug)]
pub enum Token {
    LPAR(u32, u32),
    RPAR(u32, u32),
    COMMENT(String, u32, u32),
    STRING(String, u32, u32),
    INTEGER(String, u32, u32),
    FLOAT(String, u32, u32),
    IDENT(String, u32, u32)
}

impl Token {
    fn number(string: String, is_float: bool, line: u32, chr: u32) -> Token {
        if is_float {
            Token::FLOAT(string, line, chr)
        } else {
            Token::INTEGER(string, line, chr)
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum LexError {
    INVALID(char, u32, u32),
    UNTERMINATED(String, u32, u32),
    IDENT(String, u32, u32),
    INTEGER(String, u32, u32),
    FLOAT(String, u32, u32),
    END(u32, u32)
}

impl LexError {
    fn number(string: String, is_float: bool, line: u32, chr: u32) -> LexError {
        if is_float {
            LexError::FLOAT(string, line, chr)
        } else {
            LexError::INTEGER(string, line, chr)
        }
    }
}

pub trait Lexer {
    fn get(&mut self) -> Option<char>;
    fn peek(&self) -> Option<char>;
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
            None    => Err(LexError::END(self.line(), self.chr())),
            Some(_) => self.read_token()
        }
    }

    fn read_token(&mut self) -> Result<Token, LexError> {
        match self.peek() {
            Some(c) => match c {
                '('                     => self.lpar(),
                ')'                     => self.rpar(),
                ';'                     => self.comment(),
                '"'                     => self.string(),
                '0' ... '9' | '-' | '.' => self.number(),
                'A' ... 'z'             => self.ident(),
                _                       => Err(LexError::INVALID(c, self.line(), self.chr()))
            },
            None => Err(LexError::END(self.line(), self.chr()))
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
        let line = self.line();
        let chr  = self.chr();
        self.get();
        Ok(Token::LPAR(line, chr))
    }

    fn rpar(&mut self) -> Result<Token, LexError> {
        let line = self.line();
        let chr  = self.chr();
        self.get();
        Ok(Token::RPAR(line, chr))
    }

    // consume until end of line
    fn comment(&mut self) -> Result<Token, LexError> {
        let line        = self.line();
        let chr         = self.chr();
        let mut comment = String::new();
        while let Some(c) = self.get() {
            if c != '\n' {
                comment.push(c);
            } else {
                break;
            }
        }
        Ok(Token::COMMENT(comment.trim().to_string(), line, chr))
    }

    fn string(&mut self) -> Result<Token, LexError> {
        let mut string = String::new();
        let start_line = self.line();
        let start_chr  = self.chr();

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
                '\"' => return Ok(Token::STRING(string, start_line, start_chr)),
                _    => string.push(c)
            };
        }
        Err(LexError::UNTERMINATED(string, start_line, start_chr))
    }

    fn number(&mut self) -> Result<Token, LexError> {
        let mut number = String::new();
        let start_line = self.line();
        let start_chr  = self.chr();
        let mut float  = false;

        if let Some('-') = self.peek() {
            number.push('-');
            self.get();
        }

        while let Some(c) = self.get() {
            number.push(c);
            match c {
                '0' ... '9' => (),
                '.' => if float {
                    return Err(LexError::FLOAT(number, start_line, start_chr))
                } else {
                    float = true
                },
                _ => if c.is_whitespace() {
                    break
                } else {
                    return Err(LexError::number(number, float, start_line, start_chr))
                }
            }
        }

        Ok(Token::number(number.trim().to_string(), float, start_line, start_chr))
    }

    fn ident(&mut self) -> Result<Token, LexError> {
        let invalid    = vec!['[', ']', '{', '}', '(', ')', '|', '\\', '/', '\'', '\"', '#', ','];
        let start_line = self.line();
        let start_chr  = self.chr();
        let mut ident  = String::new();

        while let Some(c) = self.get() {
            if invalid.contains(&c) {
                return Err(LexError::IDENT(ident, start_line, start_chr))
            } else if c.is_whitespace() {
                break
            } else {
                ident.push(c)
            }
        }

        Ok(Token::IDENT(ident, start_line, start_chr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::StringLexer;

    #[test]
    fn read_lpar() {
        let mut lexer = StringLexer::new("(".to_string());
        let token = lexer.next().ok().unwrap();
        assert_eq!(token, Token::LPAR(1, 1));
    }

    #[test]
    fn read_rpar() {
        let mut lexer = StringLexer::new(")".to_string());
        let token = lexer.next().ok().unwrap();
        assert_eq!(token, Token::RPAR(1, 1));
    }

    #[test]
    fn read_string() {
        let mut lexer = StringLexer::new("\"\\\"Hello\\\", world!\\\n\"".to_string());
        let token = lexer.next().ok().unwrap();
        assert_eq!(token, Token::STRING("\"Hello\", world!\n".to_string(), 1, 1));
    }

    #[test]
    fn read_comment() {
        let mut lexer = StringLexer::new("; this is some code that does some stuff".to_string());
        let token = lexer.next().ok().unwrap();
        assert_eq!(token, Token::COMMENT("; this is some code that does some stuff".to_string(), 1, 1));
    }

    #[test]
    fn read_ident() {
        let mut lexer = StringLexer::new("an-!@$%^&*-+=~?.ident-can-have-all-these-chars".to_string());
        let token = lexer.next().ok().unwrap();
        assert_eq!(token, Token::IDENT("an-!@$%^&*-+=~?.ident-can-have-all-these-chars".to_string(), 1, 1));
    }

    #[test]
    fn read_integer() {
        let mut lexer = StringLexer::new("12345".to_string());
        let token = lexer.next().ok().unwrap();
        assert_eq!(token, Token::INTEGER("12345".to_string(), 1, 1));
    }

    #[test]
    fn read_negative_integer() {
        let mut lexer = StringLexer::new("-12345".to_string());
        let token = lexer.next().ok().unwrap();
        assert_eq!(token, Token::INTEGER("-12345".to_string(), 1, 1));
    }

    #[test]
    fn read_invalid_integer() {
        let mut lexer = StringLexer::new("12f345".to_string());
        let token = lexer.next().err().unwrap();
        assert_eq!(token, LexError::INTEGER("12f".to_string(), 1, 1));
    }

    #[test]
    fn read_float_dot() {
        let mut lexer = StringLexer::new("12345.".to_string());
        let token = lexer.next().ok().unwrap();
        assert_eq!(token, Token::FLOAT("12345.".to_string(), 1, 1));
    }

    #[test]
    fn read_float_dot_zero() {
        let mut lexer = StringLexer::new("12345.0".to_string());
        let token = lexer.next().ok().unwrap();
        assert_eq!(token, Token::FLOAT("12345.0".to_string(), 1, 1));
    }

    #[test]
    fn read_float_dot_digits() {
        let mut lexer = StringLexer::new(".12345".to_string());
        let token = lexer.next().ok().unwrap();
        assert_eq!(token, Token::FLOAT(".12345".to_string(), 1, 1));
    }

    #[test]
    fn read_float_digits_dot_digits() {
        let mut lexer = StringLexer::new("12345.12345".to_string());
        let token = lexer.next().ok().unwrap();
        assert_eq!(token, Token::FLOAT("12345.12345".to_string(), 1, 1));
    }

    #[test]
    fn read_float_dot_digits_trailing_zero() {
        let mut lexer = StringLexer::new("12345.123450".to_string());
        let token = lexer.next().ok().unwrap();
        assert_eq!(token, Token::FLOAT("12345.123450".to_string(), 1, 1));
    }

    #[test]
    fn read_float_negative_dot_digits() {
        let mut lexer = StringLexer::new("-.12345".to_string());
        let token = lexer.next().ok().unwrap();
        assert_eq!(token, Token::FLOAT("-.12345".to_string(), 1, 1));
    }

    #[test]
    fn read_negative_float_dot_digits() {
        let mut lexer = StringLexer::new("-12345.12345".to_string());
        let token = lexer.next().ok().unwrap();
        assert_eq!(token, Token::FLOAT("-12345.12345".to_string(), 1, 1));
    }

    #[test]
    fn read_invalid_float_whole() {
        let mut lexer = StringLexer::new("12f345.12345".to_string());
        let token = lexer.next().err().unwrap();
        assert_eq!(token, LexError::INTEGER("12f".to_string(), 1, 1));
    }

    #[test]
    fn read_invalid_float_fractional() {
        let mut lexer = StringLexer::new("12345.12f345".to_string());
        let token = lexer.next().err().unwrap();
        assert_eq!(token, LexError::FLOAT("12345.12f".to_string(), 1, 1));
    }

    #[test]
    fn read_all() {
        let mut lexer = StringLexer::new("\
            ; hello, this is a comment \n\
            (\"this is a \\\"string\\\" with some escape chars\") \n\
            (   ) ; this is a comment after something on a line \n\
            (               ( \"s p a c e\" ) ; space \n\
            12345 is-a-number so_is -78.910 \n\
            ".to_string());

        let expected = vec![
            Token::COMMENT("; hello, this is a comment".to_string(), 1, 1),
            Token::LPAR(2, 1),
            Token::STRING("this is a \"string\" with some escape chars".to_string(), 2, 2),
            Token::RPAR(2, 47),
            Token::LPAR(3, 1),
            Token::RPAR(3, 5),
            Token::COMMENT("; this is a comment after something on a line".to_string(), 3, 7),
            Token::LPAR(4, 1),
            Token::LPAR(4, 17),
            Token::STRING("s p a c e".to_string(), 4, 19),
            Token::RPAR(4, 31),
            Token::COMMENT("; space".to_string(), 4, 33),
            Token::INTEGER("12345".to_string(), 5, 1),
            Token::IDENT("is-a-number".to_string(), 5, 7),
            Token::IDENT("so_is".to_string(), 5, 19),
            Token::FLOAT("-78.910".to_string(), 5, 25)
        ];

        let mut tokens = vec![];
        while let Ok(token) = lexer.next() {
            tokens.push(token)
        }

        assert_eq!(tokens, expected)
    }

    #[test]
    fn error_invalid() {
        let mut lexer = StringLexer::new("(    # )".to_string());
        lexer.next().ok().unwrap();
        assert_eq!(lexer.next().err().unwrap(), LexError::INVALID('#', 1, 6));
    }

    #[test]
    fn error_end_empty() {
        let mut lexer = StringLexer::new("".to_string());
        assert_eq!(lexer.next().err().unwrap(), LexError::END(1, 1));
    }

    #[test]
    fn error_end_nonempty() {
        let mut lexer = StringLexer::new(")".to_string());
        lexer.next().ok().unwrap();
        assert_eq!(lexer.next().err().unwrap(), LexError::END(1, 2));
    }

    #[test]
    fn error_unterminated() {
        let mut lexer = StringLexer::new("\"This is an unterminated string ()".to_string());
        assert_eq!(lexer.next().err().unwrap(), LexError::UNTERMINATED("This is an unterminated string ()".to_string(), 1, 1));
    }

    #[test]
    fn error_ident() {
        let invalid = vec!['[', ']', '{', '}', '(', ')', '|', '\\', '/', '\'', '\"', '#', ','];
        let ident_pre = "an-ident-cannot-have-";
        let ident_suf = "-as-a-char";

        for i in invalid {
            let mut ident = String::new();
            ident = ident + &ident_pre;
            ident.push(i);
            ident = ident + &ident_suf;

            let mut lexer = StringLexer::new(ident);
            assert_eq!(lexer.next().err().unwrap(), LexError::IDENT(ident_pre.to_string(), 1, 1));
        }
    }

    #[test]
    fn error_unterminated_multiline() {
        let mut lexer = StringLexer::new("\n \n \"This is an \\\n unterminated string ()".to_string());
        assert_eq!(lexer.next().err().unwrap(), LexError::UNTERMINATED("This is an \n unterminated string ()".to_string(), 3, 2));
    }
}
