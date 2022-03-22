use std::error::Error;
use std::fmt::{self, Formatter, Display};

#[derive(Debug)]
pub struct NotImplemented;
impl Display for NotImplemented {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "not implemented")
    }
}
impl Error for NotImplemented {}
