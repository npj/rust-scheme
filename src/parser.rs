use lexer::Lexer;

#[derive(Debug)]
pub struct Parser<T: Lexer> {
    lexer: T
}

impl<T: Lexer> Parser<T> {
    pub fn new(lexer: T) -> Parser<T> {
        Parser { lexer: lexer }
    }

    pub fn get_lexer(&mut self) -> &mut T {
        &mut self.lexer
    }
}

