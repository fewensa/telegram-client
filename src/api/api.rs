use core::borrow::Borrow;
use std::sync::Arc;

use rtdlib::tdjson;
use rtdlib::types::Function;
use crate::tglog;

#[derive(Debug, Clone)]
pub struct Api {
  tdlib: Arc<tdjson::Tdlib>
}

impl Default for Api {
  fn default() -> Self {
    Api::new(tdjson::Tdlib::new())
  }
}

impl Api {
  pub fn new(tdlib: tdjson::Tdlib) -> Self {
    Self { tdlib: Arc::new(tdlib) }
  }

  pub fn tdlib(&self) -> &tdjson::Tdlib {
    self.tdlib.borrow()
  }

  pub fn send<Fnc: Function>(&self, fnc: Fnc) {
    let json = fnc.to_json();
    info!(tglog::telegram(), "===> {}", json);
    self.tdlib.send(&json[..]);
  }

}
