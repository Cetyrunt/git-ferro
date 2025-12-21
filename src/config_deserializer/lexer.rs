#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    // Structural Tokens
    EqualsSign,           // =
    Comment,              // #
    BracketOpened,        // [
    BracketClosed,        // ]
    DoubleBracketsOpened, // [[
    DoubleBracketsClosed, // ]]

    // Key/Value Tokens
    Identifier, // Bare keys, e.g., 'name'

    // Value Tokens
    StringBasic, // "string"
}

///enum with available values for parsing
#[derive(Debug, PartialEq)]
pub enum TomlValue {
    String(String),
}

///struct for keeping track of current offset in source_chars
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CharPosition {
    pub line: usize,
    pub column: usize,
}

impl CharPosition {
    ///Resets column to start of the list
    pub fn column_start(&mut self) {
        self.column = 0
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<TomlValue>,
    pub line_position: CharPosition,
}
