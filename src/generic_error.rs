use std::{error::Error, fmt::Debug, fmt::Display};

#[derive(Debug)]
pub struct GenericError<T: Display> {
    pub message: T,
}

impl<T: Debug + Display + 'static> GenericError<T> {
    pub fn from<U>(message: T) -> Result<U, Box<dyn Error>> {
        Err(Box::new(GenericError::new(message)))
    }
}

impl<T: Display> GenericError<T> {
    pub fn new(message: T) -> Self {
        Self { message }
    }
}

impl<T: Display> Display for GenericError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

impl<T: Debug + Display> Error for GenericError<T> {}
