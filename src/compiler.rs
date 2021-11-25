use crate::chunk::{add_constant, Chunk, OP_ADD, OP_CONSTANT, OP_DIVIDE, OP_MULTIPLY, OP_NEGATE, OP_RETURN, OP_SUBTRACT, write_chunk};
use crate::scanner::{Scanner, Token, TokenType};
use crate::value::Value;
use crate::parser::{Parser, PREC_ASSIGNMENT, PREC_UNARY};

pub struct Compiler {
    parser: Parser,
    scanner: Scanner,
    pub compiling_chunk: Chunk,
}

impl Compiler {
    pub fn new(source: String, compiling_chunk: Chunk) -> Compiler {
        Compiler {
            parser: Parser::new(),
            scanner: Scanner::new(source),
            compiling_chunk,
        }
    }

    pub fn compile(&mut self) -> bool {
        self.advance();
        self.expression();
        self.consume(TokenType::EOF, "Expect end of expression.".to_string());
        self.end_compiler();
        !self.parser.had_error
    }

    fn emit_byte(&mut self, byte: usize) {
        write_chunk(&mut self.compiling_chunk, byte, self.parser.previous.line)
    }

    fn emit_bytes(&mut self, byte1: usize, byte2: usize) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn advance (&mut self) {
        self.parser.previous = self.parser.current;

        loop {
            self.parser.current = self.scanner.scan_token();
            if self.parser.current.token_type != TokenType::Error { break; }
            self.error_at_current(self.scanner.get_token_text(self.parser.current));
        }
    }

    fn error_at_current(&mut self, message: String) {
        self.error_at(self.parser.current, message)
    }

    fn error(&mut self, message: String ) {
        self.error_at(self.parser.previous, message);
    }

    fn error_at(&mut self, token: Token, message: String) {
        if self.parser.panic_mode {
            return;
        }
        self.parser.panic_mode = true;
        print!("[line {}] Error", token.line);

        if token.token_type == TokenType::EOF {
            print!(" at end.");
        }
        else if token.token_type == TokenType::Error {
            // Nothing.
        }
        else {
            print!(" at '{}, {}'", token.length, token.start);
        }

        println!(": {}", message);

        self.parser.had_error = true;
    }

    fn expression(&mut self) {
        self.parse_precedence(PREC_ASSIGNMENT);
    }

    fn parse_precedence(&mut self, precedence: i32) {
        self.advance();
        let prefix_rule = self.parser.get_rule(self.parser.previous.token_type).prefix;
        if prefix_rule as usize == Compiler::nil as usize {
            self.error("Expect expression.".to_string());
            return;
        }
        prefix_rule(self);

        let a = self.parser.get_rule(self.parser.previous.token_type);

        while precedence <= self.parser.get_rule(self.parser.current.token_type).precedence {
            self.advance();
            let infix_rule = self.parser.get_rule(self.parser.previous.token_type).infix;
            infix_rule(self);
        }
    }

    fn consume(&mut self, token_type: TokenType, message: String) {
        if self.parser.current.token_type == token_type {
            self.advance();
            return;
        }
        self.error_at_current(message);
    }

    fn end_compiler(&mut self) {
        self.emit_return();
    }

    pub fn binary(&mut self) {
        let operator_type = self.parser.previous.token_type;
        let rule = self.parser.get_rule(operator_type);
        self.parse_precedence(rule.precedence + 1);

        match operator_type {
            TokenType::Plus => { self.emit_byte(OP_ADD); },
            TokenType::Minus => { self.emit_byte(OP_SUBTRACT); },
            TokenType::Star => { self.emit_byte(OP_MULTIPLY); },
            TokenType::Slash =>  { self.emit_byte(OP_DIVIDE); },
            _ => { panic!("Unreachable binary operator.")},
        }
    }

    pub fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.".to_string());
    }

    fn emit_return(&mut self) {
        self.emit_byte(OP_RETURN);
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(OP_CONSTANT, constant);
    }

    fn make_constant(&mut self, value: Value) -> usize {
        let constant = add_constant(&mut self.compiling_chunk, value);
        if constant > 10 {
            return 0;
        }
        constant
    }

    pub fn number(&mut self) {
        let value: f64 = self.scanner.get_token_text(self.parser.previous).parse().unwrap();
        self.emit_constant(value);
    }

    pub fn unary(&mut self) {
        let operator_type = self.parser.previous.token_type;
        self.parse_precedence(PREC_UNARY);
        match operator_type {
            TokenType::Minus => { self.emit_byte(OP_NEGATE); }
            _ => {},
        }
    }

    pub fn nil(&mut self) {}
}
