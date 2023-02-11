use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct UError {
    pub message: String,
    pub code: u16,
}

impl UError {
    pub fn new(msg: &str, code: u16) -> Self {
        Self {
            message: msg.to_owned(),
            code,
        }
    }
}

impl fmt::Display for UError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}, Code: {}", self.message, self.code)
    }
}
