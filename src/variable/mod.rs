use crate::error;

pub trait Type {
	fn name(&self) -> &'static str;
}

pub trait Value {
	fn call(&self) -> Box<dyn Value>;
}

pub enum InputMode {
	UserInput,
	API
}
