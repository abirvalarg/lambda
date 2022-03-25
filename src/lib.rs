#![feature(trait_alias)]

pub mod variable;
pub mod error;

pub trait InputFunction<'a> = FnMut(&'static str, Option<Box<dyn variable::Value + 'a>>) -> Result<Box<dyn variable::Value + 'a>, Box<dyn std::error::Error>>;

pub struct Lambda<'a> {
	input: Option<Box<dyn InputFunction<'a> + 'a>>
}

impl<'a> Lambda<'a> {
	pub fn new() -> Self {
		Lambda {
			input: None
		}
	}

	pub fn init(&mut self) {
	}

	pub fn set_input(&mut self, func: impl InputFunction<'a> + 'a) {
		self.input = Some(Box::new(func));
	}
}

impl<'a> Default for Lambda<'a> {
	fn default() -> Self {
		let mut vm = Lambda::new();
		vm.init();
		vm
	}
}
