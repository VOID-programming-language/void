use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum Content {
    PLUS,
    MINUS,
    STAR,
    SLASH,
    EQ,
    CHAR,
    QUESTION,
    COLON,
    SEMICOLON,
    DOT,
    COMMA,
    NOT_EQ,
    NUMBER,
    VAR,
    FUNC,
    FIG_BR_OP,
    FIG_BR_CL,
    BR_OP,
    BR_CL,
    SQ_BR_OP,
    SQ_BR_CL,
    IDENT(&str),
    TYPE_TO,
    TYPE(&str),
}

#[derive(Debug)]
pub struct Token {
    pub content: Content,
    pub start: usize,
    pub end: usize,
    pub file: Patg
}

impl Token {
    pub fn new(content: Content, start: usize, end: usize, file: Path) -> Self {
        Token {
            content: content,
            start: start,
            end: end,
            file: file
        }
    }
}