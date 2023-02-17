use std::fmt;

#[derive(Debug)]
pub enum Code {
  Internal,
  NotFound,
}

impl fmt::Display for Code {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Code::Internal => write!(f, "Internal"),
      Code::NotFound => write!(f, "NotFound"),
    }
  }
}

#[derive(Debug)]
pub struct Error {
  code: Code,
  message: String,
}

impl Error {
  pub fn new(code: Code, msg: &str) -> Error {
    Error {
      message: msg.to_owned(),
      code,
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "code: {}, message: {}", self.code, self.message)
  }
}

#[derive(Debug)]
pub enum DBError {
  Internal(String),
  NotFound(String),
}
