use core::panic;

use super::super::lexer;

pub struct Scanner {
    pub token_stream: Vec<lexer::Token>,
    pub source_chars: Vec<char>,
    pub offset: usize,
    pub char_position: lexer::CharPosition,
}

impl Scanner {
    /// Deserializes inputted toml file to tokenstream <br>
    /// the core logic is executed in a loop
    /// ```rust
    /// while !scanner.is_at_end() {
    ///     let start = scanner.current;
    ///     scanner.scan_token(start);
    /// }
    /// ```
    pub fn deserialize(toml: &str) -> Vec<lexer::Token> {
        let mut scanner = Scanner {
            source_chars: toml.chars().collect(),
            token_stream: Vec::new(),
            offset: 0,
            char_position: lexer::CharPosition { line: 1, column: 2 },
        };

        while !scanner.is_at_end() {
            let start = scanner.offset;
            scanner.scan_token(start);
        }

        scanner.token_stream
    }

    /// Scans characters in `Scanner::chars` and lexes into tokens.
    ///
    /// ```rust
    /// '#' = Comment
    /// '=' = EqualsSign
    /// '[' = BracketOpened
    /// ']' = BracketCloses
    /// '[[' = DoubleBracketsOpened
    /// ']]' = DoublebracketsCloses
    /// ```
    ///
    /// Strings get handled in strings.rs module.
    /// ```rust
    /// "string example" = StringBasic
    /// ```
    /// Likewise, identifiers get handled in identifiers.rs module.
    /// ```rust
    /// identifier = Identifier
    /// ```
    ///
    /// <h2>Panics</h2>
    ///
    fn scan_token(&mut self, start: usize) {
        let c = self.advance();
        let start_position = self.char_position;

        match c {
            '#' => {
                while self.peek() != Some('\n') && !self.is_at_end() {
                    self.advance();
                }
                self.add_token(lexer::TokenType::Comment, None, start, start_position);
            }

            '[' => {
                let token = if self.peek() == Some('[') {
                    self.advance();
                    lexer::TokenType::DoubleBracketsOpened
                } else {
                    lexer::TokenType::BracketOpened
                };
                self.add_token(token, None, start, start_position);
            }

            ']' => {
                let token = if self.peek() == Some(']') {
                    self.advance();
                    lexer::TokenType::DoubleBracketsClosed
                } else {
                    lexer::TokenType::BracketClosed
                };
                self.add_token(token, None, start, start_position);
            }

            '=' => self.add_token(lexer::TokenType::EqualsSign, None, start, start_position),

            ',' | ' ' | '\t' | '\n' => {}

            '"' => self.scan_string(start),

            c if c.is_ascii_alphabetic() || c == '_' => self.scan_identifier(start),

            _ => {
                panic!(
                    "Unexpected character '{}' at line {}, column {} in config",
                    c, self.char_position.line, self.char_position.column,
                );
            }
        }
    }

    /// Advances to the next character in the chars Vec. <br>
    /// Increments the line position when encountering `\n`
    fn advance(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        let c = self.source_chars[self.offset];
        self.offset += 1;

        if c == '\n' {
            self.char_position.line += 1;
            self.char_position.column_start();
        } else {
            self.char_position.column += 1;
        }

        c
    }

    /// "peeks" at the next character in chars Vec
    fn peek(&self) -> Option<char> {
        self.source_chars.get(self.offset).copied()
    }

    /// Checks if scanner has reached end of `chars` Vec
    fn is_at_end(&self) -> bool {
        self.offset >= self.source_chars.len()
    }

    /// Adds token to tokenstream
    pub fn add_token(
        &mut self,
        token_type: lexer::TokenType,
        literal: Option<lexer::TomlValue>,
        start: usize,
        line_position: lexer::CharPosition,
    ) {
        let lexeme: String = self.source_chars[start..self.offset].iter().collect();
        self.token_stream.push(lexer::Token {
            token_type,
            literal,
            lexeme,
            line_position,
        });
    }
}
