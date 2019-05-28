use std::sync::{Arc, Mutex};
use std::thread;

use rtdlib::tdjson;
use rtdlib::types::*;

use crate::rtdkit;

pub struct TdRecv {

}

impl TdRecv {
  pub fn new() -> TdRecv {
    Self { }
  }

  pub fn start(&self, stop_flag: Arc<Mutex<bool>>) {
    thread::spawn(move || {
      let tdlib = tdjson::Tdlib::new();
      let is_stop = stop_flag.lock().unwrap();
      while !*is_stop {
        let recv = tdlib.receive(2.0);
        RecvHandler::new(recv.map(|text| rtdkit::fill_json_struct(text)))
          .handle();
      }
    });
  }
}

struct RecvHandler {
  recv: Option<String>
}

impl RecvHandler {
  fn new(recv: Option<String>) -> Self {
    Self {
      recv
    }
  }

  fn handle(&self) {
    match &self.recv {
      Some(json) => {
        let update: Box<Update> = serde_json::from_str(&json[..]).unwrap();

        let tdupdatetype = TDUpdateType::of(toolkit::text::uppercase_first_char(update.td_name()));
        if None == tdupdatetype {
          return;
        }

        let tdupdatetype = tdupdatetype.unwrap();
        println!("Update type => [{:?}]  Receive data => {}", tdupdatetype, json);
        match tdupdatetype {
          TDUpdateType::UpdateAuthorizationState => {
            let authorization_state: Box<UpdateAuthorizationState> = serde_json::from_str(&json[..]).unwrap();
            println!("authorization_state => {:?}", authorization_state);
          },
          _ => {}
        }

      }
      None => {}
    }
  }
}


