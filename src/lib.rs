use std::collections::HashMap;
pub mod variable;
pub mod error;
pub mod parser;

pub struct Lambda {
}

impl Lambda {
	pub fn new() -> Self {
		Lambda {
		}
	}

	pub fn init(&mut self) {
	}
}

impl Default for Lambda {
	fn default() -> Self {
		let mut vm = Lambda::new();
		vm.init();
		vm
	}
}
