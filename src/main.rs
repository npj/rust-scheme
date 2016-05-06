use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;

struct Lexer<T: Read> {
    input:     BufReader<T>,
    line:      u32,
    character: u32
}

impl<T: Read> Lexer<T> {
    fn new(input: T) -> Lexer<T> {
        Lexer { input: BufReader::new(input), line: 0, character: 0 }
    }

    fn get(&mut self) -> Option<u8> {
        let mut buf = [0; 1];
        match self.input.read(&mut buf) {
            Ok(0)  => None,
            Ok(_)  => Some(buf[0]),
            Err(_) => None
        }
    }
}

impl<T: Read> Debug for Lexer<T> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "Lexer {{ line: { }, character: { } }}", self.line, self.character)
    }
}

#[derive(Debug)]
struct Parser<T: Read> {
    lexer: Lexer<T>
}

impl<T: Read> Parser<T> {
    fn new(input: T) -> Parser<T> {
        Parser { lexer: Lexer::new(input) }
    }
}

fn main() {
    let mut parser = Parser::new(File::open("test.scm").ok().expect(""));

    for _ in 0..20 {
        match  parser.lexer.get() {
            Some(ch) => println!("{:?}", ch as char),
            None     => println!("None")
        }
    }
}
