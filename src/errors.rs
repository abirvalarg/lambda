use std::fmt::Display;

use ariadne::{sources, Report, ReportKind, Label};

#[derive(Debug)]
pub struct Usage;

impl Display for Usage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Usage: {} <script>", std::env::current_exe().unwrap().to_str().unwrap())
    }
}

impl std::error::Error for Usage {}

#[derive(Debug)]
pub struct CallNotFunction {
    path: String,
    span: logos::Span,
}

impl CallNotFunction {
    pub fn new(path: &str, span: logos::Span) -> Self {
        CallNotFunction {
            path: path.into(),
            span
        }
    }
}

impl Display for CallNotFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = Vec::new();
        let src = sources([(self.path.clone(), std::fs::read_to_string(&self.path).unwrap())]);

        Report::build(ReportKind::Error, self.path.clone(), self.span.start)
            .with_label(Label::new((self.path.clone(), self.span.clone())).with_message("attempt to call non-function value"))
            .finish()
            .write(src, &mut buf)
            .unwrap();

        write!(f, "{}", String::from_utf8(buf).unwrap())
        // write!(f, "attempt to call a non-function value")
    }
}

impl std::error::Error for CallNotFunction {}

#[derive(Debug)]
pub struct NoVar {
    path: String,
    span: logos::Span,
    name: String
}

impl NoVar {
    pub fn new(path: &str, span: logos::Span, name: &str) -> Self {
        NoVar {
            path: path.into(),
            span,
            name: name.into()
        }
    }
}

impl Display for NoVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = Vec::new();
        let src = sources([(self.path.clone(), std::fs::read_to_string(&self.path).unwrap())]);

        Report::build(ReportKind::Error, self.path.clone(), self.span.start)
            .with_label(Label::new((self.path.clone(), self.span.clone())).with_message(format!("can't find variable `{}`", self.name)))
            .finish()
            .write(src, &mut buf)
            .unwrap();

        write!(f, "{}", String::from_utf8(buf).unwrap())
    }
}

impl std::error::Error for NoVar {}

#[derive(Debug)]
pub struct BadNumber;

impl Display for BadNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "attempt to show a bad number")
    }
}

impl std::error::Error for BadNumber {}
