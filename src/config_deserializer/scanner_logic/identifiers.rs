use super::super::lexer;
use super::scanner;

impl scanner::Scanner {
    /// Lexes a bare key from the current position in `source_chars`.
    pub fn scan_identifier(&mut self, start: usize) {
        while let Some(c) = self.source_chars.get(self.offset) {
            if !self.is_ident_continue(*c) {
                break;
            }
            self.char_position.column += 1;
            self.offset += 1;
        }

        self.add_token(
            lexer::TokenType::Identifier,
            None,
            start,
            self.char_position,
        );
    }

    fn is_ident_continue(&mut self, c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }
}
