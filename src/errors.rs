use std::{error, fmt};
use std::any::Any;
use std::fmt::Debug;

use rtdlib::errors::RTDError;

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
  pub fn custom(message: impl AsRef<str>) -> Self {
    let mut error = Self::new("CUSTOM_ERROR");
    error.set_message(message.as_ref());
    error
  }
}

impl TGError {
  pub fn set_key(&mut self, key: &'static str) -> &mut Self {
    self.key = key;
    self
  }

  pub fn set_message(&mut self, message: impl AsRef<str>) -> &mut Self {
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

impl From<RTDError> for TGError {
  fn from(err: RTDError) -> Self {
    let mut tgerr = Self::new("RTDLIB_ERROR");
    tgerr.set_message(err.to_string());
    tgerr.set_context(Box::new(err));
    tgerr
  }
}

