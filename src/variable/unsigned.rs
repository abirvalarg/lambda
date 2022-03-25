use super::{Type, FromValue};

impl Type for u32 {
    fn get_name(&self) -> &'static str {
        "unsigned"
    }
}

impl FromValue<u32> for u32 {
    fn from_value(v: Box<u32>) -> Result<Box<u32>, Box<dyn std::error::Error>> {
        Ok(v)
    }
}

impl FromValue<str> for u32 {
    fn from_value(value: Box<str>) -> Result<Box<Self>, Box<dyn std::error::Error>> {
        Ok(Box::new(value.parse()?))
    }
}
