use super::lexer;

#[derive(Debug)]
pub enum Value {
    String(String),
    Array(Vec<Table>),
}

pub type Table = std::collections::HashMap<String, Value>;

pub struct Parser {
    tokens: Vec<lexer::Token>,
    current: usize,
    root: Table,
    current_array: Option<String>, // name of active [[table]]
}

impl Parser {
    ///parses the tokens in the token_stream into key value pairs
    fn parse(&mut self) -> Result<(), String> {
        while !self.is_at_end() {
            match self.peek().token_type {
                lexer::TokenType::Comment => {
                    self.advance();
                }

                lexer::TokenType::Identifier => {
                    self.parse_key_value()?;
                }

                lexer::TokenType::DoubleBracketsOpened => {
                    self.parse_array_table()?;
                }

                _ => return Err("Unexpected token".into()),
            }
        }
        Ok(())
    }

    fn parse_array_table(&mut self) -> Result<(), String> {
        self.consume(lexer::TokenType::DoubleBracketsOpened)?;
        let name: &str = &self.consume_identifier()?;
        self.consume(lexer::TokenType::DoubleBracketsClosed)?;

        let entry = self
            .root
            .entry(name.to_string())
            .or_insert_with(|| Value::Array(Vec::new()));

        match entry {
            Value::Array(arr) => arr.push(Table::new()),
            _ => return Err("Expected array-of-tables".into()),
        }

        self.current_array = Some(name.to_string());
        Ok(())
    }

    fn parse_key_value(&mut self) -> Result<(), String> {
        let key = self.consume_identifier()?;
        self.consume(lexer::TokenType::EqualsSign)?;

        let value = match self.advance().token_type {
            lexer::TokenType::StringBasic => {
                match self.previous().literal.as_ref().ok_or("Missing literal")? {
                    lexer::TomlValue::String(s) => Value::String(s.to_string()),
                }
            }
            _ => return Err("Expected value".into()),
        };

        match &self.current_array {
            Some(array_name) => {
                if let Some(Value::Array(arr)) = self.root.get_mut(array_name) {
                    let table = arr.last_mut().ok_or("No active table")?;
                    table.insert(key, value);
                }
            }
            None => {
                self.root.insert(key, value);
            }
        }

        Ok(())
    }

    fn consume(&mut self, expected: lexer::TokenType) -> Result<(), String> {
        if self.peek().token_type == expected {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Expected {:?}, found {:?}",
                expected,
                self.peek().token_type
            ))
        }
    }

    fn consume_identifier(&mut self) -> Result<String, String> {
        if self.peek().token_type == lexer::TokenType::Identifier {
            let _token_type = self.peek().token_type;
            let name = &self.advance().lexeme;
            Ok(name.to_string())
        } else {
            Err(format!(
                "Expected identifier, found {:?}",
                self.peek().token_type
            ))
        }
    }

    ///Advances to the next token
    fn advance(&mut self) -> &lexer::Token {
        self.current += 1;
        self.previous()
    }

    ///Peeks at the next token
    fn peek(&self) -> &lexer::Token {
        &self.tokens[self.current]
    }

    ///Checks the previous token
    fn previous(&self) -> &lexer::Token {
        &self.tokens[self.current - 1]
    }

    ///Checks if parser is at the end of `token_stream`
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}

impl Parser {
    pub fn run(tokens: Vec<lexer::Token>) -> Result<Table, String> {
        let mut parser = Parser {
            tokens,
            current: 0,
            root: Table::new(),
            current_array: None,
        };

        parser.parse()?;
        Ok(parser.root)
    }
}
