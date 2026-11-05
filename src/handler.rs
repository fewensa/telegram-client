use log::{error, warn};
use rtdlib::types::RObject;

use crate::api::Api;
use crate::errors::TGError;
use crate::listener::lasync::RasyncLout;
use crate::listener::levent::EventLout;
use crate::tip;

pub struct Handler<'a> {
  api: &'a Api,
  event_lout: &'a EventLout,
  rasync_lout: &'a RasyncLout,
  warn_unregister_listener: &'a bool,
}

impl<'a> Handler<'a> {
  pub(crate) fn new(
    api: &'a Api,
    event_lout: &'a EventLout,
    rasync_lout: &'a RasyncLout,
    warn_unregister_listener: &'a bool,
  ) -> Self {
    Self {
      api,
      event_lout,
      rasync_lout,
      warn_unregister_listener,
    }
  }

  pub async fn handle(&self, json: &String) {
    // event listener
    if let Some(ev) = self.event_lout.receive() {
      if let Err(e) = ev((self.api, json)) {
        if let Some(ev) = self.event_lout.exception() {
          ev((self.api, &e));
        }
      }
    }
    // async listener
    if let Some(ev) = self.rasync_lout.receive() {
      if let Err(e) = ev((self.api.clone(), json.clone())).await {
        if let Some(ev) = self.rasync_lout.exception() {
          ev((self.api.clone(), e)).await;
        }
      }
    }

    match crate::response::read_object(json) {
      Ok(t) => {
        // The receive thread already published this object to async callers.
        // Notifying again from the worker deadlocks a caller that is waiting
        // inside one of the listeners below: the response would sit in this
        // same worker's queue until that wait finished.
        match self.event_lout.handle_type(self.api, &t) {
          Ok(true) => return,
          Ok(false) => {
            if *self.warn_unregister_listener {
              warn!("{}", tip::un_register_listener(t.td_name()));
            }
          }
          Err(_err) => {
            if let Some(ev) = self.event_lout.exception() {
              ev((self.api, &TGError::new("EVENT_HANDLER_ERROR")));
            }
          }
        }

        // async listener
        match self.rasync_lout.handle_type(self.api, &t).await {
          Ok(true) => {}
          Ok(false) => {
            if *self.warn_unregister_listener {
              warn!("{}", tip::un_register_listener(t.td_name()));
            }
          }
          Err(_err) => {
            if let Some(ev) = self.rasync_lout.exception() {
              ev((self.api.clone(), TGError::new("EVENT_HANDLER_ERROR"))).await;
            }
          }
        }
      }
      Err(e) => {
        error!("{}", tip::data_fail_with_json(json));
        // eprintln!("{:?}", e);
        error!("{:?}", e);
        // event listener
        if let Some(ev) = self.event_lout.exception() {
          ev((self.api, &TGError::new("DESERIALIZE_JSON_FAIL")));
        }
        // async listener
        if let Some(ev) = self.rasync_lout.exception() {
          ev((self.api.clone(), TGError::new("DESERIALIZE_JSON_FAIL"))).await;
        }
      }
    }
  }
}
