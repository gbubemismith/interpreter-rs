use crate::{
    error::error,
    token::{LiteralTypes, Token, TokenType},
};

#[derive(Debug, PartialEq)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            String::new(),
            LiteralTypes::Nil,
            self.line,
        ));

        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen, LiteralTypes::Nil),
            ')' => self.add_token(TokenType::RightParen, LiteralTypes::Nil),
            '{' => self.add_token(TokenType::LeftBrace, LiteralTypes::Nil),
            '}' => self.add_token(TokenType::RightBrace, LiteralTypes::Nil),
            ',' => self.add_token(TokenType::Comma, LiteralTypes::Nil),
            '.' => self.add_token(TokenType::Dot, LiteralTypes::Nil),
            '-' => self.add_token(TokenType::Minus, LiteralTypes::Nil),
            '+' => self.add_token(TokenType::Plus, LiteralTypes::Nil),
            ';' => self.add_token(TokenType::SemiColon, LiteralTypes::Nil),
            '*' => self.add_token(TokenType::Star, LiteralTypes::Nil),

            '!' => {
                let is_equal = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(is_equal, LiteralTypes::Nil);
            }
            '=' => {
                let is_equal = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(is_equal, LiteralTypes::Nil);
            }
            '<' => {
                let is_equal = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(is_equal, LiteralTypes::Nil);
            }
            '>' => {
                let is_equal = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(is_equal, LiteralTypes::Nil);
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_char('*') {
                    self.scan_block_comment();
                } else {
                    self.add_token(TokenType::Slash, LiteralTypes::Nil);
                }
            }
            // Ignore whitespace
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),

            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    error(self.line as u32, "Unexpected character.");
                }
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, ttype: TokenType, literal: LiteralTypes) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(ttype, text, literal, self.line))
    }

    fn advance(&mut self) -> char {
        // warning: intended for only ASCII characters
        let c = self.source.as_bytes()[self.current] as char;
        self.current += 1;
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.as_bytes()[self.current] as char != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self.source.as_bytes()[self.current] as char;
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        return self.source.as_bytes()[self.current + 1] as char;
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            // log error
            error(self.line as u32, "Unterminated string.");
            return;
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String, LiteralTypes::String(value));
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let value: f64 = self.source[self.start..self.current].parse().unwrap();
        self.add_token(TokenType::Number, LiteralTypes::Number(value));
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = map_keyword(text).unwrap_or(TokenType::Identifier);
        self.add_token(token_type, LiteralTypes::Nil);
    }

    fn scan_block_comment(&mut self) {
        // /* */
        while !self.is_at_end() {
            match self.peek() {
                '\n' => self.line += 1,
                '*' => {
                    if self.peek() == '/' {
                        self.advance(); //consume *
                        self.advance(); //consume /
                        return;
                    }
                }
                _ => {}
            }

            self.advance();
        }
    }

    fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn is_alpha(&self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        return self.is_alpha(c) || self.is_digit(c);
    }
}

fn map_keyword(keyword: &str) -> Option<TokenType> {
    match keyword {
        "and" => Some(TokenType::And),
        "class" => Some(TokenType::Class),
        "else" => Some(TokenType::Else),
        "false" => Some(TokenType::False),
        "for" => Some(TokenType::For),
        "fun" => Some(TokenType::Fun),
        "if" => Some(TokenType::If),
        "nil" => Some(TokenType::Nil),
        "or" => Some(TokenType::Or),
        "print" => Some(TokenType::Print),
        "return" => Some(TokenType::Return),
        "super" => Some(TokenType::Super),
        "this" => Some(TokenType::This),
        "true" => Some(TokenType::True),
        "var" => Some(TokenType::Var),
        "while" => Some(TokenType::While),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Utility function for making tokens
    fn make_token(ttype: TokenType, lexeme: &str, literal: LiteralTypes, line: usize) -> Token {
        Token {
            ttype,
            lexeme: lexeme.to_string(),
            literal,
            line,
        }
    }

    /// Scanner initialization tests
    mod scanner_init {
        use super::*;

        #[test]
        fn constructs_scanner() {
            let source = "==".to_string();
            let scanner = Scanner::new(source.clone());

            let expected = Scanner {
                source,
                tokens: Vec::new(),
                start: 0,
                current: 0,
                line: 1,
            };

            assert_eq!(scanner, expected);
        }
    }

    mod tokenization {
        use super::*;

        #[test]
        fn tokenize_muliple_lines() {
            let source = r#"var name = "Gbubemi";
            var counter = 1;"#
                .to_string();
            let mut scanner = Scanner::new(source);

            let expected_vec = vec![
                make_token(TokenType::Var, "var", LiteralTypes::Nil, 1),
                make_token(TokenType::Identifier, "name", LiteralTypes::Nil, 1),
                make_token(TokenType::Equal, "=", LiteralTypes::Nil, 1),
                make_token(
                    TokenType::String,
                    r#""Gbubemi""#,
                    LiteralTypes::String(String::from("Gbubemi")),
                    1,
                ),
                make_token(TokenType::SemiColon, ";", LiteralTypes::Nil, 1),
                make_token(TokenType::Var, "var", LiteralTypes::Nil, 2),
                make_token(TokenType::Identifier, "counter", LiteralTypes::Nil, 2),
                make_token(TokenType::Equal, "=", LiteralTypes::Nil, 2),
                make_token(TokenType::Number, "1", LiteralTypes::Number(1.0), 2),
                make_token(TokenType::SemiColon, ";", LiteralTypes::Nil, 2),
                make_token(TokenType::EOF, "", LiteralTypes::Nil, 2),
            ];

            let tokens = scanner.scan_tokens();

            assert_eq!(expected_vec, tokens);
        }

        #[test]
        fn tokenize_single_chars() {
            let source = "var add = 1 + 1;".to_string();
            let mut scanner = Scanner::new(source);

            let expected_vec = vec![
                make_token(TokenType::Var, "var", LiteralTypes::Nil, 1),
                make_token(TokenType::Identifier, "add", LiteralTypes::Nil, 1),
                make_token(TokenType::Equal, "=", LiteralTypes::Nil, 1),
                make_token(TokenType::Number, "1", LiteralTypes::Number(1.0), 1),
                make_token(TokenType::Plus, "+", LiteralTypes::Nil, 1),
                make_token(TokenType::Number, "1", LiteralTypes::Number(1.0), 1),
                make_token(TokenType::SemiColon, ";", LiteralTypes::Nil, 1),
                make_token(TokenType::EOF, "", LiteralTypes::Nil, 1),
            ];

            let tokens = scanner.scan_tokens();

            assert_eq!(expected_vec, tokens);
        }

        #[test]
        fn scanner_skips_c_styled_block_comments() {
            let source = "/**  this is a sample block comment */".to_string();
            let mut scanner = Scanner::new(source);

            let expected_vec = vec![make_token(TokenType::EOF, "", LiteralTypes::Nil, 1)];

            let tokens = scanner.scan_tokens();

            assert_eq!(expected_vec, tokens);
        }
    }
}
