use thiserror::Error;

use crate::token::{Location, Token, TokenType};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct Input {
    pub input: String,
    pub cursor: u32,
}

impl Input {
    pub fn of(input: String) -> Self {
        Self { input, cursor: 0 }
    }

    pub fn move_cursor(&mut self, chars: u32) {
        if self.cursor + chars > self.input.len() as u32 {
            return;
        }

        self.cursor += chars;
    }

    pub fn remaining_length(&self) -> u32 {
        self.input.len() as u32 - self.cursor
    }

    pub fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.cursor as usize)
    }

    pub fn has_remaining_input(&self) -> bool {
        self.cursor < self.input.len() as u32
    }

    pub fn skip_whitespace(&mut self, max_spaces: u32, preserve_single: bool) {
        if preserve_single && self.remaining_length() == 1 && self.peek() == Some(' ') {
            return;
        }

        let mut i = 0;
        while i < max_spaces
            && self.has_remaining_input()
            && self.peek().is_some_and(|c| c.is_whitespace())
        {
            self.read(1);
            i += 1;
        }
    }

    pub fn remaining_input(&self) -> String {
        self.input[self.cursor as usize..].to_string()
    }

    pub fn peek_string_chars(&self, chars: u32) -> String {
        let remaining = self.remaining_input();
        if chars > remaining.len() as u32 {
            return "".to_string();
        }

        remaining[0..chars as usize].to_string()
    }

    pub fn read(&mut self, chars: u32) -> String {
        let read_string = self.peek_string_chars(chars);
        self.move_cursor(chars);
        read_string
    }

    pub fn current_location(&self) -> Location {
        let up_to_cursor = &self.input[..self.cursor as usize];
        let mut line = 1;
        let mut col = 1;

        for c in up_to_cursor.chars() {
            if c == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }

        Location { line, col }
    }
}

#[derive(Debug, Error)]
pub enum LexError {
    #[error("unterminated string at {0}")]
    UnterminatedString(Location),

    #[error("unterminated line at {0}")]
    UnterminatedLine(Location),

    #[error("dead code at {0}")]
    DeadCode(Location),
}

impl LexError {
    pub fn name(&self) -> String {
        match self {
            Self::UnterminatedString(_) => "UnterminatedString".to_string(),
            Self::UnterminatedLine(_) => "UnterminatedLine".to_string(),
            Self::DeadCode(_) => "DeadCode".to_string(),
        }
    }
}

pub struct Lexer {
    pub input: Input,
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: Input) -> Self {
        Lexer {
            input,
            tokens: Vec::new(),
        }
    }

    pub fn add_token(&mut self, token: TokenType, value: String) {
        self.tokens.push(Token {
            ty: token,
            value,
            location: self.input.current_location(),
        })
    }

    fn lex_identifier(&mut self) {
        let mut ident = self.input.read(1);

        while self.input.has_remaining_input() && self.input.peek().unwrap().is_alphanumeric() {
            ident.push_str(&self.input.read(1));
        }

        match &*ident {
            "f" | "fu" | "fun" | "func" | "funct" | "functi" | "functio" | "function" => {
                self.add_token(TokenType::FunctionDeclaration, ident)
            }

            "Int" | "String" | "Char" | "Digit" | "Bool" => {
                self.add_token(TokenType::Primitive, ident)
            }

            "true" | "false" | "maybe" => self.add_token(TokenType::BoolLiteral, ident),

            "return" => self.add_token(TokenType::Return, ident),

            _ => self.add_token(TokenType::Identifier, ident),
        }

        self.input.cursor -= 1;
    }

    pub fn lex(mut self) -> Result<Vec<Token>, LexError> {
        while self.input.has_remaining_input() {
            self.input.skip_whitespace(u32::MAX, true);

            let Some(current) = self.input.peek() else {
                break;
            };

            match current {
                ' ' => {}
                '"' => {
                    let mut str = String::new();

                    self.input.read(1);

                    while self.input.has_remaining_input() && self.input.peek().unwrap() != '"' {
                        str.push_str(&self.input.read(1));
                    }

                    if !self.input.has_remaining_input() {
                        return Err(LexError::UnterminatedString(self.input.current_location()));
                    }

                    self.add_token(TokenType::StringLiteral, str);
                }
                ',' => self.add_token(TokenType::Comma, ",".to_string()),
                '!' => {
                    let mut eol = self.input.read(1);
                    self.input.skip_whitespace(u32::MAX, false);

                    while self.input.has_remaining_input() {
                        let current = self.input.read(1);

                        if current != "!" && current != "\n" {
                            self.input.cursor -= 1; // fix the column exceeding length
                            return Err(LexError::DeadCode(self.input.current_location()));
                        } else {
                            eol.push_str(&current);
                        }
                    }

                    self.add_token(TokenType::Eol, eol);
                    break;
                }
                '?' => {
                    let eol_debug = self.input.read(1);
                    self.input.skip_whitespace(u32::MAX, false);

                    if self.input.has_remaining_input() {
                        let current = self.input.read(1);

                        if current != "\n" {
                            self.input.cursor -= 1; // fix the column exceeding length
                            return Err(LexError::DeadCode(self.input.current_location()));
                        }
                    }

                    self.add_token(TokenType::EolDebug, eol_debug);
                    break;
                }
                '(' => self.add_token(TokenType::Lparen, "(".to_string()),
                ')' => self.add_token(TokenType::Rparen, ")".to_string()),
                '{' => self.add_token(TokenType::Lbrace, "{".to_string()),
                '}' => self.add_token(TokenType::Rbrace, "}".to_string()),
                ';' => self.add_token(TokenType::Not, ";".to_string()),
                '=' => {
                    if self.input.peek().unwrap_or_default() == '>' {
                        self.input.read(1);
                        self.add_token(TokenType::Arrow, "=>".to_string());
                    } else {
                        self.add_token(TokenType::Equals, "=".to_string());
                    }
                }
                _ => {
                    if current.is_numeric() {
                        let mut number = String::new();

                        while self.input.has_remaining_input()
                            && self.input.peek().unwrap().is_numeric()
                        {
                            number.push_str(&self.input.read(1));
                        }

                        self.add_token(TokenType::IntLiteral, number);
                    }

                    if current.is_alphabetic() {
                        self.lex_identifier();
                    }
                }
            }

            self.input.read(1);
        }

        // if we cannot find an eol (debug) marker, error
        if let Some(last_token) = self.tokens.iter().next_back()
            && !(last_token.ty == TokenType::Eol || last_token.ty == TokenType::EolDebug)
        {
            return Err(LexError::UnterminatedLine(self.input.current_location()));
        }

        Ok(self.tokens)
    }
}

pub fn lex(input: Input) -> Result<Vec<Token>, LexError> {
    Lexer::new(input).lex()
}
