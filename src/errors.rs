use std::any::Any;

use std::{fmt, error};
use std::fmt::Debug;

pub trait TGDatable: Debug {
  fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct TGError {
  key: &'static str,
  message: Option<String>,
  data: Option<Box<dyn TGDatable>>,
  context: Option<Box<dyn std::error::Error>>
}

pub type TGResult<T> = Result<T, TGError>;


impl TGError {
  pub fn new(key: &'static str) -> Self {
    Self {
      key,
      message: None,
      data: None,
      context: None
    }
  }

  pub fn set_key(&mut self, key: &'static str) -> &mut Self {
    self.key = key;
    self
  }

  pub fn set_message<S: AsRef<str>>(&mut self, message: S) -> &mut Self {
    self.message = Some(message.as_ref().to_string());
    self
  }

  pub fn set_data(&mut self, data: Box<dyn TGDatable>) -> &mut Self {
    self.data = Some(data);
    self
  }

  pub fn set_context(&mut self, context: Box<dyn std::error::Error>) -> &mut Self {
    self.context = Some(context);
    self
  }

  pub fn key(&self) -> &'static str { self.key }
  pub fn message(&self) -> &Option<String> { &self.message }
  pub fn data(&self) -> &Option<Box<dyn TGDatable>> { &self.data }
  pub fn context(&self) -> &Option<Box<dyn std::error::Error>> { &self.context }
}


impl fmt::Display for TGError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[{}]: {}", self.key, self.message.clone().map_or("".to_string(), |v| v))
  }
}

impl error::Error for TGError {
  fn cause(&self) -> Option<&dyn error::Error> {
    None
  }
}

