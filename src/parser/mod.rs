use std::{iter::Peekable, vec::IntoIter, collections::HashSet, fmt::Debug};

use logos::Logos;

use self::token::{Token, LexToken};

mod token;

pub fn parse(src: &str, path: &str) -> Result<Action, Error> {
    let mut tokens = {
        let mut lex = token::LexToken::lexer(src);
        let mut tokens = Vec::with_capacity(lex.size_hint().0);
        while let Some(kind) = lex.next() {
            tokens.push(Token {
                kind,
                value: lex.slice().into(),
                span: lex.span()
            });
        }
        tokens.into_iter().peekable()
    };
    let mut actions = Vec::new();

    while tokens.peek().is_some() {
        if let Some(action) = parse_statement(&mut tokens, path)? {
            actions.push(action);
        }
    }

    Ok(Action {
        path: path.into(),
        span: 0..src.len(),
        kind: ActionKind::Chunk(actions)
    })
}

fn parse_statement(tokens: &mut Peekable<IntoIter<Token>>, path: &str) -> Result<Option<Action>, Error> {
    if tokens.peek().unwrap().kind == LexToken::Let {
        let let_span = tokens.next().unwrap().span;
        if let Some(name) = tokens.next() {
            if name.kind == LexToken::Ident {
                if let Some(eq) = tokens.next() {
                    if eq.kind == LexToken::Eq {
                        let val = parse_expr(tokens, path, false)?.0;
                        Ok(Some(Action {
                            path: path.into(),
                            span: let_span.start..val.span.end,
                            kind: ActionKind::Assign {
                                target: name.value,
                                expr: Box::new(val)
                            }
                        }))
                    } else {
                        Err(Error {
                            path: path.into(),
                            span: name.span,
                            kind: ErrorKind::UnexpectedToken
                        })
                    }
                } else {
                    Err(Error {
                        path: path.into(),
                        span: let_span,
                        kind: ErrorKind::UnexpectedEnd
                    })
                }
            } else {
                Err(Error {
                    path: path.into(),
                    span: name.span,
                    kind: ErrorKind::UnexpectedToken
                })
            }
        } else {
            Err(Error {
                path: path.into(),
                span: let_span,
                kind: ErrorKind::UnexpectedEnd
            })
        }
    } else if tokens.peek().unwrap().kind == LexToken::Sep {
        tokens.next();
        Ok(None)
    } else {
        Ok(Some(parse_expr(tokens, path, false)?.0))
    }
}

fn parse_expr(tokens: &mut Peekable<IntoIter<Token>>, path: &str, nested: bool) -> Result<(Action, HashSet<String>), Error> {
    let mut res: Option<Action> = None;
    let mut captures = HashSet::new();
    let mut need_break = false;
    while let Some(token) = tokens.next() {
        let item = match token.kind {
            LexToken::Ident => {
                let mut caps = HashSet::with_capacity(1);
                caps.insert(token.value.clone());
                (Action {
                    path: path.into(),
                    span: token.span,
                    kind: ActionKind::Ident(token.value)
                }, caps)
            }
            LexToken::ParOpen => parse_expr(tokens, path, true)?,
            LexToken::ParClose if nested => if res.is_some() {
                break;
            } else {
                return Err(Error {
                    path: path.into(),
                    span: token.span,
                    kind: ErrorKind::NoToken
                })
            }
            LexToken::Let
            | LexToken::Eq
            | LexToken::ParClose
            | LexToken::FuncSep => return Err(Error {
                path: path.into(),
                span: token.span,
                kind: ErrorKind::UnexpectedToken
            }),
            LexToken::FuncStart => {
                let item = parse_function(token.span, tokens, path, nested)?;
                let caps = match &item.kind {
                    ActionKind::FuncDef { captures, .. } => captures.clone(),
                    _ => panic!()
                };
                need_break = true;
                (item, caps)
            }
            LexToken::Sep if res.is_none() => continue,
            LexToken::Sep => break,
            LexToken::Error => return Err(Error {
                path: path.into(),
                span: token.span,
                kind: ErrorKind::UnknownToken
            })
        };
        captures.extend(item.1);
        if let Some(func) = res {
            res = Some(Action {
                path: path.into(),
                span: func.span.start..item.0.span.end,
                kind: ActionKind::Call {
                    func: Box::new(func),
                    arg: Box::new(item.0)
                }
            });
        } else {
            res = Some(item.0);
        }
        if need_break {
            break;
        }
    }
    Ok((res.unwrap(), captures))
}

fn parse_function(start_span: logos::Span, tokens: &mut Peekable<IntoIter<Token>>, path: &str, nested: bool) -> Result<Action, Error> {
    if let Some(arg) = tokens.next() {
        if let Some(sep) = tokens.next() {
            if sep.kind == LexToken::FuncSep {
                let (expr, mut caps) = parse_expr(tokens, path, nested)?;
                caps.remove(&arg.value);
                Ok(Action {
                    path: path.into(),
                    span: start_span.start..expr.span.end,
                    kind: ActionKind::FuncDef {
                        arg: arg.value,
                        expr: Box::new(expr),
                        captures: caps
                    }
                })
            } else {
                Err(Error {
                    path: path.into(),
                    span: start_span,
                    kind: ErrorKind::UnexpectedToken
                })
            }
        } else {
            Err(Error {
                path: path.into(),
                span: start_span,
                kind: ErrorKind::UnexpectedEnd
            })
        }
    } else {
        Err(Error {
            path: path.into(),
            span: start_span,
            kind: ErrorKind::UnexpectedEnd
        })
    }
}

#[derive(Clone)]
pub struct Action {
    path: String,
    span: logos::Span,
    kind: ActionKind
}

impl Action {
    pub fn kind(&self) -> &ActionKind {
        &self.kind
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn span(&self) -> logos::Span {
        self.span.clone()
    }
}

impl Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.kind)
    }
}

#[derive(Debug, Clone)]
pub enum ActionKind {
    Chunk(Vec<Action>),
    Ident(String),
    Assign {
        target: String,
        expr: Box<Action>
    },
    Call {
        func: Box<Action>,
        arg: Box<Action>
    },
    FuncDef {
        arg: String,
        expr: Box<Action>,
        captures: HashSet<String>
    }
}

#[derive(Debug)]
pub struct Error {
    path: String,
    span: logos::Span,
    kind: ErrorKind
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = Vec::new();
        let src = ariadne::sources([(self.path.clone(), std::fs::read_to_string(&self.path).unwrap())]);

        ariadne::Report::build(ariadne::ReportKind::Error, self.path.clone(), self.span.start)
            .with_label(ariadne::Label::new((self.path.clone(), self.span.clone())).with_message(self.kind.to_string()))
            .finish()
            .write(src, &mut buf)
            .unwrap();
        
        write!(f, "{}", String::from_utf8(buf).unwrap())
    }
}

impl std::error::Error for Error {}

#[derive(Copy, Clone, Debug)]
pub enum ErrorKind {
    UnexpectedEnd,
    UnexpectedToken,
    UnknownToken,
    NoToken
}

impl ToString for ErrorKind {
    fn to_string(&self) -> String {
        use ErrorKind::*;
        match self {
            UnexpectedEnd => "Unexpected end of file".into(),
            UnexpectedToken => "Unexpected token".into(),
            UnknownToken => "Unknown token".into(),
            NoToken => "A token expected".into()
        }
    }
}
