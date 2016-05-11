extern crate scheme;

use scheme::lexer::Lexer;
use scheme::lexer::IOLexer;
use scheme::lexer::StringLexer;
use scheme::parser::Parser;

use std::fs::File;

fn main() {
    let mut file_parser = Parser::new(IOLexer::new(File::open("test.scm").ok().expect("")));
    let mut str_parser  = Parser::new(StringLexer::new("()\n".to_string()));

    for _ in 0..20 {
        match file_parser.get_lexer().get() {
            Some(ch) => println!("{:?}", ch),
            None     => println!("None")
        }

        match str_parser.get_lexer().get() {
            Some(ch) => println!("{:?}", ch),
            None     => println!("None")
        }
    }
}
