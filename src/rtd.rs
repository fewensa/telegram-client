use core::borrow::Borrow;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use crate::api::Api;
use crate::handler::Handler;
use crate::listener::Listener;

pub struct TdRecv {}

impl TdRecv {
  pub fn new() -> TdRecv {
    Self {}
  }

  pub fn start(
    &self,
    api: Arc<Api>,
    stop_flag: Arc<Mutex<bool>>,
    listener: &Listener,
    warn_unregister_listener:
    Arc<bool>,
  ) -> JoinHandle<()> {
    let event_listener = listener.event_listener();
    let rasync_listener = listener.rasync_listener();
    let event_lout = event_listener.lout();
    let rasync_lout = rasync_listener.lout();

    thread::spawn(move || {
      let is_stop = stop_flag.lock().unwrap();

      while !*is_stop {
        if let Some(json) = api.receive(2.0) {
          let api = api.clone();
          let event_lout = event_lout.clone();
          let rasync_lout = rasync_lout.clone();
          let warn_unregister_listener = warn_unregister_listener.clone();
          thread::spawn(move || {
            futures::executor::block_on(async move {
              Handler::new(
                api.as_ref(),
                &event_lout,
                &rasync_lout,
                warn_unregister_listener.borrow(),
              )
                .handle(&json)
                .await;
            });
          });
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

