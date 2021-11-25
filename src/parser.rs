use crate::compiler::Compiler;
use crate::scanner::{make_empty_token, Token, TokenType};
use std::collections::HashMap;
use crate::parser::Precedence::{And, Assignment, Or};

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
            (TokenType::LeftParen, rule(Compiler::grouping, Compiler::nil, Precedence::None)),
            (TokenType::RightParen, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::LeftBrace, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::RightBrace, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::Comma, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::Dot, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::Minus, rule(Compiler::unary,Compiler::binary, Precedence::Term)),
            (TokenType::Plus, rule(Compiler::nil, Compiler::binary, Precedence::Term)),
            (TokenType::Semicolon, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::Slash, rule(Compiler::nil,Compiler::binary, Precedence::Factor)),
            (TokenType::Star, rule(Compiler::nil, Compiler::binary, Precedence::Factor)),
            (TokenType::Bang, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::BangEqual, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::Equal, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::EqualEqual, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::Greater, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::GreaterEqual, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::Less, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::LessEqual, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::Identifier, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::String, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::Number, rule(Compiler::number, Compiler::nil, Precedence::None)),
            (TokenType::And, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::Class, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::Else, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::False, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::For, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::Fun, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::If, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::Nil, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::Or, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::Print, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::Return, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::Super, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::This, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::True, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::Var, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::While, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::Error, rule(Compiler::nil, Compiler::nil, Precedence::None)),
            (TokenType::EOF, rule(Compiler::nil, Compiler::nil, Precedence::None)),

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
        self.rules.get(&token_type).unwrap()
    }
}


#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum Precedence {
    None = 1,
    Assignment = 2,
    Or = 3,
    And = 4,
    Equality = 5,
    Comparison = 6,
    Term = 7,
    Factor = 8,
    Unary = 9,
    Call = 10,
    Primary = 11,
}

pub fn next_precedence(precedence: &Precedence) -> Precedence {
    match precedence {
        Precedence::None => Precedence::Assignment,
        Precedence::Assignment => Precedence::Or,
        Precedence::Or => Precedence::And,
        Precedence::And => Precedence::Equality,
        Precedence::Equality => Precedence::Comparison,
        Precedence::Comparison => Precedence::Term,
        Precedence::Term => Precedence::Factor,
        Precedence::Factor => Precedence::Unary,
        Precedence::Unary => Precedence::Call,
        Precedence::Call => Precedence::Primary,
        Precedence::Primary => { panic!("No higher precedence.")},
    }
}

pub type ParseFn = fn(&mut Compiler);

pub struct ParseRule {
    pub prefix: ParseFn,
    pub infix: ParseFn,
    pub precedence: Precedence,
}

fn rule(prefix: ParseFn, infix: ParseFn, precedence: Precedence) -> ParseRule {
    ParseRule {
        prefix, infix, precedence
    }
}