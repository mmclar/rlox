use crate::compiler::Compiler;
use crate::scanner::{make_empty_token, Token, TokenType};
use std::collections::HashMap;

pub struct Parser {
    pub current: Token,
    pub previous: Token,
    pub had_error: bool,
    pub panic_mode: bool,
    pub rules: HashMap<TokenType, ParseRule>,
}

impl Parser {
    pub fn new() -> Parser {
        let rules: HashMap<TokenType, ParseRule>= HashMap::from([
            (TokenType::LeftParen, rule(Compiler::grouping, Compiler::nil, PREC_NONE)),
            (TokenType::RightParen, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::LeftBrace, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::RightBrace, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::Comma, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::Dot, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::Minus, rule(Compiler::unary,Compiler::binary, PREC_TERM)),
            (TokenType::Plus, rule(Compiler::nil, Compiler::binary, PREC_TERM)),
            (TokenType::Semicolon, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::Slash, rule(Compiler::nil,Compiler::binary, PREC_FACTOR)),
            (TokenType::Star, rule(Compiler::nil, Compiler::binary, PREC_FACTOR)),
            (TokenType::Bang, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::BangEqual, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::Equal, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::EqualEqual, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::Greater, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::GreaterEqual, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::Less, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::LessEqual, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::Identifier, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::String, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::Number, rule(Compiler::number, Compiler::nil, PREC_NONE)),
            (TokenType::And, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::Class, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::Else, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::False, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::For, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::Fun, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::If, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::Nil, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::Or, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::Print, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::Return, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::Super, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::This, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::True, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::Var, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::While, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::Error, rule(Compiler::nil, Compiler::nil, PREC_NONE)),
            (TokenType::EOF, rule(Compiler::nil, Compiler::nil, PREC_NONE)),

        ]);
        Parser {
            current: make_empty_token(),
            previous: make_empty_token(),
            had_error: false,
            panic_mode: false,
            rules,
        }
    }

    pub fn get_rule(&self, token_type: TokenType) -> &ParseRule {
        let r = self.rules.get(&token_type).unwrap();
        println!("r");
        r
    }
}

pub static PREC_NONE: i32 = 1;
pub static PREC_ASSIGNMENT: i32 = 2;
pub static PREC_OR: i32 = 3;
pub static PREC_AND: i32 = 4;
pub static PREC_EQUALITY: i32 = 5;
pub static PREC_COMPARISON: i32 = 6;
pub static PREC_TERM: i32 = 7;
pub static PREC_FACTOR: i32 = 8;
pub static PREC_UNARY: i32 = 9;
pub static PREC_CALL: i32 = 10;
pub static PREC_PRIMARY: i32 = 11;

pub type ParseFn = fn(&mut Compiler);

pub struct ParseRule {
    pub prefix: ParseFn,
    pub infix: ParseFn,
    pub precedence: i32,
}

fn rule(prefix: ParseFn, infix: ParseFn, precedence: i32) -> ParseRule {
    ParseRule {
        prefix, infix, precedence
    }
}