use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct UnexpectedArg(pub String);
impl Display for UnexpectedArg {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Unexpected argument `{}`", self.0)
	}
}
impl Error for UnexpectedArg {}

#[derive(Debug)]
pub struct UnknownFlag(pub String);
impl Display for UnknownFlag {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Unknown flag `{}`", self.0)
	}
}
impl Error for UnknownFlag {}
