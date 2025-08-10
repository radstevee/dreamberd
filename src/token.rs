use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    FunctionDeclaration,
    Identifier,
    Lparen,
    Rparen,
    Arrow,
    Lbrace,
    Rbrace,
    StringLiteral,
    Eol,
    EolDebug,
    IntLiteral,
    Primitive,
    Comma,
    BoolLiteral,
    Return,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    pub line: u32,
    pub col: u32,
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(line: {}, col: {})", self.line, self.col)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub ty: TokenType,
    pub value: String,
    pub location: Location,
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.ty {
            TokenType::FunctionDeclaration => write!(f, "FunctionDeclaration"),
            TokenType::Identifier => write!(f, "Identifier({})", self.value.clone()),
            TokenType::Lparen => write!(f, "Lparen"),
            TokenType::Rparen => write!(f, "Rparen"),
            TokenType::Arrow => write!(f, "Arrow"),
            TokenType::Lbrace => write!(f, "Lbrace"),
            TokenType::Rbrace => write!(f, "Rbrace"),
            TokenType::StringLiteral => write!(f, "StringLiteral({:?})", self.value.clone()),
            TokenType::Eol => write!(f, "Eol"),
            TokenType::EolDebug => write!(f, "EolDebug"),
            TokenType::IntLiteral => write!(f, "IntLiteral({})", self.value.clone()),
            TokenType::Primitive => write!(f, "Primitive({})", self.value.clone()),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::BoolLiteral => write!(f, "BoolLiteral({})", self.value.clone()),
            TokenType::Return => write!(f, "Return"),
            TokenType::Not => write!(f, "Not"),
        }
    }
}
