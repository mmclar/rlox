use crate::scanner::{Scanner, TokenType};
use crate::scanner::TokenType::TokenAnd;

pub fn compile(source: String) {
    let mut scanner = Scanner::new(source);
    let mut line = -1;
    while true {
        let token = scanner.scan_token();
        if token.line != line {
            println!("{} ", token.line);
            line = token.line;
        }
        else {
            println!("  | ")
        }
        println!("{:?} {} {}", token.token_type, token.length, token.start);
        if token.token_type == TokenType::TokenEOF {
            break;
        }
    }
}



