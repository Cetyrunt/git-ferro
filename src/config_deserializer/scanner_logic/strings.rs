use super::super::lexer;
use super::scanner::Scanner;

impl Scanner {
    /// Lexes a quoted string literal from the current position in `source_chars`.
    pub fn scan_string(&mut self, start: usize) {
        let mut literal_value = String::new();

        while let Some(c) = self.source_chars.get(self.offset) {
            if *c == '"' {
                self.offset += 1;
                self.char_position.column += 1;
                break;
            }

            if *c == '\n' {
                self.char_position.line += 1;
                self.char_position.column_start();
            } else {
                self.char_position.column += 1;
            }

            literal_value.push(*c);
            self.offset += 1;
        }

        self.add_token(
            lexer::TokenType::StringBasic,
            Some(lexer::TomlValue::String(literal_value)),
            start,
            self.char_position,
        );
    }
}
