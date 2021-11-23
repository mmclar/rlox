use crate::value::Value;

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let source = source.chars().collect();
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.is_at_end() { return self.make_token(TokenType::EOF); }
        self.start = self.current;

        let c = self.advance().clone();

        if self.is_digit(&c) { return self.number(); }

        if self.match_next(&'=') {
            match c {
                '!' => return self.make_token(TokenType::BangEqual),
                '=' => return self.make_token(TokenType::EqualEqual),
                '<' => return self.make_token(TokenType::LessEqual),
                '>' => return self.make_token(TokenType::GreaterEqual),
                _ => {},
            }
        }
        match c {
            '(' => return self.make_token(TokenType::LeftParen),
            ')' => return self.make_token(TokenType::RightParen),
            '{' => return self.make_token(TokenType::LeftBrace),
            '}' => return self.make_token(TokenType::RightBrace),
            ';' => return self.make_token(TokenType::Semicolon),
            ',' => return self.make_token(TokenType::Comma),
            '.' => return self.make_token(TokenType::Dot),
            '-' => return self.make_token(TokenType::Minus),
            '+' => return self.make_token(TokenType::Plus),
            '/' => return self.make_token(TokenType::Slash),
            '*' => return self.make_token(TokenType::Star),
            '!' => return self.make_token(TokenType::Bang),
            '=' => return self.make_token(TokenType::Equal),
            '<' => return self.make_token(TokenType::Less),
            '>' => return self.make_token(TokenType::Greater),
            '"' => return self.string(),
            _ => {},
        }

        self.error_token("Unexpected character".to_string())
    }

    fn  is_at_end(&self) -> bool {
        return self.current >= self.source.len()
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
            token_type: TokenType::Error,
            start: 0,
            length: 0,
            line: self.line,
            message: message,
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => { self.advance(); }
                '\n' => { self.line = self.line + 1; self.advance(); }
                '/' => {
                    let next = self.peek_next();
                    if next == &'/' {
                        loop {
                            if self.is_at_end() { break; }
                            let peeked = &self.peek().clone();
                            if peeked == &'\n' { break; }
                            self.advance();
                        }
                    }
                }
                _ => break
            }
        }
    }

    fn is_digit(&self, c: &char) -> bool {
        return match c {
            '0'..='9' => true,
            _ => false
        }
    }

    fn number(&mut self) -> Token {
        while self.is_digit(self.peek()) { self.advance(); }

        if self.peek() == &'.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) { self.advance(); }
        }

        self.make_token(TokenType::Number)
    }

    fn string(&mut self) -> Token {
        while self.peek() != &'"' && !self.is_at_end() {
            if self.peek() == &'\n' {
                self.line = self.line + 1;
            }
            self.advance();
        }

        if self.is_at_end() {return self.error_token("Unterminated string".to_string())}
        self.advance();
        self.make_token(TokenType::String)
    }

    fn advance(&mut self) -> &char {
        self.current = self.current + 1;
        self.get_char_at_idx(self.current - 1)
    }

    fn peek(&self) -> &char {
        self.get_char_at_idx(self.current)
    }

    fn peek_next(&self) -> &char {
        self.get_char_at_idx(self.current + 1)
    }

    fn get_char_at_idx(&self, idx: usize) -> &char {
        let ch: Option<&char> = self.source.get(idx);
        match ch {
            Some(ch) => ch,
            _ => &'\0',
        }
    }

    fn match_next(&mut self, expected: &char) -> bool {
        if self.is_at_end() { return false };
        if self.peek() == expected {
            self.current = self.current + 1;
            return true;
        }
        false
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
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,
    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    // Literals.
    Identifier, String, Number,
    // Keywords.
    And, Class, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,
    // Util
    Error, EOF
}
