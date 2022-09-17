use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum LexToken {
    #[regex(r"[a-zA-Z0-9_]+")]
    Ident,

    #[token("let")]
    Let,

    #[token("=")]
    Eq,

    #[token("(")]
    ParOpen,

    #[token(")")]
    ParClose,

    #[token(r"\")]
    #[token(r"λ")]
    FuncStart,
    
    #[token("->")]
    #[token(".")]
    FuncSep,

    #[token("\n")]
    #[token(";")]
    Sep,

    #[error]
    #[regex("[ \t]+", logos::skip)]
    #[regex("#[^\n]*", logos::skip)]
    Error
}

#[derive(Debug)]
pub struct Token {
    pub(super) kind: LexToken,
    pub(super) value: String,
    pub(super) span: logos::Span
}
