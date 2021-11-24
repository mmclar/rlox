use crate::chunk::{Chunk, OP_RETURN, write_chunk};
use crate::scanner::{Scanner, Token, TokenType};

struct Parser {
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
}

pub struct Compiler {
    parser: Parser,
    scanner: Scanner,
    pub compiling_chunk: Chunk,
}

fn make_empty_token() -> Token {
    Token {
        token_type: TokenType::Empty,
        start: 0,
        length: 0,
        line: 0,
        // message: "A token about nothing".to_string(),
    }
}

impl Compiler {
    pub fn new(source: String, compiling_chunk: Chunk) -> Compiler {
        Compiler {
            parser: Parser {
                current: make_empty_token(),
                previous: make_empty_token(),
                had_error: false,
                panic_mode: false,
            },
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

    fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn emit_return(&mut self) {
        self.emit_byte(OP_RETURN);
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

    fn _error(&mut self, message: String ) {
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

    fn expression(&mut self) {}

    fn consume(&mut self, token_type: TokenType, message: String) {
        if self.parser.current.token_type == token_type {
            self.advance();
            return;
        }
        self.error_at_current(message);
    }
}
