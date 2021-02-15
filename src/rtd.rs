use core::borrow::Borrow;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use crate::api::Api;
use crate::handler::Handler;
use crate::listener::Lout;

pub struct TdRecv {}

impl TdRecv {
  pub fn new() -> TdRecv {
    Self {}
  }

  pub fn start(&self, api: Arc<Api>, stop_flag: Arc<Mutex<bool>>, lout: Arc<Lout>, warn_unregister_listener: Arc<bool>) -> JoinHandle<()> {
    thread::spawn(move || {
      let is_stop = stop_flag.lock().unwrap();
      let event_api = api.event_api();
      while !*is_stop {
        if let Some(json) = api.receive(2.0) {
          Handler::new(&event_api, lout.borrow(), warn_unregister_listener.borrow()).handle(&json);
        }
      }
    })

    // thread::spawn(move || futures::executor::block_on(async {
    //   let is_stop = stop_flag.lock().unwrap();
    //   let event_api = api.event_api();
    //   while !*is_stop {
    //     if let Some(json) = api.receive(2.0) {
    //       Handler::new(&event_api, lout.borrow()).handle(&json).await;
    //     }
    //   }
    // }))
  }
}

