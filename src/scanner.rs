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

        if self.is_alpha(&c) { return self.identifier(); }

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

    fn is_alpha(&self, c: &char) -> bool {
        return match c {
            'a'..='z' | 'A'..='Z' | '_' => true,
            _ => false,
        }
    }

    fn identifier(&mut self) -> Token {
        while self.is_alpha(self.peek()) || self.is_digit(self.peek()) {
            self.advance();
        }
        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenType {
        match self.get_char_at_idx(self.start) {
            'a' => self.check_keyword(1, "nd".to_string(), TokenType::And),
            'c' => self.check_keyword(1, "lass".to_string(), TokenType::Class),
            'e' => self.check_keyword(1, "lse".to_string(), TokenType::Else),
            'i' => self.check_keyword(1, "f".to_string(), TokenType::If),
            'n' => self.check_keyword(1, "il".to_string(), TokenType::Nil),
            'o' => self.check_keyword(1, "r".to_string(), TokenType::Or),
            'p' => self.check_keyword(1, "rint".to_string(), TokenType::Print),
            'r' => self.check_keyword(1, "eturn".to_string(), TokenType::Return),
            's' => self.check_keyword(1, "uper".to_string(), TokenType::Super),
            'v' => self.check_keyword(1, "ar".to_string(), TokenType::Var),
            'w' => self.check_keyword(1, "hile".to_string(), TokenType::While),
            'f' => match self.get_char_at_idx(self.start + 1) {
                'a' => self.check_keyword(2, "lse".to_string(), TokenType::False),
                'o' => self.check_keyword(2, "r".to_string(), TokenType::For),
                'u' => self.check_keyword(2, "n".to_string(), TokenType::Fun),
                _ => TokenType::Identifier,
            },
            't' => match self.get_char_at_idx(self.start + 1) {
                'h' => self.check_keyword(2, "is".to_string(), TokenType::This),
                'r' => self.check_keyword(2, "ue".to_string(), TokenType::True),
                _ => TokenType::Identifier,
            },
            _ => TokenType::Identifier,
        }
    }

    fn check_keyword(&self, start: usize, rest: String, token_type: TokenType ) -> TokenType {
        let length = self.current - self.start - start;
        let strings_match = cmp(&self.source, self.start + start, length, rest);
        if strings_match {
            return token_type;
        }
        TokenType::Identifier
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

fn cmp(a: &Vec<char>, a_start: usize, length: usize, b: String) -> bool {
    if b.len() != length { return false; }
    let b_chars: Vec<char> = b.chars().collect();
    for i in 0..length {
        if a.get(i + a_start) != b_chars.get(i) {
            return false;
        }
    }
    true
}