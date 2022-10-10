use std::error::Error;

pub type Result<U> = core::result::Result<U, Box<dyn Error>>;
