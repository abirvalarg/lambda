use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct NotImplemented;
impl Display for NotImplemented {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Not implemented")
	}
}
impl Error for NotImplemented {}
