use crate::error::NotImplemented;
pub mod unsigned;

pub trait Type {
    fn get_name(&self) -> &'static str;
}

pub trait FromValue<T: ?Sized> {
    fn from_value(value: Box<T>) -> Result<Box<Self>, Box<dyn std::error::Error>>;
}

pub trait Value {
    fn call(&self, _arg: Box<dyn Value>) -> Result<Box<dyn Value>, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplemented))
    }
}
