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

  #[doc(hidden)]
  pub fn tdlib(&self) -> &tdjson::Tdlib {
    self.tdlib.borrow()
  }

  pub fn send<Fnc: Function>(&self, fnc: Fnc) {
    let json = fnc.to_json();
    info!(tglog::telegram(), "===> {}", json);
    self.tdlib.send(&json[..]);
  }

  pub fn receive(&self, timeout: f64) -> Option<String> {
    let receive = self.tdlib.receive(timeout);
    if receive.is_some() {
      info!(tglog::telegram(), "<=== {}", receive.clone().unwrap());
    }
    receive
  }

  pub fn execute<Fnc: Function>(&self, fnc: Fnc) -> Option<String> {
    let json = fnc.to_json();
    info!(tglog::telegram(), "===>>> {}", json);
    self.tdlib.execute(&json[..])
  }

  pub fn test(&self) {
    println!("test");
  }
}
