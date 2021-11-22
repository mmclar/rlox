use crate::scanner::TokenType::{TokenBang, TokenEOF, TokenError};

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenEOF);
        }
        self.error_token("Unexpected character".to_string())
    }

    fn  is_at_end(&mut self) -> bool {
        return self.current < self.source.len()
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        Token {
            token_type,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
            message: "".to_string(),
        }
    }

    fn error_token(&mut self, message: String) -> Token {
        Token {
            token_type: TokenError,
            start: 0,
            length: 0,
            line: self.line,
            message: message,
        }
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub length: usize,
    pub line: i32,
    pub message: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    TokenLeftParen, TokenRightParen,
    TokenLeftBrace, TokenRightBrace,
    TokenComma, TokenDot, TokenMinus, TokenPlus,
    TokenSemicolon, TokenSlash, TokenStar,
    // One or two character tokens.
    TokenBang, TokenBangEqual,
    TokenEqual, TokenEqualEqual,
    TokenGreater, TokenGreaterEqual,
    TokenLess, TokenLessEqual,
    // Literals.
    TokenIdentifier, TokenString, TokenNumber,
    // Keywords.
    TokenAnd, TokenClass, TokenElse, TokenFalse,
    TokenFor, TokenFun, TokenIf, TokenNil, TokenOr,
    TokenPrint, TokenReturn, TokenSuper, TokenThis,
    TokenTrue, TokenVar, TokenWhile,
    // Util
    TokenError, TokenEOF
}
