use core::borrow::Borrow;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::api::Api;
use crate::handler::handler::Handler;
use crate::listener::Lout;

pub struct TdRecv {}

impl TdRecv {
  pub fn new() -> TdRecv {
    Self {}
  }

  pub fn start(&self, api: Arc<Api>, stop_flag: Arc<Mutex<bool>>, lout: Arc<Lout>) {
    thread::spawn(move || {
      let is_stop = stop_flag.lock().unwrap();
      while !*is_stop {
        let recv = api.receive(2.0);
        // recv.map(|text| tdkit::fill_json_struct(text))
        if recv.is_none() {
          continue;
        }
        let json = recv.unwrap();
        Handler::new(api.borrow(), lout.borrow())
          .handle(&json);
      }
    });
  }
}

